import 'package:flutter_application/dto.dart';
import 'package:flutter_application/presentation_models.dart';

class LessonsMapper {
  static StudentLessonsView toViewModel(StudentLessonsDto dto) {
    final msa = dto.approved.map((l) => _toItem(l, LessonKind.msa)).toList();
    final method = dto.method.map((l) => _toItem(l, LessonKind.method)).toList();
    _sortMostRecentFirst(msa);
    _sortMostRecentFirst(method);
    return StudentLessonsView(msa: msa, method: method);
  }

  static LessonItem _toItem(LessonDto dto, LessonKind kind) => LessonItem(
        kind: kind,
        id: dto.id,
        date: dto.date,
        phase: _rangeLabel(dto.phase),
        page: _rangeLabel(dto.page),
        lesson: _rangeLabel(dto.lesson),
        clef: _clefLabel(dto.clef),
        description: dto.description,
        instructor: dto.instructor,
        method: dto.method,
      );

  static void _sortMostRecentFirst(List<LessonItem> items) {
    items.sort((a, b) => b.date.compareTo(a.date));
  }

  static String _rangeLabel(RangeDto? range) => switch (range) {
        null => '',
        RangeDto(:final from, :final to) when from == to => from,
        RangeDto(:final from, :final to) => '$from - $to',
      };

  static String _clefLabel(String? clef) => switch (clef) {
        'G' => 'Sol',
        'C' => 'Dó',
        'F' => 'Fá',
        _ => '',
      };
}
