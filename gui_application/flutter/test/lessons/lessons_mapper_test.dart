import 'package:flutter_application/lessons/lessons_mapper.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/rust/api/lessons.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('LessonsMapper', () {
    test('maps a full MSA lesson', () {
      const dto = StudentLessonsDto(
        msa: [
          LessonDto(
            id: '559783',
            date: DateDto(year: 2025, month: 9, day: 9),
            phase: RangeDto(from: '4.5', to: '4.5'),
            page: RangeDto(from: '38', to: '38'),
            lesson: RangeDto(from: '7', to: '8'),
            clef: ClefDto.g,
            description: 'Passou lições 7 e 8, estudar próximas lições.',
            instructor: 'MARCOS ROGÉRIO COSME',
          ),
        ],
        method: [],
      );

      final view = LessonsMapper.toViewModel(dto);

      expect(view.msa, hasLength(1));
      expect(view.method, isEmpty);
      final item = view.msa.single;
      expect(item.kind, LessonKind.msa);
      expect(item.id, '559783');
      expect(item.date, '09/09/2025');
      expect(item.phase, '4.5');
      expect(item.page, '38');
      expect(item.lesson, '7 - 8');
      expect(item.clef, 'Sol');
      expect(item.description, 'Passou lições 7 e 8, estudar próximas lições.');
      expect(item.instructor, 'MARCOS ROGÉRIO COSME');
      expect(item.method, '');
    });

    test('maps a full Método (instrument) lesson', () {
      const dto = StudentLessonsDto(
        msa: [],
        method: [
          LessonDto(
            id: '214020',
            date: DateDto(year: 2023, month: 12, day: 4),
            page: RangeDto(from: '00', to: '00'),
            lesson: RangeDto(from: '00', to: '00'),
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
      expect(item.clef, '');
      expect(item.method, 'MÉTODO CCB - SCHIMOLL - VIOLINO');
    });

    test('all-fields-absent lesson maps to empty strings, not nulls', () {
      const dto = StudentLessonsDto(msa: [LessonDto()], method: []);

      final view = LessonsMapper.toViewModel(dto);

      final item = view.msa.single;
      expect(item.id, '');
      expect(item.date, '');
      expect(item.phase, '');
      expect(item.page, '');
      expect(item.lesson, '');
      expect(item.clef, '');
      expect(item.description, '');
      expect(item.instructor, '');
      expect(item.method, '');
    });
  });
}
