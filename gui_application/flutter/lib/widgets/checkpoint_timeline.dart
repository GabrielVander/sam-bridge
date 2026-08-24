import 'package:flutter/material.dart';
import 'package:flutter_application/view_models.dart';

final class CheckpointTimeline extends StatelessWidget {
  final List<CheckpointVm> checkpoints;
  final String? filter;

  const CheckpointTimeline({super.key, required this.checkpoints, this.filter});

  @override
  Widget build(BuildContext context) {
    if (checkpoints.isEmpty) return const SizedBox.shrink();

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: List.generate(checkpoints.length * 2 - 1, (i) {
          if (i.isOdd) {
            final leftCp = checkpoints[(i - 1) ~/ 2];
            final rightCp = checkpoints[(i + 1) ~/ 2];
            final bothMet = _met(leftCp, filter) && _met(rightCp, filter);
            return Expanded(
              child: Container(
                height: 2,
                margin: const EdgeInsets.only(top: 10),
                color: bothMet ? Colors.green : Theme.of(context).dividerColor,
              ),
            );
          }
          final cp = checkpoints[i ~/ 2];
          final met = _met(cp, filter);
          final color = cp.achieved
              ? Colors.green
              : cp.readyToAdvance
                  ? Colors.orange
                  : Colors.grey;

          return Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                met ? Icons.check_circle : Icons.radio_button_unchecked,
                size: 20,
                color: color,
              ),
              const SizedBox(height: 2),
              SizedBox(
                width: 64,
                child: Text(
                  cp.label,
                  textAlign: TextAlign.center,
                  style: Theme.of(context).textTheme.labelSmall?.copyWith(color: color),
                ),
              ),
            ],
          );
        }),
      ),
    );
  }

  static bool _met(CheckpointVm cp, String? filter) {
    if (cp.achieved) return true;
    return switch (filter) {
      'msa' => cp.msaRequirementMet,
      'method' => cp.methodRequirementMet,
      _ => cp.readyToAdvance,
    };
  }
}
