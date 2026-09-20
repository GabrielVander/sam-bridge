import 'package:flutter/material.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_test/flutter_test.dart';

/// Builders keep tests independent of how many fields a model has: each test
/// states only the values it cares about.
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
  String levelKey = 'ensaio',
  String label = 'Ensaio',
  bool achieved = false,
  bool readyToAdvance = false,
  bool msaMet = false,
  bool methodMet = false,
}) => CheckpointView(
  levelKey: levelKey,
  label: label,
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
  String? nextLevelLabel = 'Culto',
}) => ProgressView(
  checkpoints: checkpoints ?? [checkpoint()],
  msaRelativePercent: msaRelativePercent,
  methodRelativePercent: methodRelativePercent,
  combinedPercent: combinedPercent,
  overallCheckpointPercent: overallCheckpointPercent,
  nextLevelLabel: nextLevelLabel,
);

Future<void> pumpInApp(WidgetTester tester, Widget child) =>
    tester.pumpWidget(MaterialApp(home: Scaffold(body: child)));
