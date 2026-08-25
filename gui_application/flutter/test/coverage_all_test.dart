import 'package:flutter/material.dart';
import 'package:flutter_application/dto.dart';
import 'package:flutter_application/authentication/mapper.dart';
import 'package:flutter_application/lessons/mapper.dart';
import 'package:flutter_application/roster/mapper.dart';
import 'package:flutter_application/lessons/widgets/back_bar.dart';
import 'package:flutter_application/lessons/widgets/category_lessons_view.dart';
import 'package:flutter_application/lessons/widgets/lesson_card.dart';
import 'package:flutter_application/lessons/widgets/student_detail_content.dart';
import 'package:flutter_application/lessons/widgets/unknown_level_banner.dart';
import 'package:flutter_application/widgets/checkpoint_timeline.dart';
import 'package:flutter_application/widgets/info_chip.dart';
import 'package:flutter_application/widgets/progress_bar.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/view_models.dart';
import 'package:flutter_test/flutter_test.dart';

import 'support/fake_sam_portal.dart';

void main() {
  testWidgets('AuthMapper', (tester) async {
    expect(AuthMapper.toDisplayMessage("x"), "x");
  });

  testWidgets('RosterMapper', (tester) async {
    final dto = studentItem(id: "1", name: "A", location: "B", levelName: "Candidate");
    final mapped = RosterMapper.toViewModel(dto);
    expect(mapped.id, "1");
    expect(mapped.position, "Músico · Candidato(a)");
    expect(RosterMapper.toViewModels([dto]).length, 1);
  });

  testWidgets('LessonsMapper', (tester) async {
    final dto = StudentLessonsDto(approved: const [], method: const []);
    expect(LessonsMapper.toViewModel(dto).msa.isEmpty, true);
  });

  testWidgets('BackBar', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: Scaffold(body: BackBar())));
    expect(find.byIcon(Icons.arrow_back), findsOneWidget);
  });

  testWidgets('CheckpointTimeline empty', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: CheckpointTimeline(checkpoints: [])));
    expect(find.byType(SizedBox), findsWidgets);
    await tester.pumpWidget(MaterialApp(home: CheckpointTimeline(checkpoints: [CheckpointVm(label: "A", achieved: true, readyToAdvance: false, msaRequirementMet: true, methodRequirementMet: true)], filter: "msa")));
    expect(find.byIcon(Icons.check_circle), findsOneWidget);
    await tester.pumpWidget(MaterialApp(home: CheckpointTimeline(checkpoints: [
      CheckpointVm(label: "A", achieved: false, readyToAdvance: true, msaRequirementMet: true, methodRequirementMet: false),
      CheckpointVm(label: "B", achieved: false, readyToAdvance: false, msaRequirementMet: false, methodRequirementMet: true),
    ], filter: "method")));
    expect(find.byIcon(Icons.radio_button_unchecked), findsWidgets);
  });

  testWidgets('ProgressBar', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: ProgressBar(label: "Fases", percent: 50)));
    expect(find.text("Fases"), findsOneWidget);
    expect(find.text("50%"), findsOneWidget);
    await tester.pumpWidget(const MaterialApp(home: ProgressBar(label: "Lições", percent: 120)));
    expect(find.text("120%"), findsOneWidget);
    await tester.pumpWidget(const MaterialApp(home: ProgressBar(label: "Lições", percent: -10)));
    expect(find.text("-10%"), findsOneWidget);
  });

  testWidgets('InfoChip', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: InfoChip(label: "Test")));
    expect(find.text("Test"), findsOneWidget);
  });

  testWidgets('UnknownLevelBanner', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: UnknownLevelBanner(raw: "X")));
    expect(find.textContaining("nível não reconhecido"), findsOneWidget);
    expect(find.textContaining("Valor: X"), findsOneWidget);
    await tester.pumpWidget(const MaterialApp(home: UnknownLevelBanner(raw: "")));
    expect(find.textContaining("nível não reconhecido"), findsOneWidget);
  });

  testWidgets('LessonCard', (tester) async {
    final lesson = LessonItem(kind: LessonKind.msa, id: "1", date: "2025-01-15", phase: "1", page: "2", lesson: "3", clef: "Sol", description: "desc", instructor: "inst", method: "met");
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: LessonCard(lesson))));
    expect(find.text("2025-01-15"), findsOneWidget);
    expect(find.text("Fase 1"), findsOneWidget);
    expect(find.textContaining("Clave: Sol"), findsOneWidget);
  });

  testWidgets('CategoryLessonsView empty and with lessons', (tester) async {
    await tester.pumpWidget(MaterialApp(home: CategoryLessonsView(lessons: [], emptyMessage: "empty", checkpoints: [], category: LessonCategory.msa)));
    expect(find.text("empty"), findsOneWidget);
    final lesson = LessonItem(kind: LessonKind.msa, id: "1", date: "2025-01-15", phase: "", page: "", lesson: "", clef: "", description: "", instructor: "", method: "");
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: CategoryLessonsView(lessons: [lesson], emptyMessage: "empty", checkpoints: [], category: LessonCategory.msa, msaPhasePercent: 50))));
    expect(find.byType(LessonCard), findsOneWidget);
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: CategoryLessonsView(lessons: [lesson], emptyMessage: "empty", checkpoints: [], category: LessonCategory.method, methodLessonPercent: 30))));
    expect(find.byType(LessonCard), findsOneWidget);
  });

  testWidgets('StudentDetailContent', (tester) async {
    final view = StudentLessonsView(msa: [], method: []);
    const progress = ProgressViewModel(
      msaRelativePercent: 0,
      methodRelativePercent: 0,
      combinedPercent: 0,
      overallCheckpointPercent: 0,
      nextLevelLabel: '',
      meetsYouthService: false,
      meetsOfficialService: false,
      meetsOfficialization: false,
      checkpoints: [],
    );
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: StudentDetailContent(view: view, progress: progress))));
    expect(find.text("MSA (0)"), findsOneWidget);
    expect(find.text("Método (0)"), findsOneWidget);
  });
}
