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

  static CheckpointView _toCheckpoint(CheckpointDto dto) => CheckpointView(
    level: _toLevel(dto.level),
    status: switch (dto.status) {
      CheckpointStatusDto.achieved => CheckpointStatus.achieved,
      CheckpointStatusDto.readyForExam => CheckpointStatus.readyForExam,
      CheckpointStatusDto.pending => CheckpointStatus.pending,
    },
    msaMet: dto.requirement.msaMet,
    methodMet: dto.requirement.methodMet,
  );

  static Level _toLevel(ProgressionLevelDto level) => switch (level) {
    ProgressionLevelDto.candidate => Level.candidate,
    ProgressionLevelDto.practice => Level.practice,
    ProgressionLevelDto.youthService => Level.youthService,
    ProgressionLevelDto.officialService => Level.officialService,
    ProgressionLevelDto.officialized => Level.officialized,
  };
}
