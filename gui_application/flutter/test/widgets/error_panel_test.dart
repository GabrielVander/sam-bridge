import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_test/flutter_test.dart';

const _details = 'Request failed for operation dashboard: connection refused';

const _report = ErrorReport(
  userMessage: 'Não foi possível conectar ao SAM.',
  details: _details,
);

Future<void> pumpPanel(
  WidgetTester tester, {
  ErrorReport report = _report,
  VoidCallback? onRetry,
}) async {
  await tester.pumpWidget(
    MaterialApp(
      home: Scaffold(
        body: ErrorPanel(report: report, onRetry: onRetry ?? () {}),
      ),
    ),
  );
}

void main() {
  group('ErrorPanel', () {
    testWidgets('shows the friendly message', (tester) async {
      await pumpPanel(tester);

      expect(find.text('Não foi possível conectar ao SAM.'), findsOneWidget);
    });

    testWidgets('keeps the technical details hidden until expanded', (
      tester,
    ) async {
      await pumpPanel(tester);

      expect(find.text('Detalhes técnicos'), findsOneWidget);
      expect(find.text(_details), findsNothing);

      await tester.tap(find.text('Detalhes técnicos'));
      await tester.pumpAndSettle();

      expect(find.text(_details), findsOneWidget);
    });

    testWidgets('copies the details to the clipboard and confirms it', (
      tester,
    ) async {
      String? copied;
      tester.binding.defaultBinaryMessenger.setMockMethodCallHandler(
        SystemChannels.platform,
        (call) async {
          if (call.method == 'Clipboard.setData') {
            copied = (call.arguments as Map)['text'] as String;
          }
          return null;
        },
      );
      addTearDown(
        () => tester.binding.defaultBinaryMessenger.setMockMethodCallHandler(
          SystemChannels.platform,
          null,
        ),
      );
      await pumpPanel(tester);
      await tester.tap(find.text('Detalhes técnicos'));
      await tester.pumpAndSettle();

      await tester.tap(find.byTooltip('Copiar detalhes'));
      await tester.pump();

      expect(copied, _details);
      expect(find.text('Detalhes copiados'), findsOneWidget);
    });

    testWidgets('calls onRetry when "Tentar novamente" is tapped', (
      tester,
    ) async {
      var retries = 0;
      await pumpPanel(tester, onRetry: () => retries++);

      await tester.tap(find.text('Tentar novamente'));

      expect(retries, 1);
    });

    testWidgets('shows no expander when there are no details', (tester) async {
      await pumpPanel(
        tester,
        report: const ErrorReport(userMessage: 'Algo deu errado.', details: ''),
      );

      expect(find.text('Algo deu errado.'), findsOneWidget);
      expect(find.text('Detalhes técnicos'), findsNothing);
    });
  });
}
