import 'package:flutter_application/app.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';

import 'roster.dart';

/// The whole app wired to a stand-in for the Rust application, so tests can
/// walk through screens without the native library. Like the other support
/// files, it is the only place that touches the generated bridge types.
Future<SamSiteApp> composeFakeApp({
  RestoreSessionOutcome restoreSession = RestoreSessionOutcome.notAvailable,
  LoginResult login = const LoginResult.successful(),
  LogoutOutcome logout = LogoutOutcome.successful,
  List<StudentSummaryDto>? students,
  String versionDisplay = 'v1.0.0+1',
}) => composeApp(
  versionDisplay: versionDisplay,
  login: ({required email, required password}) async => login,
  restoreSession: () async => restoreSession,
  logout: () async => logout,
  retrieveStudents: () async => RetrieveAllAvailableStudentsOutcome.success(
    students ?? [studentSummary(id: '500132', name: 'Jane Doe')],
  ),
  retrieveStudentLessons: ({required studentId}) async =>
      const RetrieveStudentLessonsOutcome.success(
        StudentLessonsDto(approved: [], method: []),
      ),
  assessStudentProgress: ({required studentId}) async =>
      const AssessStudentProgressOutcome.noInstrumentAssigned(),
);
