import 'package:flutter_application/rust/api/lessons.dart';

typedef RetrieveStudentLessonsUseCase =
    Future<RetrieveStudentLessonsOutcomeDto> Function({
      required String studentId,
    });
