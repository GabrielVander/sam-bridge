import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/errors/error_report.dart';
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
      title: 'SAM Bridge',
      debugShowCheckedModeBanner: false,
      theme: buildTheme(),
      home: Scaffold(
        body: ErrorPanel(report: report, onRetry: onRetry),
      ),
    );
  }
}
