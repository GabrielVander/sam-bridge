import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/widgets/checkpoint_timeline.dart';
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
              checkpoint(label: 'Ensaio', achieved: true),
              checkpoint(label: 'Culto'),
              checkpoint(label: 'Oficialização'),
            ],
          ),
        ),
      );

      expect(find.text('Progresso'), findsOneWidget);
      expect(find.text('Ensaio'), findsOneWidget);
      expect(find.text('Culto'), findsOneWidget);
      expect(find.text('Oficialização'), findsOneWidget);
    });

    testWidgets('marks achieved, ready and pending checkpoints differently', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(label: 'Ensaio', achieved: true),
              checkpoint(label: 'Culto', readyToAdvance: true),
              checkpoint(label: 'Oficialização'),
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
              checkpoint(label: 'Ensaio', achieved: true),
              checkpoint(label: 'Culto', readyToAdvance: true),
            ],
          ),
        ),
      );

      expect(find.byTooltip('Culto - pronto para a prova'), findsOneWidget);
      expect(find.byTooltip('Ensaio'), findsOneWidget);
    });

    testWidgets('joins consecutive checkpoints with a connector that is filled '
        'only once the later one is achieved', (tester) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(
          progress: progressView(
            checkpoints: [
              checkpoint(label: 'Ensaio', achieved: true),
              checkpoint(label: 'Culto', achieved: true),
              checkpoint(label: 'Oficialização'),
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
            nextLevelLabel: 'Culto',
            msaRelativePercent: 40,
            methodRelativePercent: 75,
          ),
        ),
      );

      expect(find.text('Rumo a: Culto'), findsOneWidget);
      expect(find.text('40%'), findsOneWidget);
      expect(find.text('75%'), findsOneWidget);
      expect(find.byType(ProgressBar), findsNWidgets(2));
      expect(find.text('Todos os níveis alcançados'), findsNothing);
    });

    testWidgets('celebrates once every level has been reached', (tester) async {
      await pumpInApp(
        tester,
        CheckpointTimeline(progress: progressView(nextLevelLabel: null)),
      );

      expect(find.text('Todos os níveis alcançados'), findsOneWidget);
      expect(find.byIcon(Icons.emoji_events), findsOneWidget);
      expect(find.byType(ProgressBar), findsNothing);
    });
  });
}
