import 'package:flutter_application/rust/api/progress.dart';

typedef AssessStudentProgressUseCase =
    Future<AssessStudentProgressOutcome> Function({required String studentId});
