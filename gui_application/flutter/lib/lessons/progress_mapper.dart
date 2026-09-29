import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/rust/api/progress.dart';
import 'package:flutter_application/shared/level.dart';

class ProgressMapper {
  static ProgressView toViewModel(ProgressAssessmentDto dto) => ProgressView(
    checkpoints: dto.checkpoints.map(_toCheckpoint).toList(),
    msaRelativePercent: dto.msaRelativePercent,
    methodRelativePercent: dto.methodRelativePercent,
    combinedPercent: dto.combinedPercent,
    overallCheckpointPercent: dto.overallCheckpointPercent,
    nextLevel: switch (dto.nextLevel) {
      final level? => _toLevel(level),
      null => null,
    },
  );

  static CheckpointView _toCheckpoint(CheckpointStatusDto dto) =>
      CheckpointView(
        level: _toLevel(dto.level),
        status: switch (dto) {
          CheckpointStatusDto(achieved: true) => CheckpointStatus.achieved,
          CheckpointStatusDto(readyToAdvance: true) =>
            CheckpointStatus.readyForExam,
          _ => CheckpointStatus.pending,
        },
        msaMet: dto.requirement.msaMet,
        methodMet: dto.requirement.methodMet,
      );

  static ReportedLevel _toLevel(MusicianLevelDto level) => switch (level) {
    MusicianLevelDto_Candidate() => const KnownLevel(Level.candidate),
    MusicianLevelDto_Practice() => const KnownLevel(Level.practice),
    MusicianLevelDto_YouthService() => const KnownLevel(Level.youthService),
    MusicianLevelDto_OfficialService() => const KnownLevel(
      Level.officialService,
    ),
    MusicianLevelDto_Officialized() => const KnownLevel(Level.officialized),
    MusicianLevelDto_Unknown(:final raw) => UnrecognizedLevel(raw),
  };
}
