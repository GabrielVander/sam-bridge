import 'package:flutter_application/rust/api/lessons.dart';

typedef RetrieveStudentLessonsUseCase =
    Future<RetrieveStudentLessonsOutcome> Function({required String studentId});
