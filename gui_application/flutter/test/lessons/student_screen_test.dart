import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/lessons/student_screen.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';
import 'package:flutter_test/flutter_test.dart';

const _failure = RetrieveStudentLessonsOutcome.failure(
  ErrorReportDto(
    kind: ErrorKindDto.unexpectedResponse,
    details: "Unexpected status for student lessons response: 500",
  ),
);

const _emptyLessons = RetrieveStudentLessonsOutcome.success(
  StudentLessonsDto(approved: [], method: []),
);

Future<List<String>> pumpScreen(
  WidgetTester tester, {
  required List<RetrieveStudentLessonsOutcome> outcomes,
  AssessStudentProgressOutcome progress =
      const AssessStudentProgressOutcome.noInstrumentAssigned(),
}) async {
  final requestedIds = <String>[];
  final remaining = [...outcomes];
  final cubit = LessonsCubitSignal(
    retrieveStudentLessons: ({required studentId}) async {
      requestedIds.add(studentId);
      return remaining.removeAt(0);
    },
    assessStudentProgress: ({required studentId}) async => progress,
  );

  await tester.pumpWidget(
    MaterialApp(
      home: BlocSignalProvider<LessonsCubitSignal>.value(
        value: cubit,
        child: const StudentScreen(studentId: '500132', studentName: 'Jane'),
      ),
    ),
  );
  await tester.pumpAndSettle();
  return requestedIds;
}

void main() {
  group('StudentScreen failure', () {
    testWidgets('shows the friendly message with the details collapsed', (
      tester,
    ) async {
      await pumpScreen(tester, outcomes: [_failure]);

      expect(
        find.textContaining('O SAM respondeu de forma inesperada'),
        findsOneWidget,
      );
      expect(find.text('Detalhes técnicos'), findsOneWidget);
      expect(find.textContaining('Unexpected status'), findsNothing);
    });

    testWidgets('retrying reloads the same student', (tester) async {
      final requestedIds = await pumpScreen(
        tester,
        outcomes: [_failure, _emptyLessons],
      );

      await tester.tap(find.text('Tentar novamente'));
      await tester.pumpAndSettle();

      expect(requestedIds, ['500132', '500132']);
      expect(find.text('Tentar novamente'), findsNothing);
      expect(find.textContaining('Instrumento ainda não definido'), findsOne);
    });
  });

  group('StudentScreen progress notices', () {
    testWidgets(
      'a progress failure keeps the lessons and shows a compact notice with collapsed details',
      (tester) async {
        await pumpScreen(
          tester,
          outcomes: [_emptyLessons],
          progress: const AssessStudentProgressOutcome.failure(
            ErrorReportDto(
              kind: ErrorKindDto.sessionExpired,
              details: 'Session expired',
            ),
          ),
        );

        expect(find.textContaining('Progresso indisponível'), findsOneWidget);
        expect(
          find.textContaining('Sua sessão expirou. Entre novamente.'),
          findsOneWidget,
        );
        expect(find.text('Detalhes técnicos'), findsOneWidget);
        expect(find.text('Session expired'), findsNothing);
        expect(find.text('Tentar novamente'), findsNothing);
      },
    );

    testWidgets('a non-musician gets a friendly notice, not a failure', (
      tester,
    ) async {
      await pumpScreen(
        tester,
        outcomes: [_emptyLessons],
        progress: const AssessStudentProgressOutcome.notAMusician(),
      );

      expect(
        find.textContaining('O progresso só é calculado para músicos'),
        findsOneWidget,
      );
      expect(find.text('Detalhes técnicos'), findsNothing);
      expect(find.textContaining('Progresso indisponível'), findsNothing);
    });
  });
}
