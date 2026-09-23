import 'package:flutter/material.dart';
import 'package:flutter_application/app.dart';
import 'package:flutter_application/main.dart' show formatVersion;
import 'package:flutter_application/router.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';

import 'support/app.dart';

Future<void> pumpApp(WidgetTester tester, SamSiteApp app) async {
  await tester.pumpWidget(app);
  await tester.pumpAndSettle();
}

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
        await composeFakeApp(
          restoreSession: const RestoreSessionOutcome.restored(),
        ),
      );

      expect(find.text('Jane Doe'), findsOneWidget);
      expect(find.text('Entre com seu usuário SAM'), findsNothing);
    });

    testWidgets('shows the app name and the version on every screen', (
      tester,
    ) async {
      await pumpApp(tester, await composeFakeApp(versionDisplay: 'v2.3.4+56'));

      expect(find.text('SAM Bridge'), findsOneWidget);
      expect(find.text('v2.3.4+56'), findsOneWidget);
    });

    testWidgets('uses the dark theme', (tester) async {
      await pumpApp(tester, await composeFakeApp());

      final theme = Theme.of(tester.element(find.byType(MainScreen)));
      expect(theme.brightness, Brightness.dark);
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
      await pumpApp(
        tester,
        await composeFakeApp(login: const LoginResult.invalidEmailOrPassword()),
      );

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
        await composeFakeApp(
          restoreSession: const RestoreSessionOutcome.restored(),
        ),
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

      expect(find.byTooltip('Log out'), findsNothing);
    });

    testWidgets('returns to the login form', (tester) async {
      await pumpApp(
        tester,
        await composeFakeApp(
          restoreSession: const RestoreSessionOutcome.restored(),
        ),
      );
      expect(find.text('Jane Doe'), findsOneWidget);

      await tester.tap(find.byTooltip('Log out'));
      await tester.pumpAndSettle();

      expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
      expect(find.text('Jane Doe'), findsNothing);
    });
  });

  group('opening a student', () {
    testWidgets('shows their page and can return to the list', (tester) async {
      await pumpApp(
        tester,
        await composeFakeApp(
          restoreSession: const RestoreSessionOutcome.restored(),
        ),
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
