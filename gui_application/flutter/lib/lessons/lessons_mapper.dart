import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/rust/api/lessons.dart';

class LessonsMapper {
  static StudentLessonsView toViewModel(StudentLessonsDto dto) =>
      StudentLessonsView(
        msa: dto.msa.map((l) => _toItem(l, LessonKind.msa)).toList(),
        method: dto.method.map((l) => _toItem(l, LessonKind.method)).toList(),
      );

  static LessonItem _toItem(LessonDto dto, LessonKind kind) => LessonItem(
    kind: kind,
    id: dto.id ?? '',
    date: _formatDate(dto.date),
    phase: _formatRange(dto.phase),
    page: _formatRange(dto.page),
    lesson: _formatRange(dto.lesson),
    clef: _formatClef(dto.clef),
    description: dto.description ?? '',
    instructor: dto.instructor ?? '',
    method: dto.method ?? '',
  );

  static String _formatDate(DateDto? date) {
    if (date == null) return '';
    String twoDigits(int n) => n.toString().padLeft(2, '0');
    return '${twoDigits(date.day)}/${twoDigits(date.month)}/${date.year}';
  }

  static String _formatRange(RangeDto? range) {
    if (range == null) return '';
    if (range.from == range.to) return range.from;
    return '${range.from} - ${range.to}';
  }

  static String _formatClef(ClefDto? clef) => switch (clef) {
    ClefDto.g => 'Sol',
    ClefDto.c => 'Dó',
    ClefDto.f => 'Fá',
    null => '',
  };
}
