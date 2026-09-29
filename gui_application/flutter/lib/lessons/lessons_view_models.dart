import 'package:flutter_application/shared/level.dart';

enum LessonKind { msa, method }

enum Clef { g, c, f }

class LessonItem {
  final LessonKind kind;
  final String id;
  final DateTime? date;
  final String phase;
  final String page;
  final String lesson;
  final Clef? clef;
  final String description;
  final String instructor;
  final String method;

  const LessonItem({
    required this.kind,
    required this.id,
    this.date,
    required this.phase,
    required this.page,
    required this.lesson,
    this.clef,
    required this.description,
    required this.instructor,
    required this.method,
  });

  bool get hasNotes =>
      clef != null || description.isNotEmpty || instructor.isNotEmpty;
}

class StudentLessonsView {
  final List<LessonItem> msa;
  final List<LessonItem> method;

  const StudentLessonsView({required this.msa, required this.method});
}

enum CheckpointStatus { achieved, readyForExam, pending }

class CheckpointView {
  final ReportedLevel level;
  final CheckpointStatus status;
  final bool msaMet;
  final bool methodMet;

  const CheckpointView({
    required this.level,
    required this.status,
    required this.msaMet,
    required this.methodMet,
  });
}

class ProgressView {
  final List<CheckpointView> checkpoints;
  final double msaRelativePercent;
  final double methodRelativePercent;
  final double combinedPercent;
  final double overallCheckpointPercent;
  final ReportedLevel? nextLevel;

  const ProgressView({
    required this.checkpoints,
    required this.msaRelativePercent,
    required this.methodRelativePercent,
    required this.combinedPercent,
    required this.overallCheckpointPercent,
    this.nextLevel,
  });
}
