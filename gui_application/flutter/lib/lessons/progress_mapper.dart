import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';

class ProgressMapper {
  static ProgressView toViewModel(ProgressAssessmentDto dto) => ProgressView(
    checkpoints: dto.checkpoints.map(_toCheckpoint).toList(),
    msaRelativePercent: dto.msaRelativePercent,
    methodRelativePercent: dto.methodRelativePercent,
    combinedPercent: dto.combinedPercent,
    overallCheckpointPercent: dto.overallCheckpointPercent,
    nextLevelLabel: dto.nextLevel == null ? null : levelLabel(dto.nextLevel!),
  );

  static CheckpointView _toCheckpoint(CheckpointStatusDto dto) =>
      CheckpointView(
        levelKey: dto.level,
        label: levelLabel(dto.level),
        achieved: dto.achieved,
        readyToAdvance: dto.readyToAdvance,
        msaMet: dto.requirement.msaMet,
        methodMet: dto.requirement.methodMet,
      );

  static String levelLabel(String rawLevel) => switch (rawLevel) {
    'Candidate' => 'Candidato(a)',
    'Practice' => 'Ensaio',
    'YouthService' => 'Reunião de Jovens e Menores',
    'OfficialService' => 'Culto Oficial',
    'Officialized' => 'Oficialização',
    _ => rawLevel,
  };
}
