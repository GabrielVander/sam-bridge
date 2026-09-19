import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/roster/students_screen.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';
import 'package:flutter_test/flutter_test.dart';

Future<void> pumpRoster(
  WidgetTester tester,
  List<StudentSummaryDto> students,
) async {
  final presenter = StudentsPresenter(
    retrieveStudents: () async =>
        RetrieveAllAvailableStudentsOutcome.success(students),
  );

  await tester.pumpWidget(
    MaterialApp(
      home: Scaffold(
        body: BlocSignalProvider<StudentsPresenter>.value(
          value: presenter,
          child: const StudentsScreen(),
        ),
      ),
    ),
  );
  await tester.pumpAndSettle();
}

StudentSummaryDto student({String? instrumentName}) => StudentSummaryDto(
  id: '1',
  name: 'Jane Doe',
  position: const StudentPositionDto.practice(),
  location: 'Some Location',
  instrumentName: instrumentName,
);

void main() {
  group('StudentsScreen row', () {
    testWidgets('shows the instrument with a music note icon', (tester) async {
      await pumpRoster(tester, [student(instrumentName: 'SAXOFONE TENOR')]);

      expect(find.text('Saxofone tenor'), findsOneWidget);
      expect(find.byIcon(Icons.music_note_outlined), findsOneWidget);
    });

    testWidgets('puts the instrument between position and location', (
      tester,
    ) async {
      await pumpRoster(tester, [student(instrumentName: 'OBOÉ')]);

      final position = tester.getTopLeft(find.text('Ensaio')).dy;
      final instrument = tester.getTopLeft(find.text('Oboé')).dy;
      final location = tester.getTopLeft(find.text('Some Location')).dy;

      expect(position, lessThan(instrument));
      expect(instrument, lessThan(location));
    });

    testWidgets('shows no instrument line when there is no instrument', (
      tester,
    ) async {
      await pumpRoster(tester, [student()]);

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

      await pumpRoster(tester, [student(instrumentName: longName)]);

      final text = tester.widget<Text>(find.textContaining('Instrumento'));
      expect(text.maxLines, 1);
      expect(text.overflow, TextOverflow.ellipsis);
      expect(tester.takeException(), isNull);
    });

    testWidgets('hides the decorative icon from screen readers', (
      tester,
    ) async {
      await pumpRoster(tester, [student(instrumentName: 'VIOLINO')]);

      expect(
        find.ancestor(
          of: find.byIcon(Icons.music_note_outlined),
          matching: find.byType(ExcludeSemantics),
        ),
        findsOneWidget,
      );
    });
  });

  group('StudentsScreen failure', () {
    const failure = RetrieveAllAvailableStudentsOutcome.failure(
      ErrorReportDto(
        kind: ErrorKindDto.network,
        details: 'Request failed for operation dashboard',
      ),
    );

    Future<StudentsPresenter> pumpFailingRoster(
      WidgetTester tester, {
      required List<RetrieveAllAvailableStudentsOutcome> outcomes,
    }) async {
      final remaining = [...outcomes];
      final presenter = StudentsPresenter(
        retrieveStudents: () async => remaining.removeAt(0),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: BlocSignalProvider<StudentsPresenter>.value(
              value: presenter,
              child: const StudentsScreen(),
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      return presenter;
    }

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
          RetrieveAllAvailableStudentsOutcome.success([student()]),
        ],
      );

      await tester.tap(find.text('Tentar novamente'));
      await tester.pumpAndSettle();

      expect(find.text('Jane Doe'), findsOneWidget);
      expect(find.text('Tentar novamente'), findsNothing);
    });
  });
}
