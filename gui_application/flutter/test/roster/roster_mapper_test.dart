import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('RosterMapper', () {
    test('maps a known position to its raw variant name', () {
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
          position: 'YouthService',
          rawLevel: 'YouthService',
        ),
      );
    });

    test('uses the same raw variant name for position and rawLevel', () {
      const dto = StudentSummaryDto(
        id: '3',
        name: 'Carlos',
        position: StudentPositionDto.gemSecretary(),
        location: 'L',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.position, viewModel.rawLevel);
      expect(viewModel.position, 'GemSecretary');
    });

    test('maps an invalid position to its raw SAM string for both fields', () {
      const dto = StudentSummaryDto(
        id: '2',
        name: 'John Doe',
        position: StudentPositionDto.invalid('ALGO DESCONHECIDO'),
        location: 'Somewhere',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.position, 'ALGO DESCONHECIDO');
      expect(viewModel.rawLevel, 'ALGO DESCONHECIDO');
    });

    test('carries the assigned instrument through as rawInstrument', () {
      const dto = StudentSummaryDto(
        id: '4',
        name: 'Maria',
        position: StudentPositionDto.youthService(),
        location: 'L',
        instrument: 'Violino',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.rawInstrument, 'Violino');
    });

    test('a musician with no assigned instrument has none in the view', () {
      const dto = StudentSummaryDto(
        id: '5',
        name: 'Pedro',
        position: StudentPositionDto.candidate(),
        location: 'L',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.rawInstrument, isNull);
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
      expect(result[0].position, 'Candidate');
      expect(result[1].position, 'GemSecretary');
    });
  });
}
