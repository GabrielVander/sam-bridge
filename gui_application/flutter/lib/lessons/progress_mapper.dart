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

  // Confirmed against the real SAM roster (see
  // discovers_role_level_and_instrument_vocabulary_from_the_real_students_listing
  // in sam/tests/sam_http_capabilities_and_behaviour.rs), not invented
  // translations. Falls back to the raw round-trip key for anything the
  // domain model reports as `Unknown`.
  static String levelLabel(String rawLevel) => switch (rawLevel) {
    'Candidate' => 'Candidato(a)',
    'Practice' => 'Ensaio',
    'YouthService' => 'Reunião de Jovens e Menores',
    'OfficialService' => 'Culto Oficial',
    'Officialized' => 'Oficialização',
    _ => rawLevel,
  };
}
