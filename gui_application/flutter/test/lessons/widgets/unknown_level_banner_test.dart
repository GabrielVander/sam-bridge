import 'package:flutter_application/lessons/widgets/unknown_level_banner.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../support/builders.dart';

void main() {
  group('UnknownLevelBanner', () {
    testWidgets('explains that progress cannot be calculated', (tester) async {
      await pumpInApp(tester, const UnknownLevelBanner(raw: 'EXÓTICO'));

      expect(find.text('nível não reconhecido'), findsOneWidget);
      expect(
        find.text('O progresso não pode ser calculado para este nível.'),
        findsOneWidget,
      );
    });

    testWidgets('shows the unrecognised value when there is one', (
      tester,
    ) async {
      await pumpInApp(tester, const UnknownLevelBanner(raw: 'EXÓTICO'));

      expect(find.text('Valor: EXÓTICO'), findsOneWidget);
    });

    testWidgets('omits the value line when the raw value is empty', (
      tester,
    ) async {
      await pumpInApp(tester, const UnknownLevelBanner(raw: ''));

      expect(find.textContaining('Valor:'), findsNothing);
    });
  });
}
