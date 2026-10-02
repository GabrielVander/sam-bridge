use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use authentication::application::gateways::{
    AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway,
};
use authentication::domain::entities::{Credential, Email, Password};
use sam::authentication::adapters::gateways::AuthorizationGatewaySamImpl;
use sam::client::{CacheTtl, Clock, SamClient, SamClientCacheDecorator, SamClientImpl};
use sam::lessons::adapters::gateways::{
    MusicianProfileGatewaySamImpl, StudentLessonsGatewaySamImpl,
};
use sam::roster::adapters::gateways::StudentGatewaySamImpl;
use student::application::gateways::{
    MusicianProfileGateway, StudentGateway, StudentGatewayError, StudentLessonsGateway,
    StudentLessonsGatewayError,
};
use student::domain::entities::StudentId;
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

#[path = "support/helpers.rs"]
mod support;
use support::sam_operations_for;

#[test]
fn the_student_and_musician_profile_gateways_share_a_single_listing_fetch() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);

        sam.listed_ids().expect("listing should succeed");
        sam.profiles
            .get_by_id(&StudentId::new("1".to_owned()))
            .expect("profile should be found");
        sam.listed_ids().expect("listing should succeed");

        assert_eq!(sam.requests_to("/alunos/listagem").await, 1);
        assert_eq!(sam.requests_to("/painel").await, 1);
    });
}

#[test]
fn students_are_served_from_the_cache_within_their_ttl() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);

        sam.listed_ids().expect("listing should succeed");
        sam.site.lists(&["1", "2"]);
        sam.clock
            .advance(STUDENTS_TTL.saturating_sub(Duration::from_secs(1)));

        assert_eq!(sam.listed_ids(), Ok(ids(&["1"])));
    });
}

#[test]
fn students_are_fetched_again_once_their_ttl_elapses() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);

        sam.listed_ids().expect("listing should succeed");
        sam.site.lists(&["1", "2"]);
        sam.clock.advance(STUDENTS_TTL);

        assert_eq!(sam.listed_ids(), Ok(ids(&["1", "2"])));
    });
}

#[test]
fn a_failed_students_fetch_is_not_cached() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);
        sam.site.goes_down();

        assert!(sam.listed_ids().is_err());
        sam.site.recovers();

        assert_eq!(sam.listed_ids(), Ok(ids(&["1"])));
    });
}

#[test]
fn a_failed_refresh_reports_the_failure_instead_of_stale_students() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);

        sam.listed_ids().expect("listing should succeed");
        sam.site.goes_down();
        sam.clock.advance(STUDENTS_TTL);

        assert!(sam.listed_ids().is_err());
    });
}

#[test]
fn lessons_are_cached_per_student() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.records_lesson("A", "a1");
        sam.site.records_lesson("B", "b1");

        sam.lesson_ids("A").expect("lessons should be retrieved");
        sam.lesson_ids("B").expect("lessons should be retrieved");
        sam.site.records_lesson("A", "a2");
        sam.site.records_lesson("B", "b2");

        assert_eq!(sam.lesson_ids("A"), Ok(ids(&["a1"])));
        assert_eq!(sam.lesson_ids("B"), Ok(ids(&["b1"])));
    });
}

#[test]
fn lessons_are_fetched_again_once_their_ttl_elapses() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.records_lesson("A", "a1");

        sam.lesson_ids("A").expect("lessons should be retrieved");
        sam.site.records_lesson("A", "a2");
        sam.clock.advance(LESSONS_TTL);

        assert_eq!(sam.lesson_ids("A"), Ok(ids(&["a2"])));
    });
}

#[test]
fn a_failed_lessons_fetch_is_not_cached() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.records_lesson("A", "a1");
        sam.site.goes_down();

        assert!(sam.lesson_ids("A").is_err());
        sam.site.recovers();

        assert_eq!(sam.lesson_ids("A"), Ok(ids(&["a1"])));
    });
}

