import 'package:flutter/material.dart';
import 'package:flutter_application/main.dart' show formatVersion;
import 'package:flutter_application/main_screen.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';

import 'support/app.dart';
import 'support/authentication.dart';
import 'support/errors.dart';

Finder get _emailField => find.widgetWithText(TextField, 'Email');
Finder get _passwordField => find.widgetWithText(TextField, 'Senha');

Future<void> signIn(WidgetTester tester) async {
  await tester.enterText(_emailField, 'user@example.com');
  await tester.enterText(_passwordField, 'hunter2');
  await tester.tap(find.text('Entrar'));
  await tester.pumpAndSettle();
}

GoRouter routerOf(WidgetTester tester) =>
    GoRouter.of(tester.element(find.byType(MainScreen)));

void main() {
  group('starting the app', () {
    testWidgets('shows the login form when there is no saved session', (
      tester,
    ) async {
      await pumpApp(tester, await composeFakeApp());

      expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
      expect(find.text('Jane Doe'), findsNothing);
    });

    testWidgets('goes straight to the students when the session is restored', (
      tester,
    ) async {
      await pumpApp(
        tester,
        await composeFakeApp(restoreSession: sessionRestored()),
      );

      expect(find.text('Jane Doe'), findsOneWidget);
      expect(find.text('Entre com seu usuário SAM'), findsNothing);
    });

    testWidgets('shows the app name', (tester) async {
      await pumpApp(tester, await composeFakeApp());

      expect(find.text('SAM Bridge'), findsOneWidget);
    });

    testWidgets('uses the dark theme', (tester) async {
      await pumpApp(tester, await composeFakeApp());

      final theme = Theme.of(tester.element(find.byType(MainScreen)));
      expect(theme.brightness, Brightness.dark);
    });
  });

  group('language', () {
    testWidgets('speaks Portuguese when the OS is in Brazilian Portuguese', (
      tester,
    ) async {
      await pumpApp(tester, await composeFakeApp());

      expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
    });

    testWidgets('speaks English when the OS is in English', (tester) async {
      await pumpApp(
        tester,
        await composeFakeApp(),
        osLocale: const Locale('en', 'US'),
      );

      expect(find.text('Sign in with your SAM account'), findsOneWidget);
    });

    testWidgets('speaks Portuguese for any Portuguese-speaking OS', (
      tester,
    ) async {
      await pumpApp(
        tester,
        await composeFakeApp(),
        osLocale: const Locale('pt', 'PT'),
      );

      expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
    });

    testWidgets('speaks the language the user chose over the OS one', (
      tester,
    ) async {
      await pumpApp(tester, await composeFakeApp(rememberedLanguage: 'en'));

      expect(find.text('Sign in with your SAM account'), findsOneWidget);
    });

    testWidgets('falls back to English for a language it does not speak', (
      tester,
    ) async {
      await pumpApp(
        tester,
        await composeFakeApp(),
        osLocale: const Locale('fr', 'FR'),
      );

      expect(find.text('Sign in with your SAM account'), findsOneWidget);
    });
  });

  group('signing in', () {
    testWidgets('leads to the list of students', (tester) async {
      await pumpApp(tester, await composeFakeApp());

      await signIn(tester);

      expect(find.text('Jane Doe'), findsOneWidget);
      expect(find.text('Entre com seu usuário SAM'), findsNothing);
    });

    testWidgets('stays on the form when the credentials are rejected', (
      tester,
    ) async {
      await pumpApp(tester, await composeFakeApp(login: loginRejected()));

      await signIn(tester);

      expect(find.text('Usuário ou senha inválido(a)'), findsOneWidget);
      expect(find.text('Jane Doe'), findsNothing);
    });
  });

  group('access control', () {
    testWidgets('signed-out visitors are sent to the login form', (
      tester,
    ) async {
      await pumpApp(tester, await composeFakeApp());

      routerOf(tester).go('/students');
      await tester.pumpAndSettle();

      expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
    });

    testWidgets('signed-in users are kept away from the login form', (
      tester,
    ) async {
      await pumpApp(
        tester,
        await composeFakeApp(restoreSession: sessionRestored()),
      );

      routerOf(tester).go('/login');
      await tester.pumpAndSettle();

      expect(find.text('Entre com seu usuário SAM'), findsNothing);
      expect(find.text('Jane Doe'), findsOneWidget);
    });
  });

  group('signing out', () {
    testWidgets('is not offered on the login form', (tester) async {
      await pumpApp(tester, await composeFakeApp());

      expect(find.byTooltip('Sair'), findsNothing);
    });

    testWidgets('returns to the login form', (tester) async {
      await pumpApp(
        tester,
        await composeFakeApp(restoreSession: sessionRestored()),
      );
      expect(find.text('Jane Doe'), findsOneWidget);

      await tester.tap(find.byTooltip('Sair'));
      await tester.pumpAndSettle();

      expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
      expect(find.text('Jane Doe'), findsNothing);
    });

    testWidgets(
      'tells the user when the saved credentials could not be removed',
      (tester) async {
        await pumpApp(
          tester,
          await composeFakeApp(
            restoreSession: sessionRestored(),
            logout: logoutFailed(
              localStorageFailure(
                'Unable to remove the credential file: Permission denied',
              ),
            ),
          ),
        );

        await tester.tap(find.byTooltip('Sair'));
        await tester.pumpAndSettle();
        await tester.tap(find.text('Detalhes técnicos'));
        await tester.pumpAndSettle();

        expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
        expect(
          find.text(
            'Não foi possível acessar os dados salvos neste dispositivo.',
          ),
          findsOneWidget,
        );
        expect(
          find.text('Unable to remove the credential file: Permission denied'),
          findsOneWidget,
        );
      },
    );
  });

  group('opening a student', () {
    testWidgets('shows their page and can return to the list', (tester) async {
      await pumpApp(
        tester,
        await composeFakeApp(restoreSession: sessionRestored()),
      );

      await tester.tap(find.text('Jane Doe'));
      await tester.pumpAndSettle();

      expect(find.text('Lista de alunos'), findsOneWidget);
      expect(find.text('Jane Doe'), findsOneWidget);
      expect(find.byTooltip('Voltar'), findsOneWidget);

      await tester.tap(find.byTooltip('Voltar'));
      await tester.pumpAndSettle();

      expect(find.byTooltip('Voltar'), findsNothing);
      expect(find.text('Jane Doe'), findsOneWidget);
    });
  });

  group('formatVersion', () {
    test('joins the version and build number', () {
      expect(formatVersion(version: '1.2.3', buildNumber: '45'), 'v1.2.3+45');
    });
  });
}
