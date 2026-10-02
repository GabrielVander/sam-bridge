import 'package:flutter/material.dart';
import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/clipboard.dart';
import '../support/localization.dart';

const _details = 'Request failed for operation dashboard: connection refused';

const _report = ErrorReport(reason: ErrorReason.network, details: _details);

void main() {
  group('ErrorPanel', () {
    const messages = {
      ErrorReason.network:
          'Não foi possível conectar ao SAM. '
          'Verifique sua conexão com a internet e tente novamente.',
      ErrorReason.unexpectedResponse:
          'O SAM respondeu de forma inesperada. '
          'Tente novamente em instantes.',
      ErrorReason.sessionExpired: 'Sua sessão expirou. Entre novamente.',
      ErrorReason.localStorage:
          'Não foi possível acessar os dados salvos neste dispositivo.',
      ErrorReason.generic: 'Algo deu errado. Tente novamente.',
    };

    for (final MapEntry(key: reason, value: message) in messages.entries) {
      testWidgets('explains a ${reason.name} failure', (tester) async {
        await pumpPanel(
          tester,
          report: ErrorReport(reason: reason, details: _details),
        );

        expect(find.text(message), findsOneWidget);
      });
    }

    testWidgets('explains the failure in English', (tester) async {
      await pumpPanel(tester, locale: const Locale('en'));

      expect(
        find.text(
          'Could not reach SAM. '
          'Check your internet connection and try again.',
        ),
        findsOneWidget,
      );
      expect(find.text('Try again'), findsOneWidget);
      expect(find.text('Technical details'), findsOneWidget);
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
      final copied = recordClipboard(tester);
      await pumpPanel(tester);
      await tester.tap(find.text('Detalhes técnicos'));
      await tester.pumpAndSettle();

      await tester.tap(find.byTooltip('Copiar detalhes'));
      await tester.pump();

      expect(copied(), _details);
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
        report: const ErrorReport(reason: ErrorReason.generic, details: ''),
      );

      expect(find.text('Algo deu errado. Tente novamente.'), findsOneWidget);
      expect(find.text('Detalhes técnicos'), findsNothing);
    });
  });
}

Future<void> pumpPanel(
  WidgetTester tester, {
  ErrorReport report = _report,
  VoidCallback? onRetry,
  Locale locale = portuguese,
}) async {
  await tester.pumpWidget(
    localizedApp(
      locale: locale,
      home: Scaffold(
        body: ErrorPanel(report: report, onRetry: onRetry ?? () {}),
      ),
    ),
  );
}
