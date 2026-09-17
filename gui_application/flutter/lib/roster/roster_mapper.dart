import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';

class RosterMapper {
  static List<StudentListItem> toViewModels(List<StudentSummaryDto> dtos) =>
      dtos.map(toViewModel).toList();

  static StudentListItem toViewModel(StudentSummaryDto dto) => StudentListItem(
    id: dto.id,
    name: dto.name,
    location: dto.location,
    position: _positionLabel(dto.position),
  );

  static String _positionLabel(StudentPositionDto position) =>
      switch (position) {
        StudentPositionDto_Candidate() => 'Candidato(a)',
        StudentPositionDto_Practice() => 'Ensaio',
        StudentPositionDto_YouthService() => 'Reunião de Jovens e Menores',
        StudentPositionDto_OfficialService() => 'Culto Oficial',
        StudentPositionDto_Officialized() => 'Oficialização',
        StudentPositionDto_HalfHour() => 'Meia Hora',
        StudentPositionDto_YouthServiceHalfHour() =>
          'Reunião de Jovens e Menores / Meia Hora',
        StudentPositionDto_YouthServicePractice() =>
          'Reunião de Jovens e Menores / Ensaio',
        StudentPositionDto_YouthServiceOfficialService() =>
          'Reunião de Jovens e Menores / Culto Oficial',
        StudentPositionDto_YouthServiceOfficialized() =>
          'Reunião de Jovens e Menores / Oficialização',
        StudentPositionDto_GemSecretary() => 'Secretário(a) do GEM',
        StudentPositionDto_MusicSecretary() => 'Secretário(a) de Música',
        StudentPositionDto_Invalid(:final field0) => field0,
      };
}