#[test]
fn students_and_lessons_expire_independently() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);
        sam.site.records_lesson("A", "a1");

        sam.listed_ids().expect("listing should succeed");
        sam.lesson_ids("A").expect("lessons should be retrieved");
        sam.site.lists(&["1", "2"]);
        sam.site.records_lesson("A", "a2");
        sam.clock.advance(LESSONS_TTL);

        assert_eq!(sam.lesson_ids("A"), Ok(ids(&["a2"])));
        assert_eq!(sam.listed_ids(), Ok(ids(&["1"])));
    });
}

#[test]
fn every_authorization_asks_the_site() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");

        assert_eq!(sam.log_in(), Ok(AuthorizationResult::Authorized));
        sam.site.rejects_logins();

        assert_eq!(sam.log_in(), Ok(AuthorizationResult::Unauthorized));
    });
}

#[test]
fn an_authorization_failure_reaches_the_caller() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.goes_down();

        assert!(sam.log_in().is_err());
    });
}

#[test]
fn logging_in_discards_everything_cached_for_the_previous_session() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);
        sam.site.records_lesson("A", "a1");

        sam.listed_ids().expect("listing should succeed");
        sam.lesson_ids("A").expect("lessons should be retrieved");
        sam.site.lists(&["2"]);
        sam.site.records_lesson("A", "a2");
        sam.log_in().expect("authorization should succeed");

        assert_eq!(sam.listed_ids(), Ok(ids(&["2"])));
        assert_eq!(sam.lesson_ids("A"), Ok(ids(&["a2"])));
    });
}

#[test]
fn a_rejected_login_also_discards_the_cache() {
    smol::block_on(async {
        let sam: Sam = Sam::start().await.expect("SAM should start");
        sam.site.lists(&["1"]);

        sam.listed_ids().expect("listing should succeed");
        sam.site.lists(&["2"]);
        sam.site.rejects_logins();
        sam.log_in().expect("authorization should be answered");

        assert_eq!(sam.listed_ids(), Ok(ids(&["2"])));
    });
}

struct Sam {
    server: MockServer,
    site: FakeSite,
    clock: Arc<FakeClock>,
    students: StudentGatewaySamImpl,
    profiles: MusicianProfileGatewaySamImpl,
    lessons: StudentLessonsGatewaySamImpl,
    authorization: AuthorizationGatewaySamImpl,
}

impl Sam {
    async fn start() -> Result<Self, reqwest::Error> {
        let server: MockServer = MockServer::start().await;
        let site: FakeSite = FakeSite::default();
        Mock::given(any())
            .respond_with(site.clone())
            .mount(&server)
            .await;

        let clock: Arc<FakeClock> = Arc::new(FakeClock::new());
        let client: Arc<dyn SamClient> = Arc::new(SamClientCacheDecorator::new(
            Arc::new(SamClientImpl::new(sam_operations_for(&server.uri())?)),
            clock.clone(),
            CacheTtl {
                students: STUDENTS_TTL,
                lessons: LESSONS_TTL,
            },
        ));

        Ok(Self {
            server,
            site,
            clock,
            students: StudentGatewaySamImpl::new(client.clone()),
            profiles: MusicianProfileGatewaySamImpl::new(client.clone()),
            lessons: StudentLessonsGatewaySamImpl::new(client.clone()),
            authorization: AuthorizationGatewaySamImpl::new(client),
        })
    }

    fn listed_ids(&self) -> Result<Vec<String>, StudentGatewayError> {
        Ok(self
            .students
            .get_available_records()?
            .into_iter()
            .map(|student| student.id.as_str().to_owned())
            .collect())
    }

    fn lesson_ids(&self, student_id: &str) -> Result<Vec<String>, StudentLessonsGatewayError> {
        Ok(self
            .lessons
            .get_all_for_student_with_id(&StudentId::new(student_id.to_owned()))?
            .msa
            .into_iter()
            .filter_map(|lesson| lesson.id)
            .collect())
    }

    fn log_in(&self) -> Result<AuthorizationResult, AuthorizationError> {
        self.authorization.authorize(&Credential::new(
            Email::new("someone@example.com".to_owned()),
            Password::new("secret".to_owned()),
        ))
    }

