import 'package:flutter_application/view_models.dart';

class RosterMapper {
  static StudentListItem toViewModel(StudentListItem dto) => dto;

  static List<StudentListItem> toViewModels(List<StudentListItem> dtos) =>
      dtos.map(toViewModel).toList();

  static String displayLocation(StudentListItem item) => item.location;
}
