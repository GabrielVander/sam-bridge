import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/widgets/info_chip.dart';

final class LessonCard extends StatelessWidget {
  final LessonItem lesson;

  const LessonCard(this.lesson, {super.key});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            CircleAvatar(
              radius: 18,
              backgroundColor: Theme.of(context).colorScheme.secondaryContainer,
              child: Icon(
                lesson.kind == LessonKind.msa
                    ? Icons.menu_book
                    : Icons.music_note,
                size: 18,
                color: Theme.of(context).colorScheme.onSecondaryContainer,
              ),
            ),
            const SizedBox(width: 14),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Wrap(
                    spacing: 6,
                    runSpacing: 6,
                    crossAxisAlignment: WrapCrossAlignment.center,
                    children: [
                      Text(
                        lesson.date.isEmpty ? '—' : lesson.date,
                        style: Theme.of(context).textTheme.titleMedium,
                      ),
                      if (lesson.phase.isNotEmpty)
                        InfoChip(label: 'Fase ${lesson.phase}'),
                      if (lesson.page.isNotEmpty)
                        InfoChip(label: 'Pág. ${lesson.page}'),
                      if (lesson.lesson.isNotEmpty)
                        InfoChip(label: 'Lição ${lesson.lesson}'),
                    ],
                  ),
                  if (lesson.clef.isNotEmpty ||
                      lesson.description.isNotEmpty ||
                      lesson.instructor.isNotEmpty) ...[
                    const SizedBox(height: 8),
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        if (lesson.clef.isNotEmpty)
                          Text('Clave: ${lesson.clef}'),
                        if (lesson.description.isNotEmpty) ...[
                          const SizedBox(height: 2),
                          Text(lesson.description),
                        ],
                        if (lesson.instructor.isNotEmpty) ...[
                          const SizedBox(height: 4),
                          Text(
                            lesson.instructor,
                            style: Theme.of(context).textTheme.bodySmall,
                          ),
                        ],
                      ],
                    ),
                  ],
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
