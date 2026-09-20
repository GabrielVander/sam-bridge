import 'package:flutter/material.dart';
import 'package:flutter_application/widgets/loading_indicator.dart';
import 'package:flutter_test/flutter_test.dart';

const _slowNotice = 'O SAM está demorando para responder';

Future<void> pumpIndicator(WidgetTester tester) => tester.pumpWidget(
  const MaterialApp(home: Scaffold(body: LoadingIndicator())),
);

void main() {
  group('LoadingIndicator', () {
    testWidgets('shows a spinner and no message at first', (tester) async {
      await pumpIndicator(tester);

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.textContaining(_slowNotice), findsNothing);
    });

    testWidgets('stays quiet for the first ten seconds', (tester) async {
      await pumpIndicator(tester);

      await tester.pump(const Duration(seconds: 9, milliseconds: 999));

      expect(find.textContaining(_slowNotice), findsNothing);
    });

    testWidgets('explains the wait after ten seconds, keeping the spinner', (
      tester,
    ) async {
      await pumpIndicator(tester);

      await tester.pump(const Duration(seconds: 10));

      expect(find.textContaining(_slowNotice), findsOneWidget);
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('leaves no timer behind when removed before it fires', (
      tester,
    ) async {
      await pumpIndicator(tester);

      await tester.pumpWidget(const SizedBox.shrink());

      // A timer still pending here would fail the test when it ends.
    });
  });
}
