import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/application/use_cases/login_use_case.dart';
import 'package:flutter_application/authentication/application/use_cases/restore_session_use_case.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/lessons/application/use_cases/assess_student_progress_use_case.dart';
import 'package:flutter_application/lessons/application/use_cases/retrieve_student_lessons_use_case.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/application/use_cases/retrieve_students_use_case.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/router.dart';

String formatVersion({required String version, required String buildNumber}) =>
    'v$version+$buildNumber';

/// Wires the presenters to the application's use cases and restores any saved
/// session, so the first screen shown already reflects whether the user is
/// signed in.
Future<SamSiteApp> composeApp({
  required String versionDisplay,
  required LoginUseCase login,
  required RestoreSessionUseCase restoreSession,
  required RetrieveStudentsUseCase retrieveStudents,
  required RetrieveStudentLessonsUseCase retrieveStudentLessons,
  required AssessStudentProgressUseCase assessStudentProgress,
}) async {
  final AuthPresenter authPresenter = AuthPresenter(
    loginUseCase: login,
    restoreSessionUseCase: restoreSession,
  );

  await authPresenter.restoreSession();

  return SamSiteApp(
    versionDisplay: versionDisplay,
    authPresenter: authPresenter,
    studentsPresenter: StudentsPresenter(retrieveStudents: retrieveStudents),
    lessonsCubitSignal: LessonsCubitSignal(
      retrieveStudentLessons: retrieveStudentLessons,
      assessStudentProgress: assessStudentProgress,
    ),
  );
}

class SamSiteApp extends StatelessWidget {
  final String versionDisplay;
  final AuthPresenter authPresenter;
  final StudentsPresenter studentsPresenter;
  final LessonsCubitSignal lessonsCubitSignal;

  const SamSiteApp({
    super.key,
    required this.versionDisplay,
    required this.authPresenter,
    required this.studentsPresenter,
    required this.lessonsCubitSignal,
  });

  @override
  Widget build(BuildContext context) {
    return MultiBlocSignalProvider(
      providers: [
        BlocSignalProvider<AuthPresenter>.value(value: authPresenter),
        BlocSignalProvider<StudentsPresenter>.value(value: studentsPresenter),
        BlocSignalProvider<LessonsCubitSignal>.value(value: lessonsCubitSignal),
      ],
      child: Builder(
        builder: (context) {
          return MaterialApp.router(
            title: 'SamSite GUI',
            routerConfig: buildRouter(
              appVersion: versionDisplay,
              authPresenter: authPresenter,
            ),
            debugShowCheckedModeBanner: false,
            theme: _buildTheme(),
          );
        },
      ),
    );
  }
}

ThemeData _buildTheme() {
  final colorScheme = ColorScheme.fromSeed(
    seedColor: Colors.cyan,
    brightness: Brightness.dark,
  );

  return ThemeData(
    colorScheme: colorScheme,
    useMaterial3: true,
    scaffoldBackgroundColor: colorScheme.surface,
    appBarTheme: const AppBarTheme(elevation: 0, scrolledUnderElevation: 1),
    cardTheme: CardThemeData(
      elevation: 0,
      color: colorScheme.surfaceContainerHigh,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      clipBehavior: Clip.antiAlias,
    ),
    listTileTheme: const ListTileThemeData(
      contentPadding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
    ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: colorScheme.surfaceContainerHighest.withValues(alpha: 0.5),
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(12),
        borderSide: BorderSide.none,
      ),
    ),
    dividerTheme: DividerThemeData(
      color: colorScheme.outlineVariant.withValues(alpha: 0.4),
    ),
  );
}
