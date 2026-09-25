import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/rust/api/roster.dart';
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
        position: StudentPositionDto.invalid(raw: 'ALGO DESCONHECIDO'),
        location: 'Somewhere',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.position, 'ALGO DESCONHECIDO');
    });

    group('instrument', () {
      StudentListItem mapInstrument(String? instrumentName) =>
          RosterMapper.toViewModel(
            StudentSummaryDto(
              id: '1',
              name: 'Jane Doe',
              position: const StudentPositionDto.practice(),
              location: 'Some Location',
              instrumentName: instrumentName,
            ),
          );

      test('keeps the first letter uppercase and lowercases the rest', () {
        expect(mapInstrument('SAXOFONE TENOR').instrument, 'Saxofone tenor');
      });

      test('keeps accented letters', () {
        expect(mapInstrument('OBOÉ').instrument, 'Oboé');
      });

      test('shows instruments the app does not know', () {
        expect(mapInstrument('BANDOLIM').instrument, 'Bandolim');
      });

      test('trims surrounding spaces', () {
        expect(mapInstrument('  VIOLINO ').instrument, 'Violino');
      });

      test('has no instrument when SAM sent none', () {
        expect(mapInstrument(null).instrument, isNull);
      });

      test('has no instrument when the name is blank', () {
        expect(mapInstrument('').instrument, isNull);
        expect(mapInstrument('   ').instrument, isNull);
      });

      test('does not change the position label', () {
        expect(mapInstrument('VIOLINO').position, 'Ensaio');
      });
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
