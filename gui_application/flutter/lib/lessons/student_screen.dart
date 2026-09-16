import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/lessons/widgets/back_bar.dart';
import 'package:flutter_application/lessons/widgets/category_lessons_view.dart';
import 'package:flutter_application/lessons/widgets/checkpoint_timeline.dart';
import 'package:flutter_application/lessons/widgets/unknown_level_banner.dart';
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
                LessonsLoaded(:final view, :final progress) => _StudentDetail(
                  view: view,
                  progress: progress,
                ),
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

final class _StudentDetail extends StatelessWidget {
  final StudentLessonsView view;
  final ProgressStatus progress;

  const _StudentDetail({required this.view, required this.progress});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
          child: _ProgressSection(progress: progress),
        ),
        const SizedBox(height: 12),
        Expanded(child: _LessonsTabs(view: view)),
      ],
    );
  }
}

final class _ProgressSection extends StatelessWidget {
  final ProgressStatus progress;

  const _ProgressSection({required this.progress});

  @override
  Widget build(BuildContext context) {
    return switch (progress) {
      ProgressAvailable(:final view) => CheckpointTimeline(progress: view),
      ProgressNoInstrumentAssigned() => const _ProgressNotice(
        icon: Icons.music_off_outlined,
        message:
            'Instrumento ainda não definido para este aluno no SAM. '
            'O progresso não pode ser calculado.',
      ),
      ProgressUnknownLevel(:final raw) => UnknownLevelBanner(raw: raw),
      ProgressUnavailable(:final message) => _ProgressNotice(
        icon: Icons.info_outline,
        message: 'Progresso indisponível: $message',
      ),
    };
  }
}

final class _ProgressNotice extends StatelessWidget {
  final IconData icon;
  final String message;

  const _ProgressNotice({required this.icon, required this.message});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            Icon(icon, color: Theme.of(context).colorScheme.onSurfaceVariant),
            const SizedBox(width: 12),
            Expanded(child: Text(message)),
          ],
        ),
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
