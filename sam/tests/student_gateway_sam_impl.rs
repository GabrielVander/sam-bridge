use std::sync::Arc;

use sam::client::{
    SamClient, SamClientError, SamClientImpl, SamCredentials, SamStudent, StudentLessonsPage,
};
use sam::http::SamOperations;
use sam::roster::adapters::gateways::StudentGatewaySamImpl;
use student::application::gateways::{FailureKind, StudentGateway, StudentGatewayError};
use student::domain::entities::{
    Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod support;
use support::{FakeSamClient, sam_operations_for, sam_student};

#[test]
fn given_accessible_dashboard_students_should_be_retrieved_and_mapped() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(
                        r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[["99998","PEDRO ÁLVARES CABRAL","JARDIM PALMARES DO NORTE <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","VIOLINO","CANDIDATO(A)","99998","0"]]}"#,
                    )
                    .insert_header("Content-Type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: Result<Vec<Student>, StudentGatewayError> = gateway.get_available_records();

        let students: Vec<Student> = result.expect("students retrieval should succeed");

        assert_eq!(students.len(), 1);
        assert_eq!(students[0].id.as_str(), "99998");
        assert_eq!(students[0].name, "PEDRO ÁLVARES CABRAL");
        assert_eq!(
            students[0].location,
            "JARDIM PALMARES DO NORTE | BR-SP-ARARAQUARA-SÃO CARLOS"
        );
        assert_eq!(
            students[0].position,
            StudentPosition::Musician {
                level: MusicianLevel::Candidate,
                instrument: Some(Instrument::Violin),
                instrument_name: Some("VIOLINO".to_owned()),
            }
        );

        let received_requests: Vec<wiremock::Request> = mock_server
            .received_requests()
            .await
            .expect("requests should have been recorded");
        let dashboard_index: usize = received_requests
            .iter()
            .position(|r| r.url.path() == "/painel")
            .expect("dashboard should have been visited");
        let listing_index: usize = received_requests
            .iter()
            .position(|r| r.url.path() == "/alunos/listagem")
            .expect("listing should have been requested");

        assert!(
            dashboard_index < listing_index,
            "dashboard should be visited before the students listing"
        );
    });
}

#[test]
fn given_musicians_without_a_real_instrument_no_instrument_name_is_carried() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(
                        r#"{"draw":"1","recordsTotal":3,"recordsFiltered":3,"data":[["1","A","SOMEWHERE","MÚSICO","A DEFINIR","CANDIDATO(A)","1","0"],["2","B","SOMEWHERE","MÚSICO","","CANDIDATO(A)","2","0"],["3","C","SOMEWHERE","ORGANISTA","VIOLINO","RJM","3","0"]]}"#,
                    )
                    .insert_header("Content-Type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let students: Vec<Student> = gateway
            .get_available_records()
            .expect("students retrieval should succeed");

        let names: Vec<Option<&str>> = students
            .iter()
            .map(|student| match &student.position {
                StudentPosition::Musician {
                    instrument_name, ..
                } => instrument_name.as_deref(),
                _ => None,
            })
            .collect();

        assert_eq!(names, vec![None, None, None]);
    });
}

#[test]
fn given_inaccessible_dashboard_students_retrieval_should_fail() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(307))
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) = failure_of(gateway.get_available_records())
            .expect("students retrieval should have failed");

        assert_eq!(kind, FailureKind::SessionExpired);
        assert!(details.contains("Session expired"), "got: {details}");
    });
}

#[test]
fn given_unexpected_listing_status_students_retrieval_should_fail() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) = failure_of(gateway.get_available_records())
            .expect("students retrieval should have failed");

        assert_eq!(kind, FailureKind::Unexpected);
        assert!(details.contains("500"), "got: {details}");
    });
}

