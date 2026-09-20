import 'package:flutter_application/widgets/info_chip.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/builders.dart';

void main() {
  testWidgets('InfoChip shows its label', (tester) async {
    await pumpInApp(tester, const InfoChip(label: 'Fase 2'));

    expect(find.text('Fase 2'), findsOneWidget);
  });
}
