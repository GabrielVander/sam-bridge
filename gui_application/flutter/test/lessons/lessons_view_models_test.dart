import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/builders.dart';
import '../support/value_equality.dart';

void main() {
  group('LessonItem', () {
    expectValueEquality<LessonItem>(
      build: lessonItem,
      variants: {
        'kind': () => lessonItem(kind: LessonKind.method),
        'id': () => lessonItem(id: '2'),
        'date': () => lessonItem(date: '02/02/2024'),
        'phase': () => lessonItem(phase: '9'),
        'page': () => lessonItem(page: '99'),
        'lesson': () => lessonItem(lesson: '9'),
        'clef': () => lessonItem(clef: 'Fá'),
        'description': () => lessonItem(description: 'Outra'),
        'instructor': () => lessonItem(instructor: 'Outro'),
        'method': () => lessonItem(method: 'Método B'),
      },
    );
  });
  group('StudentLessonsView', () {
    StudentLessonsView view({
      List<LessonItem>? msa,
      List<LessonItem>? method,
    }) => StudentLessonsView(
      msa: msa ?? [lessonItem(id: '1')],
      method: method ?? [lessonItem(id: '2')],
    );

    expectValueEquality<StudentLessonsView>(
      build: view,
      variants: {
        'the MSA lessons': () => view(msa: [lessonItem(id: '3')]),
        'the method lessons': () => view(method: [lessonItem(id: '3')]),
      },
    );

    test('empty has no lessons of either kind', () {
      final empty = StudentLessonsView.empty();

      expect(empty.msa, isEmpty);
      expect(empty.method, isEmpty);
      expect(empty, StudentLessonsView(msa: const [], method: const []));
    });
  });
  group('CheckpointView', () {
    expectValueEquality<CheckpointView>(
      build: checkpoint,
      variants: {
        'label': () => checkpoint(label: 'Culto'),
        'achieved': () => checkpoint(achieved: true),
        'readyToAdvance': () => checkpoint(readyToAdvance: true),
        'msaMet': () => checkpoint(msaMet: true),
        'methodMet': () => checkpoint(methodMet: true),
      },
    );
  });
  group('ProgressView', () {
    expectValueEquality<ProgressView>(
      build: progressView,
      variants: {
        'the checkpoints': () =>
            progressView(checkpoints: [checkpoint(label: 'Culto')]),
        'msaRelativePercent': () => progressView(msaRelativePercent: 41),
        'methodRelativePercent': () => progressView(methodRelativePercent: 76),
        'combinedPercent': () => progressView(combinedPercent: 51),
        'overallCheckpointPercent': () =>
            progressView(overallCheckpointPercent: 26),
        'nextLevelLabel': () => progressView(nextLevelLabel: null),
      },
    );
  });
}
