import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/rust/api/roster.dart';
import 'package:flutter_application/roster/student_position.dart';
import 'package:flutter_application/shared/level.dart';

class RosterMapper {
  static List<StudentListItem> toViewModels(List<StudentSummaryDto> dtos) =>
      dtos.map(toViewModel).toList();

  static StudentListItem toViewModel(StudentSummaryDto dto) => StudentListItem(
    id: dto.id,
    name: dto.name,
    location: dto.location,
    position: _position(dto.position),
    instrument: _instrumentLabel(dto.instrumentName),
  );

  static String? _instrumentLabel(String? name) {
    final trimmed = name?.trim();
    if (trimmed == null || trimmed.isEmpty) return null;
    return trimmed.substring(0, 1).toUpperCase() +
        trimmed.substring(1).toLowerCase();
  }

  static StudentPosition _position(
    StudentPositionDto position,
  ) => switch (position) {
    StudentPositionDto_Candidate() => const MusicianPosition([Level.candidate]),
    StudentPositionDto_Practice() => const MusicianPosition([Level.practice]),
    StudentPositionDto_YouthService() => const MusicianPosition([
      Level.youthService,
    ]),
    StudentPositionDto_OfficialService() => const MusicianPosition([
      Level.officialService,
    ]),
    StudentPositionDto_Officialized() => const MusicianPosition([
      Level.officialized,
    ]),
    StudentPositionDto_HalfHour() => const MusicianPosition([Level.halfHour]),
    StudentPositionDto_YouthServiceHalfHour() => const MusicianPosition([
      Level.youthService,
      Level.halfHour,
    ]),
    StudentPositionDto_YouthServicePractice() => const MusicianPosition([
      Level.youthService,
      Level.practice,
    ]),
    StudentPositionDto_YouthServiceOfficialService() => const MusicianPosition([
      Level.youthService,
      Level.officialService,
    ]),
    StudentPositionDto_YouthServiceOfficialized() => const MusicianPosition([
      Level.youthService,
      Level.officialized,
    ]),
    StudentPositionDto_GemSecretary() => const GemSecretaryPosition(),
    StudentPositionDto_MusicSecretary() => const MusicSecretaryPosition(),
    StudentPositionDto_Invalid(:final raw) => UnrecognizedPosition(raw),
  };
}
