import 'package:flutter/material.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/roster.dart';

void main() {
  group('StudentsScreen row', () {
    testWidgets('shows the instrument with a music note icon', (tester) async {
      await pumpRoster(tester, [
        studentSummary(instrumentName: 'SAXOFONE TENOR'),
      ]);

      expect(find.text('Saxofone tenor'), findsOneWidget);
      expect(find.byIcon(Icons.music_note_outlined), findsOneWidget);
    });

    testWidgets('puts the instrument between position and location', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(instrumentName: 'OBOÉ')]);

      final position = tester.getTopLeft(find.text('Ensaio')).dy;
      final instrument = tester.getTopLeft(find.text('Oboé')).dy;
      final location = tester.getTopLeft(find.text('Some Location')).dy;

      expect(position, lessThan(instrument));
      expect(instrument, lessThan(location));
    });

    testWidgets('shows no instrument line when there is no instrument', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary()]);

      expect(find.byIcon(Icons.music_note_outlined), findsNothing);
      expect(find.text('Ensaio'), findsOneWidget);
      expect(find.text('Some Location'), findsOneWidget);
    });

    testWidgets('keeps a very long instrument name on one line', (
      tester,
    ) async {
      tester.view.physicalSize = const Size(360, 800);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.reset);
      final longName = List.filled(12, 'INSTRUMENTO').join(' ');

      await pumpRoster(tester, [studentSummary(instrumentName: longName)]);

      final text = tester.widget<Text>(find.textContaining('Instrumento'));
      expect(text.maxLines, 1);
      expect(text.overflow, TextOverflow.ellipsis);
      expect(tester.takeException(), isNull);
    });

    testWidgets('hides the decorative icon from screen readers', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(instrumentName: 'VIOLINO')]);

      expect(
        find.ancestor(
          of: find.byIcon(Icons.music_note_outlined),
          matching: find.byType(ExcludeSemantics),
        ),
        findsOneWidget,
      );
    });
  });

  group('StudentsScreen navigation', () {
    testWidgets('tapping a student opens their page, passing the name along', (
      tester,
    ) async {
      await pumpRoster(tester, [
        studentSummary(id: '500132', name: 'Jane Doe'),
      ]);

      await tester.tap(find.text('Jane Doe'));
      await tester.pumpAndSettle();

      expect(find.text('página do aluno 500132 (Jane Doe)'), findsOneWidget);
    });

    testWidgets('a student without an id cannot be opened', (tester) async {
      await pumpRoster(tester, [studentSummary(id: '', name: 'Jane Doe')]);

      await tester.tap(find.text('Jane Doe'));
      await tester.pumpAndSettle();

      expect(find.textContaining('página do aluno'), findsNothing);
      expect(find.byIcon(Icons.chevron_right), findsNothing);
    });

    testWidgets('a student with an id shows that the row can be opened', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(id: '1')]);

      expect(find.byIcon(Icons.chevron_right), findsOneWidget);
    });
  });

  group('StudentsScreen avatar', () {
    testWidgets('shows the capitalised first letter of the name', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(name: 'jane doe')]);

      expect(find.text('J'), findsOneWidget);
    });

    testWidgets('shows a question mark when the name is empty', (tester) async {
      await pumpRoster(tester, [studentSummary(name: '')]);

      expect(find.text('?'), findsOneWidget);
    });
  });

  group('StudentsScreen failure', () {
    const failure = RetrieveAllAvailableStudentsOutcome.failure(
      ErrorReportDto(
        kind: ErrorKindDto.network,
        details: 'Request failed for operation dashboard',
      ),
    );

    Future<void> pumpFailingRoster(
      WidgetTester tester, {
      required List<RetrieveAllAvailableStudentsOutcome> outcomes,
    }) => pumpStudents(tester, presenterAnswering(outcomes));

    testWidgets('shows the friendly message with the details collapsed', (
      tester,
    ) async {
      await pumpFailingRoster(tester, outcomes: [failure]);

      expect(
        find.textContaining('Não foi possível conectar ao SAM'),
        findsOneWidget,
      );
      expect(find.text('Detalhes técnicos'), findsOneWidget);
      expect(find.text('Request failed for operation dashboard'), findsNothing);
    });

    testWidgets('retrying reloads the students', (tester) async {
      await pumpFailingRoster(
        tester,
        outcomes: [
          failure,
          RetrieveAllAvailableStudentsOutcome.success([studentSummary()]),
        ],
      );

      await tester.tap(find.text('Tentar novamente'));
      await tester.pumpAndSettle();

      expect(find.text('Jane Doe'), findsOneWidget);
      expect(find.text('Tentar novamente'), findsNothing);
    });
  });
}
