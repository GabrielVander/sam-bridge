import 'package:flutter_application/view_models.dart';

class LessonsMapper {
  static StudentLessonsView toViewModel(StudentLessonsView dto) => dto;

  static LessonItem toLessonItem(LessonItem dto) => dto;
}
