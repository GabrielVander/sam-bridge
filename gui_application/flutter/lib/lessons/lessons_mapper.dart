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
    date: _date(dto.date),
    phase: _formatRange(dto.phase),
    page: _formatRange(dto.page),
    lesson: _formatRange(dto.lesson),
    clef: _clef(dto.clef),
    description: dto.description ?? '',
    instructor: dto.instructor ?? '',
    method: dto.method ?? '',
  );

  static DateTime? _date(DateDto? date) =>
      date == null ? null : DateTime(date.year, date.month, date.day);

  static String _formatRange(RangeDto? range) {
    if (range == null) return '';
    if (range.from == range.to) return range.from;
    return '${range.from} - ${range.to}';
  }

  static Clef? _clef(ClefDto? clef) => switch (clef) {
    ClefDto.g => Clef.g,
    ClefDto.c => Clef.c,
    ClefDto.f => Clef.f,
    null => null,
  };
}
