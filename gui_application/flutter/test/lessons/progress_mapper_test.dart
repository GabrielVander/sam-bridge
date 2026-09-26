import 'package:flutter_application/lessons/progress_mapper.dart';
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
      expect(view.checkpoints[0].label, 'Candidato(a)');
      expect(view.checkpoints[0].achieved, isTrue);
      expect(view.checkpoints[1].label, 'Reunião de Jovens e Menores');
      expect(view.checkpoints[1].readyToAdvance, isTrue);
      expect(view.msaRelativePercent, 100);
      expect(view.methodRelativePercent, 85);
      expect(view.combinedPercent, 92.5);
      expect(view.overallCheckpointPercent, 40);
      expect(view.nextLevelLabel, 'Reunião de Jovens e Menores');
    });

    test('no next level means every checkpoint was achieved', () {
      final dto = progressAssessment(
        msaRelativePercent: 100,
        methodRelativePercent: 100,
        combinedPercent: 100,
        overallCheckpointPercent: 100,
      );

      final view = ProgressMapper.toViewModel(dto);

      expect(view.nextLevelLabel, isNull);
    });

    test('an unrecognized level falls back to what SAM wrote', () {
      expect(
        ProgressMapper.levelLabel(Levels.unknown('SomethingNew')),
        'SomethingNew',
      );
    });

    test('every known level has a confirmed Portuguese label', () {
      expect(ProgressMapper.levelLabel(Levels.candidate), 'Candidato(a)');
      expect(ProgressMapper.levelLabel(Levels.practice), 'Ensaio');
      expect(
        ProgressMapper.levelLabel(Levels.youthService),
        'Reunião de Jovens e Menores',
      );
      expect(
        ProgressMapper.levelLabel(Levels.officialService),
        'Culto Oficial',
      );
      expect(ProgressMapper.levelLabel(Levels.officialized), 'Oficialização');
    });
  });
}
