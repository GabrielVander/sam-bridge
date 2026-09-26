import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/shared/level_name.dart';
import 'package:flutter_application/widgets/progress_bar.dart';

final class CheckpointTimeline extends StatelessWidget {
  final ProgressView progress;

  const CheckpointTimeline({super.key, required this.progress});

  @override
  Widget build(BuildContext context) {
    final nextLevel = progress.nextLevel;

    return Card(
      margin: EdgeInsets.zero,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              context.l10n.progressTitle,
              style: Theme.of(context).textTheme.titleMedium,
            ),
            const SizedBox(height: 16),
            _Timeline(checkpoints: progress.checkpoints),
            const SizedBox(height: 20),
            if (nextLevel != null) ...[
              Text(
                context.l10n.progressTowards(
                  context.l10n.reportedLevelName(nextLevel),
                ),
                style: Theme.of(context).textTheme.labelLarge,
              ),
              const SizedBox(height: 10),
              ProgressBar(
                label: context.l10n.msa,
                percent: progress.msaRelativePercent,
              ),
              const SizedBox(height: 6),
              ProgressBar(
                label: context.l10n.method,
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
                    context.l10n.progressAllLevelsReached,
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
    final label = context.l10n.reportedLevelName(checkpoint.level);
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
          ? context.l10n.checkpointReady(label)
          : label,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, color: color, size: 26),
          const SizedBox(height: 4),
          SizedBox(
            width: 68,
            child: Text(
              label,
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
