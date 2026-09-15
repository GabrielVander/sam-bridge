import 'package:flutter/material.dart';
import 'package:flutter_application/presentation_models.dart';
import 'lesson_card.dart';

final class CategoryLessonsView extends StatelessWidget {
  final List<LessonItem> lessons;
  final String emptyMessage;

  const CategoryLessonsView({
    super.key,
    required this.lessons,
    required this.emptyMessage,
  });

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.all(12),
      children: [
        if (lessons.isEmpty)
          Center(child: Text(emptyMessage))
        else
          ...lessons.map((l) => LessonCard(l)),
      ],
    );
  }
}
