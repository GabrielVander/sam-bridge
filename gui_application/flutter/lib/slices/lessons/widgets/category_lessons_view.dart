import 'package:flutter/material.dart';
import 'package:flutter_application/view_models.dart';
import 'package:flutter_application/widgets/checkpoint_timeline.dart';
import 'package:flutter_application/widgets/progress_bar.dart';
import 'lesson_card.dart';

enum LessonCategory { msa, method }

final class CategoryLessonsView extends StatelessWidget {
  final List<LessonItem> lessons;
  final String emptyMessage;
  final List<CheckpointVm> checkpoints;
  final LessonCategory category;
  final double? msaPhasePercent;
  final double? methodLessonPercent;

  const CategoryLessonsView({
    super.key,
    required this.lessons,
    required this.emptyMessage,
    required this.checkpoints,
    required this.category,
    this.msaPhasePercent,
    this.methodLessonPercent,
  });

  @override
  Widget build(BuildContext context) {
    final categoryKey = category == LessonCategory.msa ? 'msa' : 'method';

    return ListView(
      padding: const EdgeInsets.all(12),
      children: [
        CheckpointTimeline(checkpoints: checkpoints, filter: categoryKey),
        const SizedBox(height: 12),
        if (category == LessonCategory.msa && msaPhasePercent != null)
          ProgressBar(label: 'Fases', percent: msaPhasePercent!),
        if (category == LessonCategory.method && methodLessonPercent != null)
          Padding(
            padding: const EdgeInsets.only(top: 4),
            child: ProgressBar(label: 'Lições', percent: methodLessonPercent!),
          ),
        const SizedBox(height: 12),
        if (lessons.isEmpty)
          Center(child: Text(emptyMessage))
        else
          ...lessons.map((l) => LessonCard(l)),
      ],
    );
  }
}
