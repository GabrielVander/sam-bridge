import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/startup_failure_app.dart';
import 'package:flutter_application/rust/api.dart';
import 'package:flutter_application/rust/api/error_report.dart';
import 'package:flutter_application/rust/frb_generated.dart';
import 'package:package_info_plus/package_info_plus.dart';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  final PackageInfo packageInfo = await PackageInfo.fromPlatform();

  await _start(
    versionDisplay: formatVersion(
      version: packageInfo.version,
      buildNumber: packageInfo.buildNumber,
    ),
  );
}

Future<void> _start({required String versionDisplay}) async {
  final ApplicationFacade application;

  try {
    application = await buildMainApplication();
  } on ErrorReportDto catch (report) {
    runApp(
      StartupFailureApp(
        report: ErrorReportMapper.toViewModel(report),
        onRetry: () => _start(versionDisplay: versionDisplay),
      ),
    );

    return;
  }

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
      versionDisplay: versionDisplay,
      authPresenter: authPresenter,
      studentsPresenter: studentsPresenter,
      lessonsPresenter: lessonsPresenter,
    ),
  );
}

String formatVersion({required String version, required String buildNumber}) =>
    'v$version+$buildNumber';
