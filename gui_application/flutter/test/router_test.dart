import 'dart:async';

import 'package:flutter_application/router.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('PresenterRefreshListenable', () {
    test(
      'tells the router to re-check access whenever the presenter changes',
      () async {
        final states = StreamController<int>.broadcast();
        final listenable = PresenterRefreshListenable(states.stream);
        var notifications = 0;
        listenable.addListener(() => notifications++);

        states.add(1);
        states.add(2);
        await Future<void>.delayed(Duration.zero);

        expect(notifications, 2);
        listenable.dispose();
        await states.close();
      },
    );

    test('stops following the presenter once disposed', () async {
      final states = StreamController<int>.broadcast();
      final listenable = PresenterRefreshListenable(states.stream);
      expect(states.hasListener, isTrue);

      listenable.dispose();
      await Future<void>.delayed(Duration.zero);

      expect(states.hasListener, isFalse);
      await states.close();
    });
  });
}
