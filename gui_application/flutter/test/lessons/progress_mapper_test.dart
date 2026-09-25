import 'package:flutter_application/lessons/progress_mapper.dart';
import 'package:flutter_application/rust/api/progress.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ProgressMapper', () {
    test('maps checkpoints and percentages', () {
      const dto = ProgressAssessmentDto(
        checkpoints: [
          CheckpointStatusDto(
            level: MusicianLevelDto.candidate(),
            achieved: true,
            readyToAdvance: false,
            requirement: RequirementStatusDto(msaMet: true, methodMet: true),
          ),
          CheckpointStatusDto(
            level: MusicianLevelDto.youthService(),
            achieved: false,
            readyToAdvance: true,
            requirement: RequirementStatusDto(msaMet: true, methodMet: true),
          ),
        ],
        msaRelativePercent: 100,
        methodRelativePercent: 85,
        combinedPercent: 92.5,
        overallCheckpointPercent: 40,
        nextLevel: MusicianLevelDto.youthService(),
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

    test('an unrecognized level falls back to what SAM wrote', () {
      expect(
        ProgressMapper.levelLabel(
          const MusicianLevelDto.unknown(raw: 'SomethingNew'),
        ),
        'SomethingNew',
      );
    });

    test('every known level has a confirmed Portuguese label', () {
      expect(
        ProgressMapper.levelLabel(const MusicianLevelDto.candidate()),
        'Candidato(a)',
      );
      expect(
        ProgressMapper.levelLabel(const MusicianLevelDto.practice()),
        'Ensaio',
      );
      expect(
        ProgressMapper.levelLabel(const MusicianLevelDto.youthService()),
        'Reunião de Jovens e Menores',
      );
      expect(
        ProgressMapper.levelLabel(const MusicianLevelDto.officialService()),
        'Culto Oficial',
      );
      expect(
        ProgressMapper.levelLabel(const MusicianLevelDto.officialized()),
        'Oficialização',
      );
    });
  });
}
