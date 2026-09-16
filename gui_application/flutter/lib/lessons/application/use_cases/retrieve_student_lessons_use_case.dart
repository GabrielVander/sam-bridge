import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';

typedef RetrieveStudentLessonsUseCase =
    Future<RetrieveStudentLessonsOutcome> Function({required String studentId});
