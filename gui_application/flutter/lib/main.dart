import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/rust/api.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/frb_generated.dart';
import 'package:package_info_plus/package_info_plus.dart';

/// Starts the native library and hands its use cases to [composeApp].
///
/// Everything else lives in `app.dart` so it can be tested; this file needs the
/// compiled Rust library and platform channels, so it is excluded from the
/// coverage report (see `tool/coverage.sh`) and only exercised by running the
/// app.
Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  final PackageInfo packageInfo = await PackageInfo.fromPlatform();
  final ApplicationFacade application = await buildMainApplication();

  runApp(
    await composeApp(
      versionDisplay: formatVersion(
        version: packageInfo.version,
        buildNumber: packageInfo.buildNumber,
      ),
      login: application.login,
      restoreSession: application.restoreSession,
      logout: application.logout,
      retrieveStudents: application.retrieveAllAvailableStudents,
      retrieveStudentLessons: application.retrieveStudentLessons,
      assessStudentProgress: application.assessStudentProgress,
    ),
  );
}