#[test]
fn given_a_listing_that_is_not_json_the_details_explain_what_could_not_be_decoded() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<html>maintenance</html>"))
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) = failure_of(gateway.get_available_records())
            .expect("students retrieval should have failed");

        assert_eq!(kind, FailureKind::Unexpected);
        assert!(
            details.contains("Unable to decode student listing JSON response"),
            "got: {details}"
        );
        assert!(
            details.contains("expected value"),
            "the underlying parse error should be kept, got: {details}"
        );
    });
}

#[test]
fn given_an_unreachable_site_the_failure_is_a_network_error_naming_the_operation() {
    // Port 1 is reserved and nothing listens on it, so the connection is refused.
    let gateway: StudentGatewaySamImpl =
        build_gateway_for("http://127.0.0.1:1").expect("client should be built");

    let (kind, details) =
        failure_of(gateway.get_available_records()).expect("students retrieval should have failed");

    assert_eq!(kind, FailureKind::Transient);
    assert!(details.contains("dashboard"), "got: {details}");
}

#[test]
fn a_client_reporting_invalid_credentials_for_a_data_fetch_fails_with_an_unknown_kind() {
    let gateway: StudentGatewaySamImpl = StudentGatewaySamImpl::new(Arc::new(FailingSamClient));

    let (kind, details) =
        failure_of(gateway.get_available_records()).expect("students retrieval should have failed");

    assert_eq!(kind, FailureKind::Unclassified);
    assert!(details.contains("Invalid credentials"), "got: {details}");
}

#[test]
fn a_student_keeps_the_id_and_name_sam_lists() {
    let student: Student = student_listed_as("MÚSICO", "CANDIDATO(A)", "A DEFINIR").unwrap();

    assert_eq!(student.id.as_str(), "99999");
    assert_eq!(student.name, "CARLOS ALBERTO DE NOBREGA");
}

#[test]
fn every_known_musician_level_is_recognized() {
    for (raw, level) in [
        ("CANDIDATO(A)", MusicianLevel::Candidate),
        ("ENSAIO", MusicianLevel::Practice),
        ("RJM", MusicianLevel::YouthService),
        ("CULTO OFICIAL", MusicianLevel::OfficialService),
    ] {
        assert_eq!(
            student_listed_as("MÚSICO", raw, "A DEFINIR")
                .unwrap()
                .position,
            musician(level, None),
            "level {raw:?}"
        );
    }
}

#[test]
fn a_musician_level_sam_has_not_confirmed_keeps_what_sam_wrote() {
    for raw in ["PRÁTICO(A)", "RJM / ENSAIO"] {
        assert_eq!(
            student_listed_as("MÚSICO", raw, "A DEFINIR")
                .unwrap()
                .position,
            musician(MusicianLevel::Unknown(raw.to_owned()), None)
        );
    }
}

#[test]
fn a_musician_keeps_the_exact_site_instrument_text_as_its_name() {
    assert_eq!(
        student_listed_as("MÚSICO", "RJM", "SAXOFONE TENOR")
            .unwrap()
            .position,
        musician(
            MusicianLevel::YouthService,
            Some((Instrument::Saxophone, "SAXOFONE TENOR"))
        )
    );
}

#[test]
fn the_instrument_name_is_trimmed() {
    assert_eq!(
        student_listed_as("MÚSICO", "RJM", "  OBOÉ \t")
            .unwrap()
            .position,
        musician(
            MusicianLevel::YouthService,
            Some((Instrument::Oboe, "OBOÉ"))
        )
    );
}

#[test]
fn a_musician_without_an_assigned_instrument_has_no_instrument_and_no_name() {
    for raw in ["A DEFINIR", " A DEFINIR ", "", "   "] {
        assert_eq!(
            student_listed_as("MÚSICO", "CANDIDATO(A)", raw)
                .unwrap()
                .position,
            musician(MusicianLevel::Candidate, None),
            "instrument column {raw:?}"
        );
    }
}

