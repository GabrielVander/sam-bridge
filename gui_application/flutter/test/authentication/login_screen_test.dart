import 'dart:async';

import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/authentication/login_screen.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';

const _details = "Request failed for operation 'authentication'";

Finder get _emailField => find.widgetWithText(TextField, 'Email');
Finder get _passwordField => find.widgetWithText(TextField, 'Senha');

Future<List<(String, String)>> pumpLoginForm(
  WidgetTester tester, {
  LoginResult result = const LoginResult.invalidEmailOrPassword(),
}) async {
  final attempts = <(String, String)>[];
  final presenter = AuthPresenter(
    loginUseCase: ({required email, required password}) async {
      attempts.add((email, password));
      return result;
    },
    restoreSessionUseCase: () async => RestoreSessionOutcome.notAvailable,
  );
  final router = GoRouter(
    initialLocation: '/login',
    routes: [
      GoRoute(
        path: '/login',
        builder: (_, _) => const Scaffold(body: LoginScreen()),
      ),
      GoRoute(
        path: '/students',
        builder: (_, _) => const Scaffold(body: Text('lista de alunos aberta')),
      ),
    ],
  );

  await tester.pumpWidget(
    BlocSignalProvider<AuthPresenter>.value(
      value: presenter,
      child: MaterialApp.router(routerConfig: router),
    ),
  );
  return attempts;
}

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

  await tester.enterText(_emailField, 'user@example.com');
  await tester.enterText(_passwordField, 'hunter2-secret');
  await tester.tap(find.text('Entrar'));
  await tester.pumpAndSettle();
}

void main() {
  group('LoginScreen loading', () {
    testWidgets('says SAM is slow when signing in takes over ten seconds', (
      tester,
    ) async {
      final answer = Completer<LoginResult>();
      final presenter = AuthPresenter(
        loginUseCase: ({required email, required password}) => answer.future,
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
      await tester.enterText(_emailField, 'user@example.com');
      await tester.enterText(_passwordField, 'hunter2-secret');
      await tester.tap(find.text('Entrar'));
      await tester.pump();
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.textContaining('O SAM está demorando'), findsNothing);

      await tester.pump(const Duration(seconds: 10));

      expect(find.textContaining('O SAM está demorando'), findsOneWidget);
    });
  });

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
  group('LoginScreen form', () {
    testWidgets('signing in successfully opens the list of students', (
      tester,
    ) async {
      final attempts = await pumpLoginForm(
        tester,
        result: const LoginResult.successful(),
      );

      await tester.enterText(_emailField, 'user@example.com');
      await tester.enterText(_passwordField, 'hunter2');
      await tester.tap(find.text('Entrar'));
      await tester.pumpAndSettle();

      expect(attempts, [('user@example.com', 'hunter2')]);
      expect(find.text('lista de alunos aberta'), findsOneWidget);
    });

    testWidgets('pressing done on the password field signs in', (tester) async {
      final attempts = await pumpLoginForm(tester);

      await tester.enterText(_emailField, 'user@example.com');
      await tester.enterText(_passwordField, 'hunter2');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      expect(attempts, [('user@example.com', 'hunter2')]);
    });

    testWidgets('asks for both fields when either is empty', (tester) async {
      final attempts = await pumpLoginForm(tester);

      await tester.enterText(_emailField, 'user@example.com');
      await tester.tap(find.text('Entrar'));
      await tester.pumpAndSettle();

      expect(find.text('Informe usuário e senha'), findsOneWidget);
      expect(attempts, isEmpty);
    });

    testWidgets('keeps the password hidden until asked to show it', (
      tester,
    ) async {
      await pumpLoginForm(tester);
      bool hidden() => tester.widget<TextField>(_passwordField).obscureText;
      expect(hidden(), isTrue);

      await tester.tap(find.byTooltip('Mostrar senha'));
      await tester.pumpAndSettle();
      expect(hidden(), isFalse);

      await tester.tap(find.byTooltip('Ocultar senha'));
      await tester.pumpAndSettle();
      expect(hidden(), isTrue);
    });
  });
}
