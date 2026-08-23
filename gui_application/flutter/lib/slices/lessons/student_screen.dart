import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/slices/lessons/lessons_presenter.dart';
import 'package:flutter_application/view_models.dart';
import 'package:go_router/go_router.dart';

class StudentScreen extends StatefulWidget {
  final String studentId;

  const StudentScreen({super.key, required this.studentId});

  @override
  State<StudentScreen> createState() => _StudentScreenState();
}

final class _StudentScreenState extends State<StudentScreen> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      context.read<LessonsCubitSignal>().load(widget.studentId);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _BackBar(),
        Expanded(
          child: BlocSignalBuilder<LessonsCubitSignal, LessonsState>(
            builder: (context, state) => switch (state) {
              LessonsLoading() => const Center(
                child: CircularProgressIndicator(),
              ),
              LessonsLoaded(:final view) => _LessonsTabs(view: view),
              LessonsFailure(:final message) => Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.error_outline,
                      size: 48,
                      color: Theme.of(context).colorScheme.error,
                    ),
                    const SizedBox(height: 16),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 24),
                      child: Text(message, textAlign: TextAlign.center),
                    ),
                    const SizedBox(height: 16),
                    FilledButton.tonal(
                      onPressed: () => context.read<LessonsCubitSignal>().load(
                        widget.studentId,
                      ),
                      child: const Text('Tentar novamente'),
                    ),
                  ],
                ),
              ),
              _ => const SizedBox.shrink(),
            },
          ),
        ),
      ],
    );
  }
}

final class _BackBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(4, 4, 4, 0),
      child: Row(
        children: [
          IconButton(
            tooltip: 'Voltar',
            icon: const Icon(Icons.arrow_back),
            onPressed: () => context.go('/students'),
          ),
        ],
      ),
    );
  }
}

final class _LessonsTabs extends StatelessWidget {
  final StudentLessonsView view;

  const _LessonsTabs({required this.view});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2,
      child: Column(
        children: [
          TabBar(
            tabs: [
              Tab(text: 'Aprovadas (MSA) (${view.msa.length})'),
              Tab(text: 'Método (${view.method.length})'),
            ],
          ),
          Expanded(
            child: TabBarView(
              children: [
                _LessonList(
                  lessons: view.msa,
                  emptyMessage: 'Nenhuma lição aprovada registrada.',
                ),
                _LessonList(
                  lessons: view.method,
                  emptyMessage: 'Nenhuma lição de método registrada.',
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

final class _LessonList extends StatelessWidget {
  final List<LessonItem> lessons;
  final String emptyMessage;

  const _LessonList({required this.lessons, required this.emptyMessage});

  @override
  Widget build(BuildContext context) {
    if (lessons.isEmpty) {
      return Center(child: Text(emptyMessage));
    }

    return ListView.separated(
      padding: const EdgeInsets.all(12),
      itemCount: lessons.length,
      separatorBuilder: (_, _) => const SizedBox(height: 8),
      itemBuilder: (context, index) {
        final lesson = lessons[index];
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
                  _Chip(label: 'Fase ${lesson.phase}'),
                if (lesson.page.isNotEmpty) _Chip(label: 'Pág. ${lesson.page}'),
                if (lesson.lesson.isNotEmpty)
                  _Chip(label: 'Lição ${lesson.lesson}'),
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
      },
    );
  }
}

final class _Chip extends StatelessWidget {
  final String label;

  const _Chip({required this.label});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.secondaryContainer,
        borderRadius: BorderRadius.circular(10),
      ),
      child: Text(label, style: Theme.of(context).textTheme.labelSmall),
    );
  }
}
