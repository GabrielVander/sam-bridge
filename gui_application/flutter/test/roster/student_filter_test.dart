import 'package:flutter_application/roster/student_filter.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_test/flutter_test.dart';

StudentListItem _student(String name, {String location = 'Centro'}) =>
    StudentListItem(
      id: name,
      name: name,
      location: location,
      position: 'Ensaio',
    );

List<String> _names(List<StudentListItem> students) =>
    students.map((s) => s.name).toList();

void main() {
  final joao = _student('João Silva', location: 'Vila Nova');
  final maria = _student('Maria Souza', location: 'Centro');
  final pedro = _student('Pedro Santos', location: 'Jardim');
  final everyone = [joao, maria, pedro];

  group('StudentFilter', () {
    test('without criteria lets every student through', () {
      expect(const StudentFilter().apply(everyone), everyone);
      expect(const StudentFilter().isEmpty, isTrue);
    });

    test('matches names ignoring accents, case and surrounding spaces', () {
      const filter = StudentFilter(nameQuery: '  JOAO ');

      expect(_names(filter.apply(everyone)), ['João Silva']);
    });

    test('tolerates small typos in the name', () {
      const filter = StudentFilter(nameQuery: 'mria');

      expect(_names(filter.apply(everyone)), contains('Maria Souza'));
    });

    test('falls back to a plain substring match far into a long name', () {
      final longName = _student(
        'Maria Aparecida da Conceição dos Santos Oliveira Pereira',
      );
      const filter = StudentFilter(nameQuery: 'pereira');

      expect(_names(filter.apply([longName, pedro])), [longName.name]);
    });

    test('keeps only the selected locations', () {
      const filter = StudentFilter(locations: {'Centro', 'Jardim'});

      expect(_names(filter.apply(everyone)), ['Maria Souza', 'Pedro Santos']);
      expect(filter.isEmpty, isFalse);
    });

    test('combines the name and the locations', () {
      const filter = StudentFilter(nameQuery: 'maria', locations: {'Jardim'});

      expect(filter.apply(everyone), isEmpty);
    });

    test('changing one criterion keeps the other', () {
      const filter = StudentFilter(nameQuery: 'maria', locations: {'Centro'});

      expect(
        filter.copyWith(nameQuery: 'pedro'),
        const StudentFilter(nameQuery: 'pedro', locations: {'Centro'}),
      );
      expect(
        filter.copyWith(locations: {'Jardim'}),
        const StudentFilter(nameQuery: 'maria', locations: {'Jardim'}),
      );
    });

    test('offers each non-empty location once, in alphabetical order', () {
      final students = [
        ...everyone,
        _student('Ana', location: 'Centro'),
        _student('Sem Local', location: ''),
      ];

      expect(StudentFilter.locationsOf(students), [
        'Centro',
        'Jardim',
        'Vila Nova',
      ]);
    });
  });
}
