import 'package:flutter_application/app.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';

import 'roster.dart';

Future<SamSiteApp> composeFakeApp({
  RestoreSessionOutcome restoreSession = const RestoreSessionOutcome.notAvailable(),
  LoginResult login = const LoginResult.successful(),
  LogoutOutcome logout = LogoutOutcome.successful,
  List<StudentSummaryDto>? students,
  String versionDisplay = 'v1.0.0+1',
}) async {
  final AuthPresenter authPresenter = AuthPresenter(
    loginUseCase: ({required email, required password}) async => login,
    restoreSessionUseCase: () async => restoreSession,
    logoutUseCase: () async => logout,
  );
  final StudentsPresenter studentsPresenter = StudentsPresenter(
    retrieveStudents: () async => RetrieveAllAvailableStudentsOutcome.success(
      students ?? [studentSummary(id: '500132', name: 'Jane Doe')],
    ),
  );
  final LessonsPresenter lessonsPresenter = LessonsPresenter(
    retrieveStudentLessons: ({required studentId}) async =>
        const RetrieveStudentLessonsOutcome.success(
          StudentLessonsDto(approved: [], method: []),
        ),
    assessStudentProgress: ({required studentId}) async =>
        const AssessStudentProgressOutcome.noInstrumentAssigned(),
  );

  await authPresenter.restoreSession();

  return SamSiteApp(
    versionDisplay: versionDisplay,
    authPresenter: authPresenter,
    studentsPresenter: studentsPresenter,
    lessonsCubitSignal: lessonsPresenter,
  );
}
