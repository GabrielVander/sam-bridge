import 'package:flutter/semantics.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/app.dart';
import '../support/authentication.dart';

Future<void> openSettings(WidgetTester tester) async {
  await tester.tap(find.byTooltip('Configurações'));
  await tester.pumpAndSettle();
}

SemanticsNode option(WidgetTester tester, String language) =>
    tester.getSemantics(find.text(language));

final Matcher chosen = isSemantics(isChecked: true);
final Matcher notChosen = isSemantics(isChecked: false);

void main() {
  testWidgets('can be opened before signing in', (tester) async {
    await pumpApp(tester, await composeFakeApp());

    await openSettings(tester);

    expect(find.text('Idioma'), findsOneWidget);
  });

  testWidgets('returns to where the user was', (tester) async {
    await pumpApp(
      tester,
      await composeFakeApp(restoreSession: sessionRestored()),
    );
    await tester.tap(find.text('Jane Doe'));
    await tester.pumpAndSettle();

    await openSettings(tester);
    await tester.tap(find.byTooltip('Voltar'));
    await tester.pumpAndSettle();

    expect(find.text('Idioma'), findsNothing);
    expect(find.text('Lista de alunos'), findsOneWidget);
  });

  testWidgets('is not offered again while it is open', (tester) async {
    await pumpApp(tester, await composeFakeApp());

    await openSettings(tester);

    expect(find.byTooltip('Configurações'), findsNothing);
  });

  testWidgets('switches the app to the language the user picks', (
    tester,
  ) async {
    await pumpApp(tester, await composeFakeApp());
    await openSettings(tester);

    await tester.tap(find.text('English'));
    await tester.pumpAndSettle();

    expect(find.text('Settings'), findsOneWidget);
    expect(find.text('Configurações'), findsNothing);
  });

  testWidgets('marks the language in use, starting with the system one', (
    tester,
  ) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    await pumpApp(tester, await composeFakeApp());
    await openSettings(tester);

    expect(option(tester, 'Idioma do sistema'), chosen);
    expect(option(tester, 'English'), notChosen);

    await tester.tap(find.text('English'));
    await tester.pumpAndSettle();

    expect(option(tester, 'System language'), notChosen);
    expect(option(tester, 'English'), chosen);
    semantics.dispose();
  });

  testWidgets('is where the user signs out, returning to the login form', (
    tester,
  ) async {
    await pumpApp(
      tester,
      await composeFakeApp(restoreSession: sessionRestored()),
    );
    await openSettings(tester);

    await tester.tap(find.text('Sair'));
    await tester.pumpAndSettle();

    expect(find.text('Entre com seu usuário SAM'), findsOneWidget);
    expect(find.text('Idioma'), findsNothing);
  });

  testWidgets('does not offer signing out before signing in', (tester) async {
    await pumpApp(tester, await composeFakeApp());

    await openSettings(tester);

    expect(find.text('Sair'), findsNothing);
  });

  testWidgets('is where the app version is shown', (tester) async {
    await pumpApp(tester, await composeFakeApp(versionDisplay: 'v2.3.4+56'));
    expect(find.text('v2.3.4+56'), findsNothing);

    await openSettings(tester);

    expect(find.text('Versão'), findsOneWidget);
    expect(find.text('v2.3.4+56'), findsOneWidget);
  });
}
