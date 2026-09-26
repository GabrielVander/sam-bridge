import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/lessons/student_screen.dart';
import 'package:flutter_application/rust/api/error_report.dart';
import 'package:flutter_application/rust/api/lessons.dart';
import 'package:flutter_application/rust/api/progress.dart';
import 'package:flutter_test/flutter_test.dart';

import 'localization.dart';
import 'pending.dart';

LessonDto lesson({
  String? id,
  (int, int, int)? date,
  (String, String)? phase,
  (String, String)? page,
  (String, String)? lesson,
  ClefDto? clef,
  String? description,
  String? instructor,
  String? method,
}) => LessonDto(
  id: id,
  date: date == null
      ? null
      : DateDto(year: date.$1, month: date.$2, day: date.$3),
  phase: _range(phase),
  page: _range(page),
  lesson: _range(lesson),
  clef: clef,
  description: description,
  instructor: instructor,
  method: method,
);

RangeDto? _range((String, String)? range) =>
    range == null ? null : RangeDto(from: range.$1, to: range.$2);

abstract final class Clefs {
  static const g = ClefDto.g;
  static const c = ClefDto.c;
  static const f = ClefDto.f;
}

StudentLessonsDto studentLessons({
  List<LessonDto> msa = const [],
  List<LessonDto> method = const [],
}) => StudentLessonsDto(msa: msa, method: method);

RetrieveStudentLessonsOutcomeDto lessonsRetrieved([
  StudentLessonsDto lessons = const StudentLessonsDto(msa: [], method: []),
]) => RetrieveStudentLessonsOutcomeDto.success(lessons: lessons);

Pending<RetrieveStudentLessonsOutcomeDto> pendingLessons() => Pending();

RetrieveStudentLessonsOutcomeDto lessonsFailed(ErrorReportDto report) =>
    RetrieveStudentLessonsOutcomeDto.failure(report: report);

abstract final class Levels {
  static const MusicianLevelDto candidate = MusicianLevelDto.candidate();
  static const MusicianLevelDto practice = MusicianLevelDto.practice();
  static const MusicianLevelDto youthService = MusicianLevelDto.youthService();
  static const MusicianLevelDto officialService =
      MusicianLevelDto.officialService();
  static const MusicianLevelDto officialized = MusicianLevelDto.officialized();

  static MusicianLevelDto unknown(String raw) =>
      MusicianLevelDto.unknown(raw: raw);
}

CheckpointStatusDto checkpointStatus(
  MusicianLevelDto level, {
  bool achieved = false,
  bool readyToAdvance = false,
}) => CheckpointStatusDto(
  level: level,
  achieved: achieved,
  readyToAdvance: readyToAdvance,
  requirement: const RequirementStatusDto(msaMet: true, methodMet: true),
);

ProgressAssessmentDto progressAssessment({
  List<CheckpointStatusDto> checkpoints = const [],
  double msaRelativePercent = 0,
  double methodRelativePercent = 0,
  double combinedPercent = 0,
  double overallCheckpointPercent = 0,
  MusicianLevelDto? nextLevel,
}) => ProgressAssessmentDto(
  checkpoints: checkpoints,
  msaRelativePercent: msaRelativePercent,
  methodRelativePercent: methodRelativePercent,
  combinedPercent: combinedPercent,
  overallCheckpointPercent: overallCheckpointPercent,
  nextLevel: nextLevel,
);

AssessStudentProgressOutcomeDto progressAssessed(
  ProgressAssessmentDto assessment,
) => AssessStudentProgressOutcomeDto.success(assessment: assessment);

AssessStudentProgressOutcomeDto noInstrumentAssigned() =>
    const AssessStudentProgressOutcomeDto.noInstrumentAssigned();

AssessStudentProgressOutcomeDto levelNotRecognized(String raw) =>
    AssessStudentProgressOutcomeDto.unknownLevel(rawLevel: raw);

AssessStudentProgressOutcomeDto notAMusician() =>
    const AssessStudentProgressOutcomeDto.notAMusician();

AssessStudentProgressOutcomeDto progressFailed(ErrorReportDto report) =>
    AssessStudentProgressOutcomeDto.failure(report: report);

Future<void> pumpStudent(
  WidgetTester tester, {
  List<RetrieveStudentLessonsOutcomeDto> lessons = const [],
  AssessStudentProgressOutcomeDto? progress,
  Locale locale = portuguese,
}) async {
  final remainingLessons = [...lessons];
  final presenter = LessonsPresenter(
    retrieveStudentLessons: ({required studentId}) async =>
        remainingLessons.isEmpty
        ? lessonsRetrieved()
        : remainingLessons.removeAt(0),
    assessStudentProgress: ({required studentId}) async =>
        progress ?? noInstrumentAssigned(),
  );

  await tester.pumpWidget(
    BlocSignalProvider<LessonsPresenter>.value(
      value: presenter,
      child: localizedApp(
        locale: locale,
        home: const StudentScreen(studentId: '500132'),
      ),
    ),
  );
  await tester.pumpAndSettle();
}
