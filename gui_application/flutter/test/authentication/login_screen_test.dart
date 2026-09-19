import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/authentication/login_screen.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_test/flutter_test.dart';

const _details = "Request failed for operation 'authentication'";

Future<void> pumpLogin(
  WidgetTester tester, {
  required LoginResult result,
}) async {
  final presenter = AuthPresenter(
    loginUseCase: ({required email, required password}) async => result,
    restoreSessionUseCase: () async => RestoreSessionOutcome.notAvailable,
  );

  await tester.pumpWidget(
    MaterialApp(
      home: Scaffold(
        body: BlocSignalProvider<AuthPresenter>.value(
          value: presenter,
          child: const LoginScreen(),
        ),
      ),
    ),
  );

  await tester.enterText(find.byType(TextField).first, 'user@example.com');
  await tester.enterText(find.byType(TextField).last, 'hunter2-secret');
  await tester.tap(find.text('Entrar'));
  await tester.pumpAndSettle();
}

void main() {
  group('LoginScreen failure', () {
    testWidgets(
      'shows the friendly message with the technical details collapsed',
      (tester) async {
        await pumpLogin(
          tester,
          result: const LoginResult.unableToPerformAuthorization(
            ErrorReportDto(kind: ErrorKindDto.network, details: _details),
          ),
        );

        expect(
          find.textContaining('Não foi possível conectar ao SAM'),
          findsOneWidget,
        );
        expect(find.text('Detalhes técnicos'), findsOneWidget);
        expect(find.text(_details), findsNothing);
      },
    );

    testWidgets('expanding reveals the details without exposing the password', (
      tester,
    ) async {
      await pumpLogin(
        tester,
        result: const LoginResult.unableToPerformAuthorization(
          ErrorReportDto(kind: ErrorKindDto.network, details: _details),
        ),
      );

      await tester.tap(find.text('Detalhes técnicos'));
      await tester.pumpAndSettle();

      expect(find.text(_details), findsOneWidget);
      expect(
        find.descendant(
          of: find.byType(TechnicalDetails),
          matching: find.textContaining('hunter2-secret'),
        ),
        findsNothing,
      );
    });

    testWidgets('invalid credentials show no technical details', (
      tester,
    ) async {
      await pumpLogin(
        tester,
        result: const LoginResult.invalidEmailOrPassword(),
      );

      expect(find.text('Usuário ou senha inválido(a)'), findsOneWidget);
      expect(find.text('Detalhes técnicos'), findsNothing);
    });
  });
}
