import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/settings/settings_presenter.dart';
import 'package:flutter_application/startup_failure_app.dart';
import 'package:flutter_application/rust/api.dart';
import 'package:flutter_application/rust/api/error_report.dart';
import 'package:flutter_application/rust/frb_generated.dart';
import 'package:flutter_application/window/desktop_window.dart'
    if (dart.library.js_interop) 'package:flutter_application/window/desktop_window_web.dart';
import 'package:flutter_application/window/window_controls.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:shared_preferences/shared_preferences.dart';

const String _chosenLocationsKey = 'roster.chosenLocations';
const String _languageKey = 'settings.language';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  final PackageInfo packageInfo = await PackageInfo.fromPlatform();
  final SharedPreferencesWithCache preferences =
      await SharedPreferencesWithCache.create(
        cacheOptions: const SharedPreferencesWithCacheOptions(
          allowList: {_chosenLocationsKey, _languageKey},
        ),
      );

  await _start(
    versionDisplay: formatVersion(
      version: packageInfo.version,
      buildNumber: packageInfo.buildNumber,
    ),
    windowControls: await frameDesktopWindow(),
    preferences: preferences,
    settingsPresenter: SettingsPresenter(
      rememberedLanguage: preferences.getString(_languageKey),
      rememberLanguage: (languageCode) => languageCode == null
          ? preferences.remove(_languageKey)
          : preferences.setString(_languageKey, languageCode),
    ),
  );
}

Future<void> _start({
  required String versionDisplay,
  required WindowControls? windowControls,
  required SharedPreferencesWithCache preferences,
  required SettingsPresenter settingsPresenter,
}) async {
  final ApplicationFacade application;

  try {
    application = await buildMainApplication();
  } on ErrorReportDto catch (report) {
    runApp(
      StartupFailureApp(
        report: ErrorReportMapper.toViewModel(report),
        onRetry: () => _start(
          versionDisplay: versionDisplay,
          windowControls: windowControls,
          preferences: preferences,
          settingsPresenter: settingsPresenter,
        ),
        windowControls: windowControls,
        locale: settingsPresenter.stateValue.appLocale,
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
    rememberedLocations: {...?preferences.getStringList(_chosenLocationsKey)},
    rememberLocations: (locations) =>
        preferences.setStringList(_chosenLocationsKey, locations.toList()),
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
      settingsPresenter: settingsPresenter,
      windowControls: windowControls,
    ),
  );
}

String formatVersion({required String version, required String buildNumber}) =>
    'v$version+$buildNumber';
