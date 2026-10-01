import 'package:flutter_test/flutter_test.dart';

import '../support/app.dart';

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
}
