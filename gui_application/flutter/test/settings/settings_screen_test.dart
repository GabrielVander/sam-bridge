import 'package:flutter_test/flutter_test.dart';

import '../support/app.dart';
import '../support/authentication.dart';

Future<void> openSettings(WidgetTester tester) async {
  await tester.tap(find.byTooltip('Configurações'));
  await tester.pumpAndSettle();
}

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
}