#[test]
fn every_instrument_sam_lists_is_recognized() {
    for (raw, instrument) in [
        ("VIOLINO", Instrument::Violin),
        ("VIOLA", Instrument::Viola),
        ("VIOLONCELO", Instrument::Cello),
        ("FLAUTA", Instrument::Flute),
        ("OBOÉ", Instrument::Oboe),
        ("FAGOTE", Instrument::Bassoon),
        ("CLARINETE", Instrument::Clarinet),
        ("CLARINETE ALTO", Instrument::AltoClarinet),
        ("CLARINETE BAIXO", Instrument::BassClarinet),
        ("TROMPA", Instrument::FrenchHorn),
        ("TROMBONE", Instrument::Trombone),
        ("EUPHONIUM", Instrument::Euphonium),
        ("TUBA", Instrument::Tuba),
        ("CORNE INGLÊS", Instrument::EnglishHorn),
        ("VIOLINO CONTRALTO", Instrument::ContraltoViolin),
    ] {
        assert_eq!(
            student_listed_as("MÚSICO", "RJM", raw).unwrap().position,
            musician(MusicianLevel::YouthService, Some((instrument, raw))),
            "instrument {raw:?}"
        );
    }
}

#[test]
fn saxophone_subtypes_all_count_as_saxophone() {
    for raw in [
        "SAXOFONE ALTO",
        "SAXOFONE SOPRANO CUR",
        "SAXOFONE SOPRANO RET",
        "SAXOFONE TENOR",
    ] {
        assert_eq!(
            student_listed_as("MÚSICO", "RJM", raw).unwrap().position,
            musician(
                MusicianLevel::YouthService,
                Some((Instrument::Saxophone, raw))
            )
        );
    }
}

#[test]
fn trumpet_aliases_count_as_trumpet() {
    for raw in ["TROMPETE", "CORNET", "FLUGELHORN"] {
        assert_eq!(
            student_listed_as("MÚSICO", "RJM", raw).unwrap().position,
            musician(
                MusicianLevel::YouthService,
                Some((Instrument::Trumpet, raw))
            )
        );
    }
}

#[test]
fn an_unrecognized_instrument_keeps_its_text() {
    assert_eq!(
        student_listed_as("MÚSICO", "ENSAIO", "BANDOLIM")
            .unwrap()
            .position,
        musician(
            MusicianLevel::Practice,
            Some((Instrument::Unknown("BANDOLIM".to_owned()), "BANDOLIM"))
        )
    );
}

#[test]
fn every_known_organist_level_is_recognized() {
    for (raw, level) in [
        ("CANDIDATO(A)", OrganistLevel::Candidate),
        ("ENSAIO", OrganistLevel::Practice),
        ("RJM", OrganistLevel::YouthService),
        ("CULTO OFICIAL", OrganistLevel::OfficialService),
        ("RJM / MEIA HORA", OrganistLevel::YouthServiceHalfHour),
        ("ALGO NOVO", OrganistLevel::Unknown("ALGO NOVO".to_owned())),
    ] {
        assert_eq!(
            student_listed_as("ORGANISTA", raw, "A DEFINIR")
                .unwrap()
                .position,
            StudentPosition::Organist { level },
            "level {raw:?}"
        );
    }
}

#[test]
fn a_gem_secretary_is_recognized() {
    assert_eq!(
        student_listed_as("SECRETÁRIO DO GEM", "RJM", "A DEFINIR")
            .unwrap()
            .position,
        StudentPosition::Secretary {
            r#type: SecretaryType::Gem,
        }
    );
}

#[test]
fn an_unknown_role_keeps_what_sam_wrote() {
    assert_eq!(
        student_listed_as("BATERISTA", "CANDIDATO(A)", "A DEFINIR")
            .unwrap()
            .position,
        StudentPosition::Unknown("BATERISTA".to_owned())
    );
}

