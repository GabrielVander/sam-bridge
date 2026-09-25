import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/rust/api.dart';
import 'package:flutter_application/rust/frb_generated.dart';
import 'package:package_info_plus/package_info_plus.dart';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  final PackageInfo packageInfo = await PackageInfo.fromPlatform();
  final ApplicationFacade application = await buildMainApplication();

  final AuthPresenter authPresenter = AuthPresenter(
    loginUseCase: application.login,
    restoreSessionUseCase: application.restoreSession,
    logoutUseCase: application.logout,
  );
  final StudentsPresenter studentsPresenter = StudentsPresenter(
    retrieveStudents: application.retrieveAllAvailableStudents,
  );
  final LessonsPresenter lessonsPresenter = LessonsPresenter(
    retrieveStudentLessons: application.retrieveStudentLessons,
    assessStudentProgress: application.assessStudentProgress,
  );

  await authPresenter.restoreSession();

  runApp(
    SamSiteApp(
      versionDisplay: formatVersion(
        version: packageInfo.version,
        buildNumber: packageInfo.buildNumber,
      ),
      authPresenter: authPresenter,
      studentsPresenter: studentsPresenter,
      lessonsPresenter: lessonsPresenter,
    ),
  );
}

String formatVersion({required String version, required String buildNumber}) =>
    'v$version+$buildNumber';
