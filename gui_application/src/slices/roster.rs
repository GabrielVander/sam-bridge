use crate::view_models::StudentListItem;
use student_management::api::domain::Student;

pub fn to_list_items(students: &[Student]) -> Vec<StudentListItem> {
    students.iter().map(StudentListItem::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::domain::{MusicianLevel, Region, StudentPosition};

    #[test]
    fn given_students_should_map_to_list_items() {
        let students = vec![Student {
            id: "1".to_owned(),
            name: "ALUNA UM".to_owned(),
            position: StudentPosition::Musician {
                level: MusicianLevel::Candidate,
            },
            location: "BAIRRO".to_owned(),
            region: Region::AraraquaraSaoCarlos,
        }];

        let items = to_list_items(&students);

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
