import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/errors.dart';
import '../support/lessons.dart';

void main() {
  group('StudentScreen progress', () {
    testWidgets('explains that the student has no instrument yet', (
      tester,
    ) async {
      await pumpStudent(tester, progress: noInstrumentAssigned());

      expect(
        find.text(
          'Instrumento ainda não definido para este aluno no SAM. '
          'O progresso não pode ser calculado.',
        ),
        findsOneWidget,
      );
    });

    testWidgets('explains that progress is only for musicians', (tester) async {
      await pumpStudent(tester, progress: notAMusician());

      expect(
        find.text(
          'O progresso só é calculado para músicos; '
          'este aluno tem outra função no SAM.',
        ),
        findsOneWidget,
      );
    });

    testWidgets('explains why progress is unavailable, with the details', (
      tester,
    ) async {
      await pumpStudent(
        tester,
        progress: progressFailed(sessionExpiredFailure('Session expired')),
      );

      expect(
        find.text(
          'Progresso indisponível. Sua sessão expirou. Entre novamente.',
        ),
        findsOneWidget,
      );
      expect(find.text('Detalhes técnicos'), findsOneWidget);
    });

    testWidgets('explains why progress is unavailable in English', (
      tester,
    ) async {
      await pumpStudent(
        tester,
        progress: progressFailed(sessionExpiredFailure('Session expired')),
        locale: const Locale('en'),
      );

      expect(
        find.text(
          'Progress unavailable. '
          'Your session has expired. Please sign in again.',
        ),
        findsOneWidget,
      );
    });
  });

  group('StudentScreen checkpoints', () {
    testWidgets('shows every level, marking the one ready for the exam', (
      tester,
    ) async {
      await pumpStudent(
        tester,
        progress: progressAssessed(
          progressAssessment(
            checkpoints: [
              achievedCheckpoint(Levels.youthService),
              readyForExamCheckpoint(Levels.officialService),
              pendingCheckpoint(Levels.unknown('EXÓTICO')),
            ],
            nextLevel: Levels.officialService,
          ),
        ),
      );

      expect(find.text('Reunião de Jovens e Menores'), findsOneWidget);
      expect(
        find.byTooltip('Culto Oficial - pronto para a prova'),
        findsOneWidget,
      );
      expect(find.text('EXÓTICO'), findsOneWidget);
    });

    testWidgets('marks an achieved level as achieved', (tester) async {
      await pumpStudent(
        tester,
        progress: progressAssessed(
          progressAssessment(
            checkpoints: [achievedCheckpoint(Levels.youthService)],
            nextLevel: Levels.officialService,
          ),
        ),
      );

      expect(find.byTooltip('Reunião de Jovens e Menores'), findsOneWidget);
      expect(find.byIcon(Icons.check_circle), findsOneWidget);
    });
  });

  group('StudentScreen lessons', () {
    testWidgets('shows the date, a phase range and a single page', (
      tester,
    ) async {
      await pumpStudent(
        tester,
        lessons: [
          lessonsRetrieved(
            studentLessons(
              msa: [
                lesson(
                  id: '1',
                  date: (2025, 9, 9),
                  phase: ('3.4', '4.1'),
                  page: ('38', '38'),
                ),
              ],
            ),
          ),
        ],
      );

      expect(find.text('09/09/2025'), findsOneWidget);
      expect(find.text('Fase 3.4 - 4.1'), findsOneWidget);
      expect(find.text('Pág. 38'), findsOneWidget);
    });

    testWidgets('explains a failure of an unknown kind generically', (
      tester,
    ) async {
      await pumpStudent(
        tester,
        lessons: [lessonsFailed(unknownFailure('boom'))],
      );

      expect(find.text('Algo deu errado. Tente novamente.'), findsOneWidget);
    });

    testWidgets('loads the lessons again when retrying after a failure', (
      tester,
    ) async {
      await pumpStudent(
        tester,
        lessons: [lessonsFailed(networkFailure('connection refused'))],
      );
      expect(find.text('Tentar novamente'), findsOneWidget);

      await tester.tap(find.text('Tentar novamente'));
      await tester.pumpAndSettle();

      expect(find.text('Tentar novamente'), findsNothing);
      expect(find.text('MSA (0)'), findsOneWidget);
    });
  });
}
