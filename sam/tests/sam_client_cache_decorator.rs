use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use sam::client::{
    CacheTtl, Clock, MsaLesson, SamClient, SamClientCacheDecorator, SamClientError, SamCredentials,
    SamStudent, StudentLessonsPage,
};

#[test]
fn students_are_served_from_the_cache_within_the_ttl() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    let first = client.students().expect("first call succeeds");
    clock.advance(STUDENTS_TTL.saturating_sub(Duration::from_secs(1)));
    let second = client.students().expect("second call succeeds");

    assert_eq!(first, vec![student("1")]);
    assert_eq!(second, first);
    assert_eq!(FakeSamClient::calls(&inner.students_calls), 1);
}

#[test]
fn students_are_fetched_again_once_the_ttl_elapses() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.students().expect("first call succeeds");
    inner.replace_students(vec![student("1"), student("2")]);
    clock.advance(STUDENTS_TTL);
    let refreshed = client.students().expect("second call succeeds");

    assert_eq!(refreshed, vec![student("1"), student("2")]);
    assert_eq!(FakeSamClient::calls(&inner.students_calls), 2);
}

#[test]
fn a_failed_students_fetch_is_not_cached() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    inner.set_failing(true);
    assert!(client.students().is_err());

    inner.set_failing(false);
    let recovered = client.students().expect("retry succeeds");

    assert_eq!(recovered, vec![student("1")]);
    assert_eq!(FakeSamClient::calls(&inner.students_calls), 2);
}

#[test]
fn a_failed_refresh_surfaces_the_error_instead_of_stale_data() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.students().expect("first call succeeds");
    clock.advance(STUDENTS_TTL);
    inner.set_failing(true);

    assert!(matches!(
        client.students(),
        Err(SamClientError::SessionExpired)
    ));
}

#[test]
fn lessons_are_cached_per_student() {
    let inner = Arc::new(FakeSamClient::default());
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    let first_of_one = client.student_lessons("1").expect("succeeds");
    let first_of_two = client.student_lessons("2").expect("succeeds");
    let second_of_one = client.student_lessons("1").expect("succeeds");
    let second_of_two = client.student_lessons("2").expect("succeeds");

    assert_eq!(first_of_one, page_for("1"));
    assert_eq!(first_of_two, page_for("2"));
    assert_eq!(second_of_one, first_of_one);
    assert_eq!(second_of_two, first_of_two);
    assert_eq!(FakeSamClient::calls(&inner.lessons_calls), 2);
}

#[test]
fn lessons_are_fetched_again_once_their_ttl_elapses() {
    let inner = Arc::new(FakeSamClient::default());
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.student_lessons("1").expect("succeeds");
    clock.advance(LESSONS_TTL);
    client.student_lessons("1").expect("succeeds");

    assert_eq!(FakeSamClient::calls(&inner.lessons_calls), 2);
}

#[test]
fn a_failed_lessons_fetch_is_not_cached() {
    let inner = Arc::new(FakeSamClient::default());
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    inner.set_failing(true);
    assert!(client.student_lessons("1").is_err());

    inner.set_failing(false);
    let recovered = client.student_lessons("1").expect("retry succeeds");

    assert_eq!(recovered, page_for("1"));
    assert_eq!(FakeSamClient::calls(&inner.lessons_calls), 2);
}

#[test]
fn students_and_lessons_expire_independently() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.students().expect("succeeds");
    client.student_lessons("1").expect("succeeds");
    clock.advance(LESSONS_TTL);
    client.students().expect("succeeds");
    client.student_lessons("1").expect("succeeds");

    assert_eq!(FakeSamClient::calls(&inner.students_calls), 1);
    assert_eq!(FakeSamClient::calls(&inner.lessons_calls), 2);
}

#[test]
fn login_is_delegated_and_never_cached() {
    let inner = Arc::new(FakeSamClient::default());
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.login(&credentials()).expect("first login succeeds");
    client.login(&credentials()).expect("second login succeeds");

    assert_eq!(FakeSamClient::calls(&inner.login_calls), 2);
}

