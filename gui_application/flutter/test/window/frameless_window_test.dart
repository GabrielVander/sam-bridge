import 'package:flutter/gestures.dart';
import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/startup_failure_app.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/app.dart';
import '../support/localization.dart';
import '../support/window.dart';

Future<void> pumpAppIn(WidgetTester tester, FakeWindow? window) async {
  setOsLocale(tester, brazilianPortuguese);
  await tester.pumpWidget(
    await composeFakeApp(windowControls: window?.controls),
  );
  await tester.pumpAndSettle();
}

void main() {
  group('a window without the platform frame', () {
    testWidgets('is minimised, maximised and closed from the title bar', (
      tester,
    ) async {
      final window = FakeWindow();
      await pumpAppIn(tester, window);

      await tester.tap(find.byTooltip('Minimizar'));
      await tester.tap(find.byTooltip('Maximizar'));
      await tester.tap(find.byTooltip('Fechar'));

      expect(window.requests, ['minimize', 'toggle maximize', 'close']);
    });

    testWidgets('shows the app title in the middle of the title bar', (
      tester,
    ) async {
      await pumpAppIn(tester, FakeWindow());

      final double windowWidth =
          tester.view.physicalSize.width / tester.view.devicePixelRatio;
      expect(tester.getCenter(find.text('SAM Bridge')).dx, windowWidth / 2);
    });

    testWidgets('moves when the title bar is dragged', (tester) async {
      final window = FakeWindow();
      await pumpAppIn(tester, window);

      await tester.drag(find.text('SAM Bridge'), const Offset(40, 20));
      await tester.pump(kDoubleTapTimeout);

      expect(window.requests, ['start dragging']);
    });

    testWidgets('maximises or restores when the title bar is double-clicked', (
      tester,
    ) async {
      final window = FakeWindow();
      await pumpAppIn(tester, window);

      await tester.tap(find.text('SAM Bridge'));
      await tester.pump(const Duration(milliseconds: 50));
      await tester.tap(find.text('SAM Bridge'));
      await tester.pumpAndSettle();

      expect(window.requests, ['toggle maximize']);
    });

    testWidgets('can still be closed when the app cannot start', (
      tester,
    ) async {
      final window = FakeWindow();
      setOsLocale(tester, brazilianPortuguese);
      await tester.pumpWidget(
        StartupFailureApp(
          report: const ErrorReport(reason: ErrorReason.generic, details: ''),
          onRetry: () {},
          windowControls: window.controls,
        ),
      );
      await tester.pumpAndSettle();

      await tester.tap(find.byTooltip('Fechar'));

      expect(window.requests, ['close']);
    });
  });

  testWidgets('a window framed by the platform offers no window buttons', (
    tester,
  ) async {
    await pumpAppIn(tester, null);

    expect(find.byTooltip('Fechar'), findsNothing);
    expect(find.byTooltip('Minimizar'), findsNothing);
  });
}
