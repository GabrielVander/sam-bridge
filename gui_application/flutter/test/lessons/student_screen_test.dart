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

  group('StudentScreen lessons', () {
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
