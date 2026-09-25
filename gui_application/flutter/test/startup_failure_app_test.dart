import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/startup_failure_app.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const ErrorReport report = ErrorReport(
    userMessage: 'Não foi possível acessar os dados salvos neste dispositivo.',
    details: 'Unable to set up the credential storage: no data directory',
  );

  testWidgets('explains why the app could not start', (tester) async {
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
    await tester.pumpWidget(
      StartupFailureApp(report: report, onRetry: () => retries++),
    );

    await tester.tap(find.text('Tentar novamente'));

    expect(retries, 1);
  });
}
