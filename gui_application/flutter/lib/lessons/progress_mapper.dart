import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/rust/api/progress.dart';
import 'package:flutter_application/shared/level_labels.dart';

class ProgressMapper {
  static ProgressView toViewModel(ProgressAssessmentDto dto) => ProgressView(
    checkpoints: dto.checkpoints.map(_toCheckpoint).toList(),
    msaRelativePercent: dto.msaRelativePercent,
    methodRelativePercent: dto.methodRelativePercent,
    combinedPercent: dto.combinedPercent,
    overallCheckpointPercent: dto.overallCheckpointPercent,
    nextLevelLabel: switch (dto.nextLevel) {
      final level? => levelLabel(level),
      null => null,
    },
  );

  static CheckpointView _toCheckpoint(CheckpointStatusDto dto) =>
      CheckpointView(
        label: levelLabel(dto.level),
        achieved: dto.achieved,
        readyToAdvance: dto.readyToAdvance,
        msaMet: dto.requirement.msaMet,
        methodMet: dto.requirement.methodMet,
      );

  static String levelLabel(MusicianLevelDto level) => switch (level) {
    MusicianLevelDto_Candidate() => LevelLabels.candidate,
    MusicianLevelDto_Practice() => LevelLabels.practice,
    MusicianLevelDto_YouthService() => LevelLabels.youthService,
    MusicianLevelDto_OfficialService() => LevelLabels.officialService,
    MusicianLevelDto_Officialized() => LevelLabels.officialized,
    MusicianLevelDto_Unknown(:final raw) => raw,
  };
}
