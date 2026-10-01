import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_application/window/window_controls.dart';
import 'package:flutter_application/window/window_title_bar.dart';

class StartupFailureApp extends StatelessWidget {
  final ErrorReport report;
  final VoidCallback onRetry;
  final WindowControls? windowControls;
  final Locale? locale;

  const StartupFailureApp({
    super.key,
    required this.report,
    required this.onRetry,
    this.windowControls,
    this.locale,
  });

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      onGenerateTitle: (context) => context.l10n.appTitle,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      locale: locale,
      debugShowCheckedModeBanner: false,
      theme: buildTheme(),
      home: Scaffold(
        appBar: windowControls == null
            ? null
            : WindowTitleBar(title: '', controls: windowControls),
        body: ErrorPanel(report: report, onRetry: onRetry),
      ),
    );
  }
}
