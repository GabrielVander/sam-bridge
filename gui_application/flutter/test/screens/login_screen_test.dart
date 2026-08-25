import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/authentication/login_screen.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

Widget harness(FakeSamPortal portal) {
  final presenter = AuthCubitSignal(portal);
  return BlocSignalProvider<AuthCubitSignal>.value(
    value: presenter,
    child: MaterialApp.router(
      routerConfig: GoRouter(
        initialLocation: '/login',
        routes: [
          GoRoute(path: '/login', builder: (_, _) => const LoginScreen()),
          GoRoute(path: '/students', builder: (_, _) => const Scaffold(body: Text('STUDENTS'))),
        ],
      ),
    ),
  );
}

void main() {
  testWidgets('renders username, password and login button', (tester) async {
    await tester.pumpWidget(harness(FakeSamPortal()));

    expect(find.text('Username'), findsOneWidget);
    expect(find.text('Password'), findsOneWidget);
    expect(find.text('Login'), findsOneWidget);
  });

  testWidgets('submitting empty fields shows the validation message', (tester) async {
    await tester.pumpWidget(harness(FakeSamPortal()));

    await tester.tap(find.text('Login'));
    await tester.pump();

    expect(find.textContaining('Informe'), findsOneWidget);
  });

  testWidgets('successful login calls the portal with typed credentials', (tester) async {
    final portal = FakeSamPortal();
    await tester.pumpWidget(harness(portal));

    await tester.enterText(find.widgetWithText(TextField, 'Username'), 'gabriel');
    await tester.enterText(find.widgetWithText(TextField, 'Password'), 'secret');
    await tester.tap(find.text('Login'));
    await tester.pump();

    expect(portal.loginCalls.single, ('gabriel', 'secret'));
    expect(find.byIcon(Icons.check_rounded), findsOneWidget,
        reason: 'Success state renders the check icon');
  });
}
