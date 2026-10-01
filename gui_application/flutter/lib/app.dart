import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/router.dart';
import 'package:flutter_application/window/window_controls.dart';
import 'package:go_router/go_router.dart';

class SamSiteApp extends StatefulWidget {
  final String versionDisplay;
  final AuthPresenter authPresenter;
  final StudentsPresenter studentsPresenter;
  final LessonsPresenter lessonsPresenter;
  final WindowControls? windowControls;

  const SamSiteApp({
    super.key,
    required this.versionDisplay,
    required this.authPresenter,
    required this.studentsPresenter,
    required this.lessonsPresenter,
    this.windowControls,
  });

  @override
  State<SamSiteApp> createState() => _SamSiteAppState();
}

class _SamSiteAppState extends State<SamSiteApp> {
  late final GoRouter _router = buildRouter(
    appVersion: widget.versionDisplay,
    windowControls: widget.windowControls,
    authPresenter: widget.authPresenter,
  );

  @override
  Widget build(BuildContext context) {
    return MultiBlocSignalProvider(
      providers: [
        BlocSignalProvider<AuthPresenter>.value(value: widget.authPresenter),
        BlocSignalProvider<StudentsPresenter>.value(
          value: widget.studentsPresenter,
        ),
        BlocSignalProvider<LessonsPresenter>.value(
          value: widget.lessonsPresenter,
        ),
      ],
      child: MaterialApp.router(
        onGenerateTitle: (context) => context.l10n.appTitle,
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        routerConfig: _router,
        debugShowCheckedModeBanner: false,
        theme: buildTheme(),
      ),
    );
  }

  @override
  void dispose() {
    _router.dispose();
    super.dispose();
  }
}

ThemeData buildTheme() {
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
