import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/widgets/checkpoint_timeline.dart';
import 'package:flutter_application/shared/level.dart';
import 'package:flutter_application/widgets/progress_bar.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../support/builders.dart';

void main() {
  group('CheckpointTimeline', () {
    testWidgets('lists every checkpoint of the journey by label', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(
                level: const KnownLevel(Level.practice),
                achieved: true,
              ),
              checkpoint(level: const KnownLevel(Level.officialService)),
              checkpoint(level: const KnownLevel(Level.officialized)),
            ],
          ),
        ),
      );

      expect(find.text('Progresso'), findsOneWidget);
      expect(find.text('Ensaio'), findsOneWidget);
      expect(find.text('Culto Oficial'), findsOneWidget);
      expect(find.text('Oficialização'), findsOneWidget);
    });

    const levelNames = {
      Level.candidate: 'Candidato(a)',
      Level.practice: 'Ensaio',
      Level.youthService: 'Reunião de Jovens e Menores',
      Level.officialService: 'Culto Oficial',
      Level.officialized: 'Oficialização',
      Level.halfHour: 'Meia Hora',
    };

    for (final MapEntry(key: level, value: name) in levelNames.entries) {
      testWidgets('names the ${level.name} level', (tester) async {
        await pumpInApp(
          tester,
          CheckpointTimeline(
            progress: progressView(
              checkpoints: [checkpoint(level: KnownLevel(level))],
              nextLevel: null,
            ),
          ),
        );

        expect(find.text(name), findsOneWidget);
      });
    }

    testWidgets('shows what SAM wrote for a level it does not recognize', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(level: const UnrecognizedLevel('SomethingNew')),
            ],
            nextLevel: null,
          ),
        ),
      );

      expect(find.text('SomethingNew'), findsOneWidget);
    });

    testWidgets('marks achieved, ready and pending checkpoints differently', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(
                level: const KnownLevel(Level.practice),
                achieved: true,
              ),
              checkpoint(
                level: const KnownLevel(Level.officialService),
                readyToAdvance: true,
              ),
              checkpoint(level: const KnownLevel(Level.officialized)),
            ],
          ),
        ),
      );

      expect(find.byIcon(Icons.check_circle), findsOneWidget);
      expect(find.byIcon(Icons.star), findsOneWidget);
      expect(find.byIcon(Icons.radio_button_unchecked), findsOneWidget);
    });

    testWidgets('says which checkpoints are ready for the exam in a tooltip', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(
                level: const KnownLevel(Level.practice),
                achieved: true,
              ),
              checkpoint(
                level: const KnownLevel(Level.officialService),
                readyToAdvance: true,
              ),
            ],
          ),
        ),
      );

      expect(
        find.byTooltip('Culto Oficial - pronto para a prova'),
        findsOneWidget,
      );
      expect(find.byTooltip('Ensaio'), findsOneWidget);
    });

    testWidgets('joins consecutive checkpoints with a connector that is filled '
        'only once the later one is achieved', (tester) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(
                level: const KnownLevel(Level.practice),
                achieved: true,
              ),
              checkpoint(
                level: const KnownLevel(Level.officialService),
                achieved: true,
              ),
              checkpoint(level: const KnownLevel(Level.officialized)),
            ],
          ),
        ),
      );

      final scheme = Theme.of(
        tester.element(find.byType(Scaffold)),
      ).colorScheme;
      bool isConnector(Widget w, Color color) =>
          w is Container && w.color == color && w.constraints?.maxHeight == 2;

      expect(
        find.byWidgetPredicate((w) => isConnector(w, scheme.primary)),
        findsOneWidget,
      );
      expect(
        find.byWidgetPredicate(
          (w) => isConnector(w, scheme.surfaceContainerHighest),
        ),
        findsOneWidget,
      );
    });

    testWidgets('shows progress towards the next level while there is one', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            nextLevel: const KnownLevel(Level.officialService),
            msaRelativePercent: 40,
            methodRelativePercent: 75,
          ),
        ),
      );

      expect(find.text('Rumo a: Culto Oficial'), findsOneWidget);
      expect(find.text('40%'), findsOneWidget);
      expect(find.text('75%'), findsOneWidget);
      expect(find.byType(ProgressBar), findsNWidgets(2));
      expect(find.text('Todos os níveis alcançados'), findsNothing);
    });

    testWidgets('names the levels in English', (tester) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(
                level: const KnownLevel(Level.youthService),
                readyToAdvance: true,
              ),
            ],
          ),
        ),
        locale: const Locale('en'),
      );

      expect(find.text('Progress'), findsOneWidget);
      expect(find.text('Youth Meeting'), findsOneWidget);
      expect(
        find.byTooltip('Youth Meeting - ready for the exam'),
        findsOneWidget,
      );
      expect(find.text('Next: Official Service'), findsOneWidget);
    });

    testWidgets('celebrates once every level has been reached', (tester) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(progress: progressView(nextLevel: null)),
      );

      expect(find.text('Todos os níveis alcançados'), findsOneWidget);
      expect(find.byIcon(Icons.emoji_events), findsOneWidget);
      expect(find.byType(ProgressBar), findsNothing);
    });
  });
}
