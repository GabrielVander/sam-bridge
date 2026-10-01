import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/startup_failure_app.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'support/localization.dart';

void main() {
  const ErrorReport report = ErrorReport(
    reason: ErrorReason.localStorage,
    details: 'Unable to set up the credential storage: no data directory',
  );

  testWidgets('explains why the app could not start', (tester) async {
    setOsLocale(tester, brazilianPortuguese);
    await tester.pumpWidget(StartupFailureApp(report: report, onRetry: () {}));
    await tester.tap(find.text('Detalhes técnicos'));
    await tester.pumpAndSettle();

    expect(
      find.text('Não foi possível acessar os dados salvos neste dispositivo.'),
      findsOneWidget,
    );
    expect(
      find.text('Unable to set up the credential storage: no data directory'),
      findsOneWidget,
    );
  });

  testWidgets('lets the user try to start again', (tester) async {
    var retries = 0;
    setOsLocale(tester, brazilianPortuguese);
    await tester.pumpWidget(
      StartupFailureApp(report: report, onRetry: () => retries++),
    );

    await tester.tap(find.text('Tentar novamente'));

    expect(retries, 1);
  });

  testWidgets('explains the failure in English when the OS is in English', (
    tester,
  ) async {
    setOsLocale(tester, const Locale('en', 'US'));
    await tester.pumpWidget(StartupFailureApp(report: report, onRetry: () {}));

    expect(
      find.text('Could not access the data saved on this device.'),
      findsOneWidget,
    );
    expect(find.text('Try again'), findsOneWidget);
  });

  testWidgets('explains the failure in the language the user chose', (
    tester,
  ) async {
    setOsLocale(tester, brazilianPortuguese);
    await tester.pumpWidget(
      StartupFailureApp(
        report: report,
        onRetry: () {},
        locale: const Locale('en'),
      ),
    );

    expect(find.text('Try again'), findsOneWidget);
  });
}
