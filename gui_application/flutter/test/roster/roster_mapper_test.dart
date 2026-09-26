import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/roster.dart';

void main() {
  group('RosterMapper', () {
    test('maps a known position to its Portuguese label', () {
      final dto = studentSummary(
        id: '1',
        name: 'Jane Doe',
        position: Positions.youthService,
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
      final dto = studentSummary(
        id: '2',
        name: 'John Doe',
        position: Positions.invalid('ALGO DESCONHECIDO'),
        location: 'Somewhere',
      );

      final viewModel = RosterMapper.toViewModel(dto);

      expect(viewModel.position, 'ALGO DESCONHECIDO');
    });

    group('instrument', () {
      StudentListItem mapInstrument(String? instrumentName) =>
          RosterMapper.toViewModel(
            studentSummary(
              id: '1',
              name: 'Jane Doe',
              position: Positions.practice,
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
      final dtos = [
        studentSummary(
          id: '1',
          name: 'A',
          position: Positions.candidate,
          location: 'L1',
        ),
        studentSummary(
          id: '2',
          name: 'B',
          position: Positions.gemSecretary,
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