#[test]
fn login_errors_are_propagated() {
    let inner = Arc::new(FakeSamClient::default());
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    inner.set_failing(true);

    assert!(matches!(
        client.login(&credentials()),
        Err(SamClientError::SessionExpired)
    ));
}

#[test]
fn logging_in_discards_everything_cached_for_the_previous_session() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.students().expect("succeeds");
    client.student_lessons("1").expect("succeeds");
    client.login(&credentials()).expect("login succeeds");
    client.students().expect("succeeds");
    client.student_lessons("1").expect("succeeds");

    assert_eq!(FakeSamClient::calls(&inner.students_calls), 2);
    assert_eq!(FakeSamClient::calls(&inner.lessons_calls), 2);
}

#[test]
fn a_failed_login_also_discards_the_cache() {
    let inner = Arc::new(FakeSamClient::with_students(vec![student("1")]));
    let clock = Arc::new(FakeClock::new());
    let client = build(&inner, &clock);

    client.students().expect("succeeds");
    inner.set_failing(true);
    assert!(client.login(&credentials()).is_err());
    inner.set_failing(false);
    client.students().expect("succeeds");

    assert_eq!(FakeSamClient::calls(&inner.students_calls), 2);
}

const STUDENTS_TTL: Duration = Duration::from_secs(300);
const LESSONS_TTL: Duration = Duration::from_secs(60);

fn locked<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
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
        let mut elapsed = locked(&self.elapsed);
        *elapsed = elapsed.saturating_add(by);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        let elapsed: Duration = *locked(&self.elapsed);
        self.origin.checked_add(elapsed).unwrap_or(self.origin)
    }
}

#[derive(Default)]
struct FakeSamClient {
    login_calls: AtomicUsize,
    students_calls: AtomicUsize,
    lessons_calls: AtomicUsize,
    failing: AtomicBool,
    students: Mutex<Vec<SamStudent>>,
}

impl FakeSamClient {
    fn with_students(students: Vec<SamStudent>) -> Self {
        Self {
            students: Mutex::new(students),
            ..Self::default()
        }
    }

    fn replace_students(&self, students: Vec<SamStudent>) {
        *locked(&self.students) = students;
    }

    fn set_failing(&self, failing: bool) {
        self.failing.store(failing, Ordering::SeqCst);
    }

    fn calls(counter: &AtomicUsize) -> usize {
        counter.load(Ordering::SeqCst)
    }

    fn outcome(&self) -> Result<(), SamClientError> {
        if self.failing.load(Ordering::SeqCst) {
            Err(SamClientError::SessionExpired)
        } else {
            Ok(())
        }
    }
}

impl SamClient for FakeSamClient {
    fn login(&self, _: &SamCredentials) -> Result<(), SamClientError> {
        self.login_calls.fetch_add(1, Ordering::SeqCst);
        self.outcome()
    }

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
        self.students_calls.fetch_add(1, Ordering::SeqCst);
        self.outcome()?;
        Ok(locked(&self.students).clone())
    }

    fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
        self.lessons_calls.fetch_add(1, Ordering::SeqCst);
        self.outcome()?;
        Ok(page_for(student_id))
    }
}

fn student(id: &str) -> SamStudent {
    SamStudent {
        id: id.to_owned(),
        name: format!("STUDENT {id}"),
        location: "SOMEWHERE".to_owned(),
        role: "MÚSICO".to_owned(),
        instrument: "VIOLINO".to_owned(),
        level: "RJM".to_owned(),
    }
}

fn page_for(student_id: &str) -> StudentLessonsPage {
    StudentLessonsPage {
        msa: vec![MsaLesson {
            id: Some(student_id.to_owned()),
            ..MsaLesson::default()
        }],
        method: Vec::new(),
    }
}

fn credentials() -> SamCredentials {
    SamCredentials {
        login: "someone@example.com".to_owned(),
        password: "secret".to_owned(),
    }
}

fn build(inner: &Arc<FakeSamClient>, clock: &Arc<FakeClock>) -> SamClientCacheDecorator {
    SamClientCacheDecorator::new(
        inner.clone(),
        clock.clone(),
        CacheTtl {
            students: STUDENTS_TTL,
            lessons: LESSONS_TTL,
        },
    )
}
