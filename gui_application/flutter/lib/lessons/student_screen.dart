import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/lessons/widgets/back_bar.dart';
import 'package:flutter_application/lessons/widgets/category_lessons_view.dart';
import 'package:flutter_application/lessons/widgets/checkpoint_timeline.dart';
import 'package:flutter_application/lessons/widgets/unknown_level_banner.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_application/widgets/loading_indicator.dart';

class StudentScreen extends StatefulWidget {
  final String studentId;
  final String? studentName;

  const StudentScreen({super.key, required this.studentId, this.studentName});

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
          BackBar(studentName: widget.studentName),
          const Divider(height: 1),
          Expanded(
            child: BlocSignalBuilder<LessonsCubitSignal, LessonsState>(
              builder: (context, state) => switch (state) {
                LessonsLoading() => const LoadingIndicator(),
                LessonsLoaded(:final view, :final progress) => _StudentDetail(
                  view: view,
                  progress: progress,
                ),
                LessonsFailure(:final report) => ErrorPanel(
                  report: report,
                  onRetry: () =>
                      context.read<LessonsCubitSignal>().load(widget.studentId),
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
      ProgressNotAMusician() => const _ProgressNotice(
        icon: Icons.info_outline,
        message:
            'O progresso só é calculado para músicos; '
            'este aluno tem outra função no SAM.',
      ),
      ProgressUnavailable(:final report) => _ProgressNotice(
        icon: Icons.info_outline,
        message: 'Progresso indisponível. ${report.userMessage}',
        details: report.details,
      ),
    };
  }
}

final class _ProgressNotice extends StatelessWidget {
  final IconData icon;
  final String message;
  final String details;

  const _ProgressNotice({
    required this.icon,
    required this.message,
    this.details = '',
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  icon,
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
                const SizedBox(width: 12),
                Expanded(child: Text(message)),
              ],
            ),
            if (details.isNotEmpty) TechnicalDetails(details: details),
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
