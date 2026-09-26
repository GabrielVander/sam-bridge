import 'package:flutter_application/lessons/progress_mapper.dart';
import 'package:flutter_application/shared/level.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/lessons.dart';

void main() {
  group('ProgressMapper', () {
    test('maps checkpoints and percentages', () {
      final dto = progressAssessment(
        checkpoints: [
          checkpointStatus(Levels.candidate, achieved: true),
          checkpointStatus(Levels.youthService, readyToAdvance: true),
        ],
        msaRelativePercent: 100,
        methodRelativePercent: 85,
        combinedPercent: 92.5,
        overallCheckpointPercent: 40,
        nextLevel: Levels.youthService,
      );

      final view = ProgressMapper.toViewModel(dto);

      expect(view.checkpoints, hasLength(2));
      expect(view.checkpoints[0].level, const KnownLevel(Level.candidate));
      expect(view.checkpoints[0].achieved, isTrue);
      expect(view.checkpoints[1].level, const KnownLevel(Level.youthService));
      expect(view.checkpoints[1].readyToAdvance, isTrue);
      expect(view.msaRelativePercent, 100);
      expect(view.methodRelativePercent, 85);
      expect(view.combinedPercent, 92.5);
      expect(view.overallCheckpointPercent, 40);
      expect(view.nextLevel, const KnownLevel(Level.youthService));
    });

    test('no next level means every checkpoint was achieved', () {
      final dto = progressAssessment(
        msaRelativePercent: 100,
        methodRelativePercent: 100,
        combinedPercent: 100,
        overallCheckpointPercent: 100,
      );

      final view = ProgressMapper.toViewModel(dto);

      expect(view.nextLevel, isNull);
    });

    final levels = {
      Levels.candidate: const KnownLevel(Level.candidate),
      Levels.practice: const KnownLevel(Level.practice),
      Levels.youthService: const KnownLevel(Level.youthService),
      Levels.officialService: const KnownLevel(Level.officialService),
      Levels.officialized: const KnownLevel(Level.officialized),
      Levels.unknown('SomethingNew'): const UnrecognizedLevel('SomethingNew'),
    };

    for (final MapEntry(key: dto, value: expected) in levels.entries) {
      test('maps the $dto level to $expected', () {
        final view = ProgressMapper.toViewModel(
          progressAssessment(checkpoints: [checkpointStatus(dto)]),
        );

        expect(view.checkpoints.single.level, expected);
      });
    }
  });
}
