import 'package:equatable/equatable.dart';
import 'package:flutter_application/shared/level.dart';

enum LessonKind { msa, method }

enum Clef { g, c, f }

class LessonItem extends Equatable {
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

  @override
  List<Object?> get props => [
    kind,
    id,
    date,
    phase,
    page,
    lesson,
    clef,
    description,
    instructor,
    method,
  ];
}

class StudentLessonsView extends Equatable {
  final List<LessonItem> msa;
  final List<LessonItem> method;

  const StudentLessonsView({required this.msa, required this.method});

  factory StudentLessonsView.empty() =>
      const StudentLessonsView(msa: [], method: []);

  @override
  List<Object?> get props => [msa, method];
}

class CheckpointView extends Equatable {
  final ReportedLevel level;
  final bool achieved;
  final bool readyToAdvance;
  final bool msaMet;
  final bool methodMet;

  const CheckpointView({
    required this.level,
    required this.achieved,
    required this.readyToAdvance,
    required this.msaMet,
    required this.methodMet,
  });

  @override
  List<Object?> get props => [
    level,
    achieved,
    readyToAdvance,
    msaMet,
    methodMet,
  ];
}

class ProgressView extends Equatable {
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

  @override
  List<Object?> get props => [
    checkpoints,
    msaRelativePercent,
    methodRelativePercent,
    combinedPercent,
    overallCheckpointPercent,
    nextLevel,
  ];
}
