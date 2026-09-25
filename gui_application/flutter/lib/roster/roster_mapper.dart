import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/rust/api/roster.dart';
import 'package:flutter_application/shared/level_labels.dart';

class RosterMapper {
  static List<StudentListItem> toViewModels(List<StudentSummaryDto> dtos) =>
      dtos.map(toViewModel).toList();

  static StudentListItem toViewModel(StudentSummaryDto dto) => StudentListItem(
    id: dto.id,
    name: dto.name,
    location: dto.location,
    position: _positionLabel(dto.position),
    instrument: _instrumentLabel(dto.instrumentName),
  );

  static String? _instrumentLabel(String? name) {
    final trimmed = name?.trim();
    if (trimmed == null || trimmed.isEmpty) return null;
    return trimmed.substring(0, 1).toUpperCase() +
        trimmed.substring(1).toLowerCase();
  }

  static String _positionLabel(StudentPositionDto position) =>
      switch (position) {
        StudentPositionDto_Candidate() => LevelLabels.candidate,
        StudentPositionDto_Practice() => LevelLabels.practice,
        StudentPositionDto_YouthService() => LevelLabels.youthService,
        StudentPositionDto_OfficialService() => LevelLabels.officialService,
        StudentPositionDto_Officialized() => LevelLabels.officialized,
        StudentPositionDto_HalfHour() => LevelLabels.halfHour,
        StudentPositionDto_YouthServiceHalfHour() =>
          '${LevelLabels.youthService} / ${LevelLabels.halfHour}',
        StudentPositionDto_YouthServicePractice() =>
          '${LevelLabels.youthService} / ${LevelLabels.practice}',
        StudentPositionDto_YouthServiceOfficialService() =>
          '${LevelLabels.youthService} / ${LevelLabels.officialService}',
        StudentPositionDto_YouthServiceOfficialized() =>
          '${LevelLabels.youthService} / ${LevelLabels.officialized}',
        StudentPositionDto_GemSecretary() => 'Secretário(a) do GEM',
        StudentPositionDto_MusicSecretary() => 'Secretário(a) de Música',
        StudentPositionDto_Invalid(:final raw) => raw,
      };
}
