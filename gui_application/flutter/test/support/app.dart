import 'package:flutter_application/app.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/rust/api/authentication.dart';
import 'package:flutter_application/rust/api/lessons.dart';
import 'package:flutter_application/rust/api/progress.dart';
import 'package:flutter_application/rust/api/roster.dart';

import 'roster.dart';

Future<SamSiteApp> composeFakeApp({
  RestoreSessionOutcomeDto restoreSession =
      const RestoreSessionOutcomeDto.notAvailable(),
  LoginOutcomeDto login = const LoginOutcomeDto.successful(),
  LogoutOutcomeDto logout = const LogoutOutcomeDto.successful(),
  List<StudentSummaryDto>? students,
  String versionDisplay = 'v1.0.0+1',
}) async {
  final AuthPresenter authPresenter = AuthPresenter(
    loginUseCase: ({required email, required password}) async => login,
    restoreSessionUseCase: () async => restoreSession,
    logoutUseCase: () async => logout,
  );
  final StudentsPresenter studentsPresenter = StudentsPresenter(
    retrieveStudents: () async =>
        RetrieveAllAvailableStudentsOutcomeDto.success(
          students:
              students ?? [studentSummary(id: '500132', name: 'Jane Doe')],
        ),
  );
  final LessonsPresenter lessonsPresenter = LessonsPresenter(
    retrieveStudentLessons: ({required studentId}) async =>
        const RetrieveStudentLessonsOutcomeDto.success(
          lessons: StudentLessonsDto(msa: [], method: []),
        ),
    assessStudentProgress: ({required studentId}) async =>
        const AssessStudentProgressOutcomeDto.noInstrumentAssigned(),
  );

  await authPresenter.restoreSession();

  return SamSiteApp(
    versionDisplay: versionDisplay,
    authPresenter: authPresenter,
    studentsPresenter: studentsPresenter,
    lessonsPresenter: lessonsPresenter,
  );
}
