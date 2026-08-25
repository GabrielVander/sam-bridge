import 'package:flutter/material.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/widgets/info_chip.dart';

final class LessonCard extends StatelessWidget {
  final LessonItem lesson;

  const LessonCard(this.lesson, {super.key});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: ListTile(
        title: Wrap(
          spacing: 6,
          runSpacing: 4,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            Text(
              lesson.date.isEmpty ? '—' : lesson.date,
              style: Theme.of(context).textTheme.titleMedium,
            ),
            if (lesson.phase.isNotEmpty)
              InfoChip(label: 'Fase ${lesson.phase}'),
            if (lesson.page.isNotEmpty) InfoChip(label: 'Pág. ${lesson.page}'),
            if (lesson.lesson.isNotEmpty)
              InfoChip(label: 'Lição ${lesson.lesson}'),
          ],
        ),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (lesson.clef.isNotEmpty) Text('Clave: ${lesson.clef}'),
            if (lesson.description.isNotEmpty) Text(lesson.description),
            if (lesson.instructor.isNotEmpty)
              Text(
                lesson.instructor,
                style: Theme.of(context).textTheme.bodySmall,
              ),
          ],
        ),
        isThreeLine: true,
      ),
    );
  }
}
