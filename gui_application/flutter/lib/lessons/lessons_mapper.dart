import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';

class LessonsMapper {
  static StudentLessonsView toViewModel(StudentLessonsDto dto) =>
      StudentLessonsView(
        msa: dto.approved.map((l) => _toItem(l, LessonKind.msa)).toList(),
        method: dto.method.map((l) => _toItem(l, LessonKind.method)).toList(),
      );

  static LessonItem _toItem(LessonDto dto, LessonKind kind) => LessonItem(
    kind: kind,
    id: dto.id ?? '',
    date: dto.date ?? '',
    phase: _formatRange(dto.phase),
    page: _formatRange(dto.page),
    lesson: _formatRange(dto.lesson),
    clef: _formatClef(dto.clef),
    description: dto.description ?? '',
    instructor: dto.instructor ?? '',
    method: dto.method ?? '',
  );

  static String _formatRange(RangeDto? range) {
    if (range == null) return '';
    if (range.from == range.to) return range.from;
    return '${range.from} - ${range.to}';
  }

  // SAM's own clef vocabulary ("Sol"/"Dó"/"Fá") is shown back to the user
  // rather than the Rust enum's raw variant names, since this is a
  // display-only field with no downstream round-trip requirement.
  static String _formatClef(ClefDto? clef) => switch (clef) {
    ClefDto.g => 'Sol',
    ClefDto.c => 'Dó',
    ClefDto.f => 'Fá',
    null => '',
  };
}
