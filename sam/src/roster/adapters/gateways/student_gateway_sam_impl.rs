use std::sync::Arc;

use async_trait::async_trait;
use student::{
    application::gateways::{StudentGateway, StudentGatewayError},
    domain::entities::{MusicianLevel, Region, Student, StudentPosition},
};

use crate::client::{SamClient, SamStudent};

pub struct StudentGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl StudentGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl StudentGateway for StudentGatewaySamImpl {
    async fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        let sam_students: Vec<SamStudent> = self
            .client
            .students()
            .map_err(|_| StudentGatewayError::UnableToPerformOperation)?;

        Ok(sam_students.into_iter().map(Student::from).collect())
    }
}

impl From<SamStudent> for Student {
    fn from(sam_student: SamStudent) -> Self {
        let position: StudentPosition = parse_position(&sam_student.role, &sam_student.level);
        let region: Region = parse_region(&sam_student.location);
        let location: String = clean_location(&sam_student.location);

        Self {
            id: sam_student.id,
            name: sam_student.name,
            position,
            location,
            region,
        }
    }
}

/// SAM embeds presentational markup directly in the location string (e.g.
/// `<span class='m-r-10'></span>` separators) — strip it so only plain text
/// ever reaches the UI.
fn clean_location(raw: &str) -> String {
    let mut without_tags: String = String::with_capacity(raw.len());
    let mut inside_tag: bool = false;

    for ch in raw.chars() {
        match ch {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => without_tags.push(ch),
            _ => {}
        }
    }

    without_tags
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// SAM's raw `role`/`level` vocabulary is only partially confirmed against
/// the real portal today (see `sam/src/parsing/students_listing.rs`'s test
/// fixtures and `sam/tests/sam_http_capabilities_and_behaviour.rs`).
/// Unrecognized raw values intentionally fall through to `Unknown` rather
/// than guessing — extend these as more of SAM's vocabulary is confirmed.
fn parse_position(role: &str, level: &str) -> StudentPosition {
    match role {
        "MÚSICO" => StudentPosition::Musician {
            level: parse_musician_level(level),
        },
        other => StudentPosition::Unknown(other.to_owned()),
    }
}

fn parse_musician_level(level: &str) -> MusicianLevel {
    match level {
        "CANDIDATO(A)" => MusicianLevel::Candidate,
        other => MusicianLevel::Unknown(other.to_owned()),
    }
}

/// `Student.region` is currently unused by `StudentSummaryDto`'s mapping, so
/// imprecision here doesn't affect what's displayed today.
fn parse_region(location: &str) -> Region {
    if location.contains("ARARAQUARA-SÃO CARLOS") {
        Region::AraraquaraSaoCarlos
    } else if location.contains("ARARAQUARA-ITIRAPINA") {
        Region::AraraquaraItirapina
    } else {
        Region::Other(location.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use student::domain::entities::{MusicianLevel, Region, StudentPosition};

    use super::{clean_location, parse_musician_level, parse_position, parse_region};

    #[test]
    fn known_musician_candidate_is_recognized() {
        assert_eq!(
            parse_position("MÚSICO", "CANDIDATO(A)"),
            StudentPosition::Musician {
                level: MusicianLevel::Candidate
            }
        );
    }

    #[test]
    fn unknown_role_falls_through_to_unknown() {
        assert_eq!(
            parse_position("ORGANISTA", "CANDIDATO(A)"),
            StudentPosition::Unknown("ORGANISTA".to_owned())
        );
    }

    #[test]
    fn unknown_musician_level_falls_through_to_unknown() {
        assert_eq!(
            parse_musician_level("PRÁTICO(A)"),
            MusicianLevel::Unknown("PRÁTICO(A)".to_owned())
        );
    }

    #[test]
    fn confirmed_region_is_recognized() {
        assert_eq!(
            parse_region(
                "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS"
            ),
            Region::AraraquaraSaoCarlos
        );
    }

    #[test]
    fn unrecognized_region_falls_through_to_other() {
        assert_eq!(
            parse_region("SOME OTHER LOCATION"),
            Region::Other("SOME OTHER LOCATION".to_owned())
        );
    }

    #[test]
    fn html_markup_is_stripped_from_location() {
        assert_eq!(
            clean_location(
                "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS"
            ),
            "JARDIM PALMARES DO SUL | BR-SP-ARARAQUARA-SÃO CARLOS"
        );
    }

    #[test]
    fn location_without_markup_is_unchanged() {
        assert_eq!(clean_location("SOME PLAIN LOCATION"), "SOME PLAIN LOCATION");
    }

    #[test]
    fn empty_location_stays_empty() {
        assert_eq!(clean_location(""), "");
    }
}
