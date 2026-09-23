import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/router.dart';

class SamSiteApp extends StatelessWidget {
  final String versionDisplay;
  final AuthPresenter authPresenter;
  final StudentsPresenter studentsPresenter;
  final LessonsPresenter lessonsCubitSignal;

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
        BlocSignalProvider<LessonsPresenter>.value(value: lessonsCubitSignal),
      ],
      child: Builder(
        builder: (context) {
          return MaterialApp.router(
            title: 'SAM Bridge',
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
