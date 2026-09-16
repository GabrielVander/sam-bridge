import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';

class RosterMapper {
  static List<StudentListItem> toViewModels(List<StudentSummaryDto> dtos) =>
      dtos.map(toViewModel).toList();

  static StudentListItem toViewModel(StudentSummaryDto dto) => StudentListItem(
    id: dto.id,
    name: dto.name,
    location: dto.location,
    position: _rawVariantName(dto.position),
  );

  static String _rawVariantName(StudentPositionDto position) =>
      switch (position) {
        StudentPositionDto_Candidate() => 'Candidate',
        StudentPositionDto_Practice() => 'Practice',
        StudentPositionDto_YouthService() => 'YouthService',
        StudentPositionDto_OfficialService() => 'OfficialService',
        StudentPositionDto_Officialized() => 'Officialized',
        StudentPositionDto_HalfHour() => 'HalfHour',
        StudentPositionDto_YouthServiceHalfHour() => 'YouthServiceHalfHour',
        StudentPositionDto_YouthServicePractice() => 'YouthServicePractice',
        StudentPositionDto_YouthServiceOfficialService() =>
          'YouthServiceOfficialService',
        StudentPositionDto_YouthServiceOfficialized() =>
          'YouthServiceOfficialized',
        StudentPositionDto_GemSecretary() => 'GemSecretary',
        StudentPositionDto_MusicSecretary() => 'MusicSecretary',
        StudentPositionDto_Invalid(:final field0) => field0,
      };
}
