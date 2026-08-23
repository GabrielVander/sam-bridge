import 'package:flutter_application/slices/lessons/lessons_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

void main() {
  test('starts idle', () {
    final presenter = LessonsCubitSignal(FakeSamPortal());
    expect(presenter.stateValue, isA<LessonsIdle>());
  });

  test('given a lessons view should expose it loaded', () async {
    final portal = FakeSamPortal()
      ..lessons = StudentLessonsViewT(
        msa: [
          lessonItem(id: '12', date: '2025-06-02'),
          lessonItem(id: '11', date: '2025-06-01'),
        ],
      ).view;
    final presenter = LessonsCubitSignal(portal);

    await presenter.load('7');

    final state = presenter.stateValue;
    expect(state, isA<LessonsLoaded>());
    final view = (state as LessonsLoaded).view;
    expect(view.msa.first.id, '12', reason: 'Rust slice guarantees newest first');
    expect(view.msa.last.id, '11');
  });

  test('given portal failure should surface failure state', () async {
    final portal = FakeSamPortal()..lessonsError = Exception('Not authenticated');
    final presenter = LessonsCubitSignal(portal);

    await presenter.load('7');

    final state = presenter.stateValue;
    expect(state, isA<LessonsFailure>());
    expect((state as LessonsFailure).message, contains('Not authenticated'));
  });
}
