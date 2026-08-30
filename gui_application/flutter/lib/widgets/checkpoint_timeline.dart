import 'package:flutter/material.dart';

final class CheckpointTimeline extends StatelessWidget {
  // final List<CheckpointVm> checkpoints;
  final String? filter;
  final double? msaRelative;
  final double? methodRelative;
  final double? combinedPercent;
  final double? overallCheckpointPercent;

  const CheckpointTimeline({
    super.key,
    // required this.checkpoints,
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
            children: List.generate(/*checkpoints.length * 2 - 1*/ 0, (i) {
              return FlutterLogo();
              // if (i.isOdd) {
              //   final leftCp = checkpoints[(i - 1) ~/ 2];
              //   final rightCp = checkpoints[(i + 1) ~/ 2];
              //   final bothMet = _met(leftCp, filter) && _met(rightCp, filter);
              //   final isPastOverall =
              //       overall != null && ((i ~/ 2 + 1) * 20 <= overall);
              //   return Expanded(
              //     child: Container(
              //       height: 2,
              //       margin: const EdgeInsets.only(top: 10),
              //       color: isPastOverall
              //           ? Colors.green
              //           : bothMet
              //           ? Colors.green
              //           : Theme.of(context).dividerColor,
              //     ),
              //   );
              // }
              // final cp = checkpoints[i ~/ 2];
              // final met = _met(cp, filter);
              // final color = cp.achieved
              //     ? Colors.green
              //     : cp.readyToAdvance
              //     ? Colors.orange
              //     : Colors.grey;
              //
              // return Column(
              //   mainAxisSize: MainAxisSize.min,
              //   children: [
              //     Icon(
              //       met ? Icons.check_circle : Icons.radio_button_unchecked,
              //       size: 20,
              //       color: color,
              //     ),
              //     const SizedBox(height: 2),
              //     SizedBox(
              //       width: 64,
              //       child: Text(
              //         cp.label,
              //         textAlign: TextAlign.center,
              //         style: Theme.of(
              //           context,
              //         ).textTheme.labelSmall?.copyWith(color: color),
              //       ),
              //     ),
              //     if (msa != null || method != null || combined != null)
              //       Padding(
              //         padding: const EdgeInsets.only(top: 4),
              //         child: SizedBox(
              //           width: 64,
              //           child: Column(
              //             children: [
              //               if (msa != null)
              //                 LinearProgressIndicator(
              //                   value: (msa / 100).clamp(0.0, 1.0),
              //                   minHeight: 3,
              //                   backgroundColor: Theme.of(
              //                     context,
              //                   ).colorScheme.surfaceContainerHighest,
              //                 ),
              //               if (msa != null && method != null)
              //                 const SizedBox(height: 2),
              //               if (method != null)
              //                 LinearProgressIndicator(
              //                   value: (method / 100).clamp(0.0, 1.0),
              //                   minHeight: 3,
              //                   backgroundColor: Theme.of(
              //                     context,
              //                   ).colorScheme.surfaceContainerHighest,
              //                 ),
              //               if (combined != null) ...[
              //                 const SizedBox(height: 2),
              //                 LinearProgressIndicator(
              //                   value: (combined / 100).clamp(0.0, 1.0),
              //                   minHeight: 3,
              //                   color: Theme.of(context).colorScheme.primary,
              //                   backgroundColor: Theme.of(
              //                     context,
              //                   ).colorScheme.surfaceContainerHighest,
              //                 ),
              //               ],
              //             ],
              //           ),
              //         ),
              //       ),
              //   ],
              // );
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

  // static bool _met(CheckpointVm cp, String? filter) {
  //   if (cp.achieved) return true;
  //   return switch (filter) {
  //     'msa' => cp.msaRequirementMet,
  //     'method' => cp.methodRequirementMet,
  //     _ => cp.readyToAdvance,
  //   };
  // }
}