#[test]
fn positions_other_than_musician_ignore_the_instrument_column() {
    assert_eq!(
        student_listed_as("ORGANISTA", "RJM", "VIOLINO")
            .unwrap()
            .position,
        StudentPosition::Organist {
            level: OrganistLevel::YouthService,
        }
    );
    assert_eq!(
        student_listed_as("SECRETÁRIO DO GEM", "RJM", "VIOLINO")
            .unwrap()
            .position,
        StudentPosition::Secretary {
            r#type: SecretaryType::Gem,
        }
    );
    assert_eq!(
        student_listed_as("BATERISTA", "RJM", "VIOLINO")
            .unwrap()
            .position,
        StudentPosition::Unknown("BATERISTA".to_owned())
    );
}

#[test]
fn the_region_is_read_from_the_location() {
    assert_eq!(
        student_located_at(LOCATION).unwrap().region,
        Region::AraraquaraSaoCarlos
    );
    assert_eq!(
        student_located_at("JARDIM SÃO PAULO | BR-SP-ARARAQUARA-ITIRAPINA")
            .unwrap()
            .region,
        Region::AraraquaraItirapina
    );
}

#[test]
fn an_unrecognized_region_keeps_the_location_sam_wrote() {
    assert_eq!(
        student_located_at("SOME OTHER LOCATION").unwrap().region,
        Region::Other("SOME OTHER LOCATION".to_owned())
    );
}

#[test]
fn html_markup_is_stripped_from_the_location() {
    assert_eq!(
        student_located_at(LOCATION).unwrap().location,
        "JARDIM PALMARES DO SUL | BR-SP-ARARAQUARA-SÃO CARLOS"
    );
}

#[test]
fn a_location_without_markup_is_unchanged() {
    assert_eq!(
        student_located_at("SOME PLAIN LOCATION").unwrap().location,
        "SOME PLAIN LOCATION"
    );
    assert_eq!(student_located_at("").unwrap().location, "");
}

const LOCATION: &str = "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS";

fn student_listed_as(role: &str, level: &str, instrument: &str) -> Option<Student> {
    student_listed(sam_student(role, level, instrument, LOCATION))
}

fn student_located_at(location: &str) -> Option<Student> {
    student_listed(sam_student("MÚSICO", "RJM", "VIOLINO", location))
}

fn student_listed(row: SamStudent) -> Option<Student> {
    let gateway = StudentGatewaySamImpl::new(Arc::new(FakeSamClient::listing(vec![row])));

    gateway.get_available_records().ok()?.pop()
}

fn musician(level: MusicianLevel, instrument: Option<(Instrument, &str)>) -> StudentPosition {
    let (instrument, instrument_name) = instrument.map_or((None, None), |(instrument, name)| {
        (Some(instrument), Some(name.to_owned()))
    });

    StudentPosition::Musician {
        level,
        instrument,
        instrument_name,
    }
}

fn build_gateway(mock_server: &MockServer) -> Result<StudentGatewaySamImpl, reqwest::Error> {
    build_gateway_for(&mock_server.uri())
}

fn build_gateway_for(base_url: &str) -> Result<StudentGatewaySamImpl, reqwest::Error> {
    let sam_operations: SamOperations = sam_operations_for(base_url)?;

    let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(StudentGatewaySamImpl::new(sam_client))
}

fn failure_of(result: Result<Vec<Student>, StudentGatewayError>) -> Option<(FailureKind, String)> {
    match result {
        Err(StudentGatewayError::UnableToPerformOperation { kind, details }) => {
            Some((kind, details))
        }
        Ok(_) => None,
    }
}

struct FailingSamClient;

impl SamClient for FailingSamClient {
    fn login(&self, _credentials: &SamCredentials) -> Result<(), SamClientError> {
        Err(SamClientError::InvalidCredentials)
    }

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
        Err(SamClientError::InvalidCredentials)
    }

    fn student_lessons(&self, _student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
        Err(SamClientError::InvalidCredentials)
    }
}
