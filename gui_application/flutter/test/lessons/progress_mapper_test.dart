import 'package:flutter_application/lessons/progress_mapper.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ProgressMapper', () {
    test('maps checkpoints and percentages', () {
      const dto = ProgressAssessmentDto(
        checkpoints: [
          CheckpointStatusDto(
            level: 'Candidate',
            achieved: true,
            readyToAdvance: false,
            requirement: RequirementStatusDto(msaMet: true, methodMet: true),
          ),
          CheckpointStatusDto(
            level: 'YouthService',
            achieved: false,
            readyToAdvance: true,
            requirement: RequirementStatusDto(msaMet: true, methodMet: true),
          ),
        ],
        msaRelativePercent: 100,
        methodRelativePercent: 85,
        combinedPercent: 92.5,
        overallCheckpointPercent: 40,
        nextLevel: 'YouthService',
      );

      final view = ProgressMapper.toViewModel(dto);

      expect(view.checkpoints, hasLength(2));
      expect(view.checkpoints[0].levelKey, 'Candidate');
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
      const dto = ProgressAssessmentDto(
        checkpoints: [],
        msaRelativePercent: 100,
        methodRelativePercent: 100,
        combinedPercent: 100,
        overallCheckpointPercent: 100,
      );

      final view = ProgressMapper.toViewModel(dto);

      expect(view.nextLevelLabel, isNull);
    });

    test('unrecognized level keys fall back to the raw value', () {
      expect(ProgressMapper.levelLabel('SomethingNew'), 'SomethingNew');
    });

    test('every known level key has a confirmed Portuguese label', () {
      expect(ProgressMapper.levelLabel('Candidate'), 'Candidato(a)');
      expect(ProgressMapper.levelLabel('Practice'), 'Ensaio');
      expect(
        ProgressMapper.levelLabel('YouthService'),
        'Reunião de Jovens e Menores',
      );
      expect(ProgressMapper.levelLabel('OfficialService'), 'Culto Oficial');
      expect(ProgressMapper.levelLabel('Officialized'), 'Oficialização');
    });
  });
}
