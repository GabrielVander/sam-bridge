import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/shared/level.dart';
import 'package:flutter_test/flutter_test.dart';

import 'localization.dart';

LessonItem lessonItem({
  LessonKind kind = LessonKind.msa,
  String id = '1',
  (int, int, int)? date = (2024, 2, 1),
  String phase = '2',
  String page = '10',
  String lesson = '3',
  Clef? clef = Clef.g,
  String description = 'Escalas maiores',
  String instructor = 'Maria Souza',
  String method = 'Método A',
}) => LessonItem(
  kind: kind,
  id: id,
  date: date == null ? null : DateTime(date.$1, date.$2, date.$3),
  phase: phase,
  page: page,
  lesson: lesson,
  clef: clef,
  description: description,
  instructor: instructor,
  method: method,
);

CheckpointView checkpoint({
  ReportedLevel level = const KnownLevel(Level.practice),
  CheckpointStatus status = CheckpointStatus.pending,
  bool msaMet = false,
  bool methodMet = false,
}) => CheckpointView(
  level: level,
  status: status,
  msaMet: msaMet,
  methodMet: methodMet,
);

ProgressView progressView({
  List<CheckpointView>? checkpoints,
  double msaRelativePercent = 40,
  double methodRelativePercent = 75,
  double combinedPercent = 50,
  double overallCheckpointPercent = 25,
  ReportedLevel? nextLevel = const KnownLevel(Level.officialService),
}) => ProgressView(
  checkpoints: checkpoints ?? [checkpoint()],
  msaRelativePercent: msaRelativePercent,
  methodRelativePercent: methodRelativePercent,
  combinedPercent: combinedPercent,
  overallCheckpointPercent: overallCheckpointPercent,
  nextLevel: nextLevel,
);

Future<void> pumpInApp(
  WidgetTester tester,
  Widget child, {
  Locale locale = portuguese,
}) => tester.pumpWidget(
  localizedApp(
    locale: locale,
    home: Scaffold(body: child),
  ),
);
