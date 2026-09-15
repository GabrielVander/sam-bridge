import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/lessons/widgets/back_bar.dart';
import 'package:flutter_application/lessons/widgets/category_lessons_view.dart';
import 'package:flutter_application/presentation_models.dart';

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
    return Scaffold(
      body: Column(
        children: [
          const BackBar(),
          const Divider(height: 1),
          Expanded(
            child: BlocSignalBuilder<LessonsCubitSignal, LessonsState>(
              builder: (context, state) => switch (state) {
                LessonsLoading() => const Center(
                  child: CircularProgressIndicator(),
                ),
                LessonsLoaded(:final view) => _LessonsTabs(view: view),
                LessonsFailure(:final message) => Center(
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Text(message, textAlign: TextAlign.center),
                  ),
                ),
                _ => const SizedBox.shrink(),
              },
            ),
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
                ),
                CategoryLessonsView(
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
