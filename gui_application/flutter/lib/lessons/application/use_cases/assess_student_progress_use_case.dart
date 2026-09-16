import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';

typedef AssessStudentProgressUseCase =
    Future<AssessStudentProgressOutcome> Function({required String studentId});
