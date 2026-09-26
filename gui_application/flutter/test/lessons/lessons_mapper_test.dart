import 'package:flutter_application/lessons/lessons_mapper.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/lessons.dart';

void main() {
  group('LessonsMapper', () {
    test('maps a full MSA lesson', () {
      final dto = studentLessons(
        msa: [
          lesson(
            id: '559783',
            date: (2025, 9, 9),
            phase: ('4.5', '4.5'),
            page: ('38', '38'),
            lesson: ('7', '8'),
            clef: Clefs.g,
            description: 'Passou lições 7 e 8, estudar próximas lições.',
            instructor: 'MARCOS ROGÉRIO COSME',
          ),
        ],
      );

      final view = LessonsMapper.toViewModel(dto);

      expect(view.msa, hasLength(1));
      expect(view.method, isEmpty);
      final item = view.msa.single;
      expect(item.kind, LessonKind.msa);
      expect(item.id, '559783');
      expect(item.date, DateTime(2025, 9, 9));
      expect(item.phase, '4.5');
      expect(item.page, '38');
      expect(item.lesson, '7 - 8');
      expect(item.clef, Clef.g);
      expect(item.description, 'Passou lições 7 e 8, estudar próximas lições.');
      expect(item.instructor, 'MARCOS ROGÉRIO COSME');
      expect(item.method, '');
    });

    test('maps a full Método (instrument) lesson', () {
      final dto = studentLessons(
        method: [
          lesson(
            id: '214020',
            date: (2023, 12, 4),
            page: ('00', '00'),
            lesson: ('00', '00'),
            description: 'Postura do violino',
            instructor: 'MURILO FAGNER CARDOSO',
            method: 'MÉTODO CCB - SCHIMOLL - VIOLINO',
          ),
        ],
      );

      final view = LessonsMapper.toViewModel(dto);

      expect(view.msa, isEmpty);
      final item = view.method.single;
      expect(item.kind, LessonKind.method);
      expect(item.page, '00');
      expect(item.lesson, '00');
      expect(item.clef, isNull);
      expect(item.method, 'MÉTODO CCB - SCHIMOLL - VIOLINO');
    });

    test('an all-fields-absent lesson has no date or clef and empty text', () {
      final dto = studentLessons(msa: [lesson()]);

      final view = LessonsMapper.toViewModel(dto);

      final item = view.msa.single;
      expect(item.id, '');
      expect(item.date, isNull);
      expect(item.phase, '');
      expect(item.page, '');
      expect(item.lesson, '');
      expect(item.clef, isNull);
      expect(item.description, '');
      expect(item.instructor, '');
      expect(item.method, '');
    });

    final clefs = {Clefs.g: Clef.g, Clefs.c: Clef.c, Clefs.f: Clef.f};

    for (final MapEntry(key: dto, value: expected) in clefs.entries) {
      test('maps the $dto clef', () {
        final view = LessonsMapper.toViewModel(
          studentLessons(msa: [lesson(clef: dto)]),
        );

        expect(view.msa.single.clef, expected);
      });
    }
  });
}
