import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/widgets/error_panel.dart';

class StartupFailureApp extends StatelessWidget {
  final ErrorReport report;
  final VoidCallback onRetry;

  const StartupFailureApp({
    super.key,
    required this.report,
    required this.onRetry,
  });

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      onGenerateTitle: (context) => context.l10n.appTitle,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      debugShowCheckedModeBanner: false,
      theme: buildTheme(),
      home: Scaffold(
        body: ErrorPanel(report: report, onRetry: onRetry),
      ),
    );
  }
}
