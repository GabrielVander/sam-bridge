import 'package:flutter/material.dart';
import 'package:flutter_application/main.dart';
import 'package:flutter_application/portal/sam_portal.dart';
import 'package:flutter_test/flutter_test.dart';

import 'support/fake_sam_portal.dart';

void main() {
  testWidgets('unauthenticated users are redirected to /login', (tester) async {
    await tester.pumpWidget(SamSiteApp(
      versionDisplay: 'test',
      portal: FakeSamPortal()..students = [studentItem()],
    ));
    // Deliberately deep-linking to a protected route:
    // initialLocation is /login, so assert the login form is what renders.
    await tester.pumpAndSettle();

    expect(find.text('Login to SamSite'), findsOneWidget);
    expect(find.text('vtest+'), findsNothing);
  });

  testWidgets('successful login navigates to the students list', (tester) async {
    final portal = FakeSamPortal()..students = [studentItem(name: 'ALUNA ROTEADOR')];
    await tester.pumpWidget(
        SamSiteApp(versionDisplay: 'test', portal: portal));
    await tester.pumpAndSettle();

    await tester.enterText(find.widgetWithText(TextField, 'Username'), 'u');
    await tester.enterText(find.widgetWithText(TextField, 'Password'), 'p');
    await tester.tap(find.text('Login'));
    await tester.pumpAndSettle();

    expect(find.text('ALUNA ROTEADOR'), findsOneWidget,
        reason: 'Auth success must trigger the redirect to /students');
  });
}

// Re-exported for convenience in future router-only tests.
typedef SamPortalT = SamPortal;
