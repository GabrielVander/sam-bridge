import 'package:flutter/material.dart';

final class CheckpointTimeline extends StatelessWidget {
  final String? filter;
  final double? msaRelative;
  final double? methodRelative;
  final double? combinedPercent;
  final double? overallCheckpointPercent;

  const CheckpointTimeline({
    super.key,
    this.filter,
    this.msaRelative,
    this.methodRelative,
    this.combinedPercent,
    this.overallCheckpointPercent,
  });

  @override
  Widget build(BuildContext context) {
    final overall = overallCheckpointPercent;
    final msa = msaRelative;
    final method = methodRelative;
    final combined = combinedPercent;

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        if (overall != null)
          Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: Row(
              children: [
                const SizedBox(width: 4),
                const Icon(Icons.flag, size: 14),
                const SizedBox(width: 6),
                Expanded(
                  child: LinearProgressIndicator(
                    value: (overall / 100).clamp(0.0, 1.0),
                    backgroundColor: Theme.of(
                      context,
                    ).colorScheme.surfaceContainerHighest,
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  '${overall.toStringAsFixed(0)}%',
                  style: Theme.of(context).textTheme.labelSmall,
                ),
              ],
            ),
          ),
        Padding(
          padding: const EdgeInsets.symmetric(vertical: 4),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: List.generate(0, (i) {
              return FlutterLogo();
            }),
          ),
        ),
        if (msa != null || method != null)
          Padding(
            padding: const EdgeInsets.only(top: 6),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                if (msa != null) ...[
                  const Icon(Icons.music_note, size: 12),
                  const SizedBox(width: 2),
                  Text(
                    'MSA ${msa.toStringAsFixed(0)}%',
                    style: Theme.of(context).textTheme.labelSmall,
                  ),
                  const SizedBox(width: 12),
                ],
                if (method != null) ...[
                  const Icon(Icons.menu_book, size: 12),
                  const SizedBox(width: 2),
                  Text(
                    'Método ${method.toStringAsFixed(0)}%',
                    style: Theme.of(context).textTheme.labelSmall,
                  ),
                ],
              ],
            ),
          ),
      ],
    );
  }
}
