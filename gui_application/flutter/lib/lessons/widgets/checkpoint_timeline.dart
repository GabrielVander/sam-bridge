import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/widgets/progress_bar.dart';

final class CheckpointTimeline extends StatelessWidget {
  final ProgressView progress;

  const CheckpointTimeline({super.key, required this.progress});

  @override
  Widget build(BuildContext context) {
    final nextLevelLabel = progress.nextLevelLabel;

    return Card(
      margin: EdgeInsets.zero,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Progresso', style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 16),
            _Timeline(checkpoints: progress.checkpoints),
            const SizedBox(height: 20),
            if (nextLevelLabel != null) ...[
              Text(
                'Rumo a: $nextLevelLabel',
                style: Theme.of(context).textTheme.labelLarge,
              ),
              const SizedBox(height: 10),
              ProgressBar(label: 'MSA', percent: progress.msaRelativePercent),
              const SizedBox(height: 6),
              ProgressBar(
                label: 'Método',
                percent: progress.methodRelativePercent,
              ),
            ] else
              Row(
                children: [
                  Icon(
                    Icons.emoji_events,
                    color: Colors.amber.shade700,
                    size: 20,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    'Todos os níveis alcançados',
                    style: Theme.of(context).textTheme.labelLarge,
                  ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

final class _Timeline extends StatelessWidget {
  final List<CheckpointView> checkpoints;

  const _Timeline({required this.checkpoints});

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        for (var i = 0; i < checkpoints.length; i++) ...[
          if (i > 0)
            Expanded(
              child: Padding(
                padding: const EdgeInsets.only(bottom: 22),
                child: _Connector(filled: checkpoints[i].achieved),
              ),
            ),
          _CheckpointDot(checkpoint: checkpoints[i]),
        ],
      ],
    );
  }
}

final class _Connector extends StatelessWidget {
  final bool filled;

  const _Connector({required this.filled});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 2,
      color: filled
          ? Theme.of(context).colorScheme.primary
          : Theme.of(context).colorScheme.surfaceContainerHighest,
    );
  }
}

final class _CheckpointDot extends StatelessWidget {
  final CheckpointView checkpoint;

  const _CheckpointDot({required this.checkpoint});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final (icon, color) = switch (checkpoint) {
      CheckpointView(achieved: true) => (
        Icons.check_circle,
        colorScheme.primary,
      ),
      CheckpointView(readyToAdvance: true) => (
        Icons.star,
        Colors.amber.shade700,
      ),
      _ => (Icons.radio_button_unchecked, colorScheme.outline),
    };

    return Tooltip(
      message: checkpoint.readyToAdvance
          ? '${checkpoint.label} - pronto para a prova'
          : checkpoint.label,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, color: color, size: 26),
          const SizedBox(height: 4),
          SizedBox(
            width: 68,
            child: Text(
              checkpoint.label,
              textAlign: TextAlign.center,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
              style: Theme.of(context).textTheme.labelSmall,
            ),
          ),
        ],
      ),
    );
  }
}
