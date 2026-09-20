import 'package:equatable/equatable.dart';

class StudentListItem extends Equatable {
  final String id;
  final String name;
  final String location;
  final String position;
  final String? instrument;

  const StudentListItem({
    required this.id,
    required this.name,
    required this.location,
    required this.position,
    this.instrument,
  });

  @override
  List<Object?> get props => [id, name, location, position, instrument];
}

enum LessonKind { msa, method }

class LessonItem extends Equatable {
  final LessonKind kind;
  final String id;
  final String date;
  final String phase;
  final String page;
  final String lesson;
  final String clef;
  final String description;
  final String instructor;
  final String method;

  const LessonItem({
    required this.kind,
    required this.id,
    required this.date,
    required this.phase,
    required this.page,
    required this.lesson,
    required this.clef,
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
  final String levelKey;
  final String label;
  final bool achieved;
  final bool readyToAdvance;
  final bool msaMet;
  final bool methodMet;

  const CheckpointView({
    required this.levelKey,
    required this.label,
    required this.achieved,
    required this.readyToAdvance,
    required this.msaMet,
    required this.methodMet,
  });

  @override
  List<Object?> get props => [
    levelKey,
    label,
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
  final String? nextLevelLabel;

  const ProgressView({
    required this.checkpoints,
    required this.msaRelativePercent,
    required this.methodRelativePercent,
    required this.combinedPercent,
    required this.overallCheckpointPercent,
    this.nextLevelLabel,
  });

  @override
  List<Object?> get props => [
    checkpoints,
    msaRelativePercent,
    methodRelativePercent,
    combinedPercent,
    overallCheckpointPercent,
    nextLevelLabel,
  ];
}

class ErrorReport extends Equatable {
  final String userMessage;
  final String details;

  const ErrorReport({required this.userMessage, required this.details});

  @override
  List<Object?> get props => [userMessage, details];
}
