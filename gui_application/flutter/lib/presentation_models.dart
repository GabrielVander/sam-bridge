class StudentListItem {
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
  int get hashCode =>
      id.hashCode ^
      name.hashCode ^
      location.hashCode ^
      position.hashCode ^
      instrument.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StudentListItem &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          name == other.name &&
          location == other.location &&
          position == other.position &&
          instrument == other.instrument;
}

enum LessonKind { msa, method }

class LessonItem {
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
  int get hashCode =>
      kind.hashCode ^
      id.hashCode ^
      date.hashCode ^
      phase.hashCode ^
      page.hashCode ^
      lesson.hashCode ^
      clef.hashCode ^
      description.hashCode ^
      instructor.hashCode ^
      method.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LessonItem &&
          runtimeType == other.runtimeType &&
          kind == other.kind &&
          id == other.id &&
          date == other.date &&
          phase == other.phase &&
          page == other.page &&
          lesson == other.lesson &&
          clef == other.clef &&
          description == other.description &&
          instructor == other.instructor &&
          method == other.method;
}

class StudentLessonsView {
  final List<LessonItem> msa;
  final List<LessonItem> method;

  const StudentLessonsView({required this.msa, required this.method});

  factory StudentLessonsView.empty() =>
      const StudentLessonsView(msa: [], method: []);

  @override
  int get hashCode => msa.hashCode ^ method.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StudentLessonsView &&
          runtimeType == other.runtimeType &&
          msa == other.msa &&
          method == other.method;
}

class CheckpointView {
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
  int get hashCode =>
      levelKey.hashCode ^
      label.hashCode ^
      achieved.hashCode ^
      readyToAdvance.hashCode ^
      msaMet.hashCode ^
      methodMet.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CheckpointView &&
          runtimeType == other.runtimeType &&
          levelKey == other.levelKey &&
          label == other.label &&
          achieved == other.achieved &&
          readyToAdvance == other.readyToAdvance &&
          msaMet == other.msaMet &&
          methodMet == other.methodMet;
}

class ProgressView {
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
  int get hashCode =>
      checkpoints.hashCode ^
      msaRelativePercent.hashCode ^
      methodRelativePercent.hashCode ^
      combinedPercent.hashCode ^
      overallCheckpointPercent.hashCode ^
      nextLevelLabel.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ProgressView &&
          runtimeType == other.runtimeType &&
          checkpoints == other.checkpoints &&
          msaRelativePercent == other.msaRelativePercent &&
          methodRelativePercent == other.methodRelativePercent &&
          combinedPercent == other.combinedPercent &&
          overallCheckpointPercent == other.overallCheckpointPercent &&
          nextLevelLabel == other.nextLevelLabel;
}
