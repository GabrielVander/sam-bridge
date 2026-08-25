import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/lessons/student_screen.dart';
import 'package:flutter_application/view_models.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

Widget harness(FakeSamPortal portal) {
  final presenter = LessonsCubitSignal(portal);
  final studentsPresenter = StudentsCubitSignal(portal)
    ..emit(StudentsLoaded([studentItem(id: '7')]));
  return MultiBlocSignalProvider(providers: [
    BlocSignalProvider<LessonsCubitSignal>.value(value: presenter),
    BlocSignalProvider<StudentsCubitSignal>.value(value: studentsPresenter),
  ], child: const MaterialApp(home: Scaffold(body: StudentScreen(studentId: '7'))));
}

void main() {
  testWidgets('renders MSA lessons most recent first inside the first tab',
      (tester) async {
    final portal = FakeSamPortal()
      ..lessons = StudentLessonsViewT(
        msa: [
          lessonItem(
            id: '12',
            date: '2025-06-02',
            phase: '4.5',
            page: '38 - 38',
            lesson: '7 - 8',
            clef: 'Sol',
            description: 'Estudar próximas lições.',
          ),
          lessonItem(id: '11', date: '2025-01-15', page: '30 - 34'),
        ],
        method: [lessonItem(id: 'm1', date: '2023-12-04', kind: LessonKind.method)],
      ).view;

    await tester.pumpWidget(harness(portal));
    await tester.pumpAndSettle();

    expect(find.byTooltip('Voltar'), findsOneWidget,
        reason: 'Back navigation must exist on the detail screen');
    expect(find.text('MSA (2)'), findsOneWidget);
    expect(find.text('Método (1)'), findsOneWidget);

    final newest = find.text('2025-06-02');
    final oldest = find.text('2025-01-15');
    expect(newest, findsOneWidget);
    expect(oldest, findsOneWidget);

    final newY = tester.getCenter(newest).dy;
    final oldY = tester.getCenter(oldest).dy;
    expect(newY < oldY, isTrue, reason: 'Most recent lesson must be listed first');

    expect(find.textContaining('Estudar próximas lições.'), findsOneWidget);
    expect(find.textContaining('Clave: Sol'), findsOneWidget);
  });

  testWidgets('method tab shows its own list', (tester) async {
    final portal = FakeSamPortal()
      ..lessons = StudentLessonsViewT(
        method: [
          lessonItem(
            id: 'm1',
            date: '2023-12-04',
            kind: LessonKind.method,
            page: '00',
            description: 'Postura do violino',
          ),
        ],
      ).view;

    await tester.pumpWidget(harness(portal));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Método (1)'));
    await tester.pumpAndSettle();

    expect(find.text('Postura do violino'), findsOneWidget);
    expect(find.textContaining('Pág. 00'), findsOneWidget);
  });

  testWidgets('empty lists show friendly messages', (tester) async {
    await tester.pumpWidget(harness(FakeSamPortal()));
    await tester.pumpAndSettle();

    expect(find.text('Nenhuma lição aprovada registrada.'), findsOneWidget);
    expect(find.byTooltip('Voltar'), findsOneWidget,
        reason: 'Back bar stays available even on empty/error content');
  });
}