    async fn requests_to(&self, request_path: &str) -> usize {
        self.server
            .received_requests()
            .await
            .unwrap_or_default()
            .iter()
            .filter(|request| request.url.path() == request_path)
            .count()
    }
}

#[derive(Clone, Default)]
struct FakeSite {
    state: Arc<Mutex<SiteState>>,
}

impl FakeSite {
    fn lists(&self, student_ids: &[&str]) {
        self.state().listed = ids(student_ids);
    }

    fn records_lesson(&self, student_id: &str, lesson_id: &str) {
        self.state()
            .latest_lesson
            .insert(student_id.to_owned(), lesson_id.to_owned());
    }

    fn goes_down(&self) {
        self.state().down = true;
    }

    fn recovers(&self) {
        self.state().down = false;
    }

    fn rejects_logins(&self) {
        self.state().rejects_logins = true;
    }

    fn state(&self) -> MutexGuard<'_, SiteState> {
        locked(&self.state)
    }
}

impl Respond for FakeSite {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        site_response(&self.state(), request.url.path())
    }
}

#[derive(Default)]
struct SiteState {
    listed: Vec<String>,
    latest_lesson: HashMap<String, String>,
    down: bool,
    rejects_logins: bool,
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|&value| value.to_owned()).collect()
}

fn locked<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn site_response(state: &SiteState, request_path: &str) -> ResponseTemplate {
    if request_path == "/painel" {
        return ResponseTemplate::new(200);
    }
    if state.down {
        return ResponseTemplate::new(500);
    }
    if request_path == "/autenticar" {
        return login_response(state);
    }
    if request_path == "/alunos/listagem" {
        return listing_response(&state.listed);
    }

    request_path.strip_prefix("/licoes/index/").map_or_else(
        || ResponseTemplate::new(404),
        |id| lessons_response(state.latest_lesson.get(id)),
    )
}

fn login_response(state: &SiteState) -> ResponseTemplate {
    if state.rejects_logins {
        return ResponseTemplate::new(200)
            .set_body_string("<p>* Oops... O usuário ou senha incorretos!</p>");
    }

    ResponseTemplate::new(303)
}

fn listing_response(student_ids: &[String]) -> ResponseTemplate {
    let rows: Vec<String> = student_ids
        .iter()
        .map(|id| {
            format!(r#"["{id}","FULANO DE TAL","SOMEWHERE","MÚSICO","VIOLINO","RJM","1","0"]"#)
        })
        .collect();

    ResponseTemplate::new(200)
        .set_body_string(format!(
            r#"{{"draw":"1","recordsTotal":{count},"recordsFiltered":{count},"data":[{rows}]}}"#,
            count = rows.len(),
            rows = rows.join(","),
        ))
        .insert_header("Content-Type", "application/json")
}

fn lessons_response(lesson_id: Option<&String>) -> ResponseTemplate {
    let rows: String = lesson_id.map_or_else(String::new, |id| {
        format!(r#"<tr id="msa_{id}"><td>01/01/2024</td></tr>"#)
    });

    ResponseTemplate::new(200)
        .set_body_string(format!(
            r#"<html><body><div id="msa"><table><tbody>{rows}</tbody></table></div></body></html>"#
        ))
        .insert_header("Content-Type", "text/html")
}

struct FakeClock {
    origin: Instant,
    elapsed: Mutex<Duration>,
}

impl FakeClock {
    fn new() -> Self {
        Self {
            origin: Instant::now(),
            elapsed: Mutex::new(Duration::ZERO),
        }
    }

    fn advance(&self, by: Duration) {
        let mut elapsed: MutexGuard<'_, Duration> = locked(&self.elapsed);
        *elapsed = elapsed.saturating_add(by);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        let elapsed: Duration = *locked(&self.elapsed);
        self.origin.checked_add(elapsed).unwrap_or(self.origin)
    }
}

const STUDENTS_TTL: Duration = Duration::from_secs(300);
const LESSONS_TTL: Duration = Duration::from_secs(60);
