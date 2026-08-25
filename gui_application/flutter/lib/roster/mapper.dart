import 'package:flutter_application/dto.dart';
import 'package:flutter_application/presentation_models.dart';

class RosterMapper {
  static StudentListItem toViewModel(StudentSummaryDto dto) {
    return StudentListItem(
      id: dto.id,
      name: dto.name,
      location: dto.location,
      position: _positionLabel(dto.position),
      rawLevel: _rawLevel(dto.position),
    );
  }

  static List<StudentListItem> toViewModels(List<StudentSummaryDto> dtos) =>
      dtos.map(toViewModel).toList();

  static String _positionLabel(DtoPosition position) => switch (position) {
        DtoPosition_Musician(:final levelName) => 'Músico · ${_musicianLevel(levelName)}',
        DtoPosition_Organist(:final levelName) => 'Organista · ${_organistLevel(levelName)}',
        DtoPosition_Secretary(:final typeName) => _secretary(typeName),
        DtoPosition_Unknown(:final raw) => raw,
      };

  static String _rawLevel(DtoPosition position) => switch (position) {
        DtoPosition_Musician(:final levelName) => levelName,
        _ => '',
      };

  static String _musicianLevel(String level) => switch (level) {
        'Candidate' => 'Candidato(a)',
        'Practice' => 'Ensaio',
        'YouthService' => 'RJM',
        'OfficialService' => 'Culto Oficial',
        'Officialized' => 'Oficializado',
        _ => level,
      };

  static String _organistLevel(String level) => switch (level) {
        'Candidate' => 'Candidato(a)',
        'Practice' => 'Ensaio',
        'YouthService' => 'RJM',
        'OfficialService' => 'Culto Oficial',
        'HalfHour' => 'Meia hora',
        'YouthServicePractice' => 'RJM / Ensaio',
        'YouthServiceHalfHour' => 'RJM / Meia hora',
        'YouthServiceOfficialService' => 'RJM / Culto Oficial',
        'YouthServiceOfficialized' => 'RJM / Oficializado(a)',
        _ => level,
      };

  static String _secretary(String type) => switch (type) {
        'Gem' => 'Secretário do GEM',
        'Music' => 'Secretário da Música',
        _ => type,
      };
}
