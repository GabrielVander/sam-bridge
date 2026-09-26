import 'package:flutter_application/rust/api/progress.dart';

typedef AssessStudentProgressUseCase =
    Future<AssessStudentProgressOutcomeDto> Function({
      required String studentId,
    });
