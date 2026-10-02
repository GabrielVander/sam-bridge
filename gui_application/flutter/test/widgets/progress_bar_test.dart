import 'package:flutter/material.dart';
import 'package:flutter_application/widgets/progress_bar.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/builders.dart';

void main() {
  group('ProgressBar', () {
    testWidgets(
      'shows its label and the percentage rounded to a whole number',
      (tester) async {
        await pumpInApp(tester, const ProgressBar(label: 'MSA', percent: 45.6));

        expect(find.text('MSA'), findsOneWidget);
        expect(find.text('46%'), findsOneWidget);
      },
    );

    testWidgets('fills the bar in proportion to the percentage', (
      tester,
    ) async {
      await pumpInApp(tester, const ProgressBar(label: 'MSA', percent: 25));

      expect(shownFraction(tester), 0.25);
    });
  });
}

double? shownFraction(WidgetTester tester) => tester
    .widget<LinearProgressIndicator>(find.byType(LinearProgressIndicator))
    .value;
