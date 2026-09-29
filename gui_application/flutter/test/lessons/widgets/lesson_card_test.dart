import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/widgets/lesson_card.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../support/builders.dart';

void main() {
  group('LessonCard', () {
    testWidgets('shows every detail of a complete lesson', (tester) async {
      await pumpInApp(tester, LessonCard(lessonItem()));

      expect(find.text('01/02/2024'), findsOneWidget);
      expect(find.text('Fase 2'), findsOneWidget);
      expect(find.text('Pág. 10'), findsOneWidget);
      expect(find.text('Lição 3'), findsOneWidget);
      expect(find.text('Clave: Sol'), findsOneWidget);
      expect(find.text('Escalas maiores'), findsOneWidget);
      expect(find.text('Maria Souza'), findsOneWidget);
    });

    const clefNames = {Clef.g: 'Sol', Clef.c: 'Dó', Clef.f: 'Fá'};

    for (final MapEntry(key: clef, value: name) in clefNames.entries) {
      testWidgets('names the ${clef.name} clef', (tester) async {
        await pumpInApp(tester, LessonCard(lessonItem(clef: clef)));

        expect(find.text('Clave: $name'), findsOneWidget);
      });
    }

    testWidgets('shows a lesson in English', (tester) async {
      await pumpInApp(
        tester,
        LessonCard(lessonItem()),
        locale: const Locale('en'),
      );

      expect(find.text('2/1/2024'), findsOneWidget);
      expect(find.text('Phase 2'), findsOneWidget);
      expect(find.text('p. 10'), findsOneWidget);
      expect(find.text('Lesson 3'), findsOneWidget);
      expect(find.text('G clef'), findsOneWidget);
    });

    testWidgets('shows a dash when the date is unknown', (tester) async {
      await pumpInApp(tester, LessonCard(lessonItem(date: null)));

      expect(find.text('—'), findsOneWidget);
    });

    testWidgets('leaves out the chips whose value is empty', (tester) async {
      await pumpInApp(
        tester,
        LessonCard(lessonItem(phase: '', page: '', lesson: '')),
      );

      expect(find.textContaining('Fase'), findsNothing);
      expect(find.textContaining('Pág.'), findsNothing);
      expect(find.textContaining('Lição'), findsNothing);
    });

    testWidgets(
      'leaves out the description block when it has nothing to show',
      (tester) async {
        await pumpInApp(
          tester,
          LessonCard(lessonItem(clef: null, description: '', instructor: '')),
        );

        expect(find.textContaining('Clave'), findsNothing);
        expect(find.text('Escalas maiores'), findsNothing);
        expect(find.text('Maria Souza'), findsNothing);
        expect(find.text('01/02/2024'), findsOneWidget);
      },
    );

    testWidgets('shows only the parts of the description that exist', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        LessonCard(
          lessonItem(clef: null, description: 'Escalas', instructor: ''),
        ),
      );

      expect(find.text('Escalas'), findsOneWidget);
      expect(find.textContaining('Clave'), findsNothing);
    });

    testWidgets('shows the instructor even without a clef or description', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        LessonCard(lessonItem(clef: null, description: '')),
      );

      expect(find.text('Maria Souza'), findsOneWidget);
    });

    testWidgets('tells MSA lessons apart from method lessons by icon', (
      tester,
    ) async {
      await pumpInApp(tester, LessonCard(lessonItem(kind: LessonKind.msa)));
      expect(find.byIcon(Icons.menu_book), findsOneWidget);
      expect(find.byIcon(Icons.music_note), findsNothing);

      await pumpInApp(tester, LessonCard(lessonItem(kind: LessonKind.method)));
      expect(find.byIcon(Icons.music_note), findsOneWidget);
      expect(find.byIcon(Icons.menu_book), findsNothing);
    });
  });
}
