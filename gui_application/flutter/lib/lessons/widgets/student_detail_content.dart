import 'package:flutter/material.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/widgets/checkpoint_timeline.dart';
import 'category_lessons_view.dart';

final class StudentDetailContent extends StatelessWidget {
  final StudentLessonsView view;
  // final ProgressViewModel progress;

  const StudentDetailContent({
    super.key,
    required this.view,
    // required this.progress,
  });

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2,
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            child: CheckpointTimeline(
              // checkpoints: progress.checkpoints,
              filter: null,
              msaRelative: 0.0 /*progress.msaRelativePercent*/,
              methodRelative: 0.0 /*progress.methodRelativePercent*/,
              combinedPercent: 0.0 /*progress.combinedPercent*/,
              overallCheckpointPercent:
                  0.0 /*progress.overallCheckpointPercent*/,
            ),
          ),
          // if (progress.nextLevelLabel.isNotEmpty)
          //   Padding(
          //     padding: const EdgeInsets.only(bottom: 4),
          //     child: Text(
          //       'Próximo: ${progress.nextLevelLabel}',
          //       style: Theme.of(context).textTheme.labelSmall,
          //     ),
          //   ),
          // TabBar(
          //   tabs: [
          //     Tab(text: 'MSA (${view.msa.length})'),
          //     Tab(text: 'Método (${view.method.length})'),
          //   ],
          // ),
          // Expanded(
          //   child: TabBarView(
          //     children: [
          //       CategoryLessonsView(
          //         lessons: view.msa,
          //         emptyMessage: 'Nenhuma lição aprovada registrada.',
          //         checkpoints: progress.checkpoints,
          //         category: LessonCategory.msa,
          //         msaPhasePercent: progress.msaRelativePercent,
          //       ),
          //       CategoryLessonsView(
          //         lessons: view.method,
          //         emptyMessage: 'Nenhuma lição de método registrada.',
          //         checkpoints: progress.checkpoints,
          //         category: LessonCategory.method,
          //         methodLessonPercent: progress.methodRelativePercent,
          //       ),
          //     ],
          //   ),
          // ),
        ],
      ),
    );
  }
}
