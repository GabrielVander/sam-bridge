use crate::view_models::{position_label, StudentListItem};
use student_management::api::application::StudentSummaryDto;
use student_management::api::domain::StudentPosition;

pub fn to_list_items(dtos: &[StudentSummaryDto]) -> Vec<StudentListItem> {
    dtos.iter()
        .map(|dto| StudentListItem {
            id: dto.id.clone(),
            name: dto.name.clone(),
            location: dto.location.clone(),
            position: position_label(&dto.position),
            raw_level: extract_raw_level(&dto.position),
        })
        .collect()
}

fn extract_raw_level(position: &StudentPosition) -> String {
    match position {
        StudentPosition::Musician { level } => level.name(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::application::StudentSummaryDto;
    use student_management::api::domain::{MusicianLevel, Region, StudentPosition};

    #[test]
    fn given_students_should_map_to_list_items() {
        let dtos = vec![StudentSummaryDto {
            id: "1".to_owned(),
            name: "ALUNA UM".to_owned(),
            location: "BAIRRO".to_owned(),
            position: StudentPosition::Musician { level: MusicianLevel::Candidate },
            region: Region::AraraquaraSaoCarlos,
        }];

        let items = to_list_items(&dtos);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "1");
        assert_eq!(items[0].name, "ALUNA UM");
        assert_eq!(items[0].position, "Músico · Candidato(a)");
    }

    #[test]
    fn given_empty_slice_should_return_empty() {
        let items = to_list_items(&[]);
        assert!(items.is_empty());
    }
}
