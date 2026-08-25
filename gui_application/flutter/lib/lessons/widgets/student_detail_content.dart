import 'package:flutter/material.dart';
import 'package:flutter_application/view_models.dart';
import 'package:flutter_application/widgets/checkpoint_timeline.dart';
import 'category_lessons_view.dart';

final class StudentDetailContent extends StatelessWidget {
  final StudentLessonsView view;
  final List<CheckpointVm> checkpoints;
  final double msaPercent;
  final double methodLessonPercent;

  const StudentDetailContent({
    super.key,
    required this.view,
    required this.checkpoints,
    required this.msaPercent,
    required this.methodLessonPercent,
  });

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2,
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            child: CheckpointTimeline(checkpoints: checkpoints, filter: null),
          ),
          TabBar(
            tabs: [
              Tab(text: 'MSA (${view.msa.length})'),
              Tab(text: 'Método (${view.method.length})'),
            ],
          ),
          Expanded(
            child: TabBarView(
              children: [
                CategoryLessonsView(
                  lessons: view.msa,
                  emptyMessage: 'Nenhuma lição aprovada registrada.',
                  checkpoints: checkpoints,
                  category: LessonCategory.msa,
                  msaPhasePercent: msaPercent,
                ),
                CategoryLessonsView(
                  lessons: view.method,
                  emptyMessage: 'Nenhuma lição de método registrada.',
                  checkpoints: checkpoints,
                  category: LessonCategory.method,
                  methodLessonPercent: methodLessonPercent,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
