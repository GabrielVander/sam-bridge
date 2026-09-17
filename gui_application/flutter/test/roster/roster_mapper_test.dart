import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('RosterMapper', () {
    test('maps a known position to its Portuguese label', () {
      const dto = StudentSummaryDto(
        id: '1',
        name: 'Jane Doe',
        position: StudentPositionDto.youthService(),
        location: 'Some Location',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(
        viewModel,
        const StudentListItem(
          id: '1',
          name: 'Jane Doe',
          location: 'Some Location',
          position: 'Reunião de Jovens e Menores',
        ),
      );
    });

    test('maps an invalid position to its raw SAM string', () {
      const dto = StudentSummaryDto(
        id: '2',
        name: 'John Doe',
        position: StudentPositionDto.invalid('ALGO DESCONHECIDO'),
        location: 'Somewhere',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.position, 'ALGO DESCONHECIDO');
    });

    test('maps a list of dtos preserving order', () {
      const dtos = [
        StudentSummaryDto(
          id: '1',
          name: 'A',
          position: StudentPositionDto.candidate(),
          location: 'L1',
        ),
        StudentSummaryDto(
          id: '2',
          name: 'B',
          position: StudentPositionDto.gemSecretary(),
          location: 'L2',
        ),
      ];

      final result = RosterMapper.toViewModels(dtos);

      expect(result.map((s) => s.id).toList(), ['1', '2']);
      expect(result[0].position, 'Candidato(a)');
      expect(result[1].position, 'Secretário(a) do GEM');
    });
  });
}
