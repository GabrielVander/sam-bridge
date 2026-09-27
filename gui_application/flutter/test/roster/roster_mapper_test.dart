import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/roster/student_position.dart';
import 'package:flutter_application/shared/level.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/roster.dart';

void main() {
  group('RosterMapper', () {
    test('maps a student with their position', () {
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
          position: MusicianPosition([Level.youthService]),
        ),
      );
    });

    final positions = {
      Positions.candidate: const MusicianPosition([Level.candidate]),
      Positions.practice: const MusicianPosition([Level.practice]),
      Positions.youthService: const MusicianPosition([Level.youthService]),
      Positions.officialService: const MusicianPosition([
        Level.officialService,
      ]),
      Positions.officialized: const MusicianPosition([Level.officialized]),
      Positions.halfHour: const MusicianPosition([Level.halfHour]),
      Positions.youthServiceHalfHour: const MusicianPosition([
        Level.youthService,
        Level.halfHour,
      ]),
      Positions.youthServicePractice: const MusicianPosition([
        Level.youthService,
        Level.practice,
      ]),
      Positions.youthServiceOfficialService: const MusicianPosition([
        Level.youthService,
        Level.officialService,
      ]),
      Positions.youthServiceOfficialized: const MusicianPosition([
        Level.youthService,
        Level.officialized,
      ]),
      Positions.gemSecretary: const GemSecretaryPosition(),
      Positions.musicSecretary: const MusicSecretaryPosition(),
      Positions.invalid('ALGO DESCONHECIDO'): const UnrecognizedPosition(
        'ALGO DESCONHECIDO',
      ),
    };

    for (final MapEntry(key: dto, value: expected) in positions.entries) {
      test('maps the $dto position to $expected', () {
        final viewModel = RosterMapper.toViewModel(
          studentSummary(position: dto),
        );

        expect(viewModel.position, expected);
      });
    }

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
      expect(result[0].position, const MusicianPosition([Level.candidate]));
      expect(result[1].position, const GemSecretaryPosition());
    });
  });
}
