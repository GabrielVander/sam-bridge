import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/shared/level.dart';
import 'package:flutter_test/flutter_test.dart';

import 'localization.dart';

LessonItem lessonItem({
  LessonKind kind = LessonKind.msa,
  String id = '1',
  String date = '01/02/2024',
  String phase = '2',
  String page = '10',
  String lesson = '3',
  String clef = 'Sol',
  String description = 'Escalas maiores',
  String instructor = 'Maria Souza',
  String method = 'Método A',
}) => LessonItem(
  kind: kind,
  id: id,
  date: date,
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
  bool achieved = false,
  bool readyToAdvance = false,
  bool msaMet = false,
  bool methodMet = false,
}) => CheckpointView(
  level: level,
  achieved: achieved,
  readyToAdvance: readyToAdvance,
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

Future<void> pumpInApp(WidgetTester tester, Widget child) =>
    tester.pumpWidget(localizedApp(home: Scaffold(body: child)));
