import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_test/flutter_test.dart';

import 'support/builders.dart';

/// Every model is compared by value: presenters emit states and tests (and the
/// UI's rebuild logic) rely on two equal models being interchangeable.
void expectValueEquality<T>({
  required T Function() build,
  required Map<String, T Function()> variants,
}) {
  test('is equal, with the same hash, when every field is equal', () {
    expect(build(), build());
    expect(build().hashCode, build().hashCode);
  });

  variants.forEach((field, variant) {
    test('differs when only $field differs', () {
      expect(build(), isNot(variant()));
    });
  });
}

void main() {
  group('StudentListItem', () {
    StudentListItem item({String? instrument}) => StudentListItem(
      id: '1',
      name: 'Jane Doe',
      location: 'Some Location',
      position: 'Ensaio',
      instrument: instrument,
    );

    test('is equal when every field is equal, instrument included', () {
      expect(item(instrument: 'Violino'), item(instrument: 'Violino'));
      expect(
        item(instrument: 'Violino').hashCode,
        item(instrument: 'Violino').hashCode,
      );
      expect(item(), item());
    });

    test('differs when only the instrument differs', () {
      expect(item(instrument: 'Violino'), isNot(item(instrument: 'Viola')));
      expect(item(instrument: 'Violino'), isNot(item()));
    });
  });

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
        'levelKey': () => checkpoint(levelKey: 'culto'),
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

  group('ErrorReport', () {
    expectValueEquality<ErrorReport>(
      build: () => const ErrorReport(userMessage: 'Falhou', details: 'detalhe'),
      variants: {
        'userMessage': () =>
            const ErrorReport(userMessage: 'Outra', details: 'detalhe'),
        'details': () =>
            const ErrorReport(userMessage: 'Falhou', details: 'outro'),
      },
    );
  });
}
