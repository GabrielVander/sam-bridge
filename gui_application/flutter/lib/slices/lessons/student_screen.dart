import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/api.dart' as api;
import 'package:flutter_application/slices/lessons/lessons_presenter.dart';
import 'package:flutter_application/slices/roster/students_presenter.dart';
import 'package:flutter_application/view_models.dart';
import 'package:go_router/go_router.dart';

class StudentScreen extends StatefulWidget {
  final String studentId;

  const StudentScreen({super.key, required this.studentId});

  @override
  State<StudentScreen> createState() => _StudentScreenState();
}

final class _StudentScreenState extends State<StudentScreen> {
  _ProgressData? _progress;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      if (!mounted) return;
      // Look up the student's assigned level from the roster data.
      final studentsState = context.read<StudentsCubitSignal>().stateValue;
      String assignedLevel = 'Candidate';
      if (studentsState is StudentsLoaded) {
        for (final s in studentsState.students) {
          if (s.id == widget.studentId) {
            assignedLevel = s.rawLevel;
            break;
          }
        }
      }
      context.read<LessonsCubitSignal>().load(widget.studentId);
      try {
        final vm = await api.retrieveStudentProgress(
          studentId: widget.studentId,
          assignedLevel: assignedLevel,
        );
        if (mounted) setState(() => _progress = _ProgressData(vm));
      } catch (e) {
        if (mounted) {
          setState(() => _progress = _ProgressData.failure(e.toString()));
        }
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          _BackBar(),
          Expanded(
            child: BlocSignalBuilder<LessonsCubitSignal, LessonsState>(
              builder: (context, state) => switch (state) {
                LessonsLoading() => const Center(
                  child: CircularProgressIndicator(),
                ),
                LessonsLoaded(:final view) => _StudentDetailContent(
                  view: view,
                  checkpoints: _progress?.vm.checkpoints ?? [],
                  msaPercent: _progress?.vm.msaPercent ?? 0,
                  methodLessonPercent: _progress?.vm.methodLessonPercent ?? 0,
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

final class _StudentDetailContent extends StatelessWidget {
  final StudentLessonsView view;
  final List<CheckpointVm> checkpoints;
  final double msaPercent;
  final double methodLessonPercent;

  const _StudentDetailContent({
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
                _CategoryLessonsView(
                  lessons: view.msa,
                  emptyMessage: 'Nenhuma lição aprovada registrada.',
                  checkpoints: checkpoints,
                  category: LessonCategory.msa,
                  msaPhasePercent: msaPercent,
                ),
                _CategoryLessonsView(
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

enum LessonCategory { msa, method }

/// Always-visible level checkpoint timeline showing achieved / ready / pending.
final class CheckpointTimeline extends StatelessWidget {
  final List<CheckpointVm> checkpoints;
  final String? filter;

  const CheckpointTimeline({super.key, required this.checkpoints, this.filter});

  @override
  Widget build(BuildContext context) {
    if (checkpoints.isEmpty) return const SizedBox.shrink();

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: List.generate(checkpoints.length * 2 - 1, (i) {
          if (i.isOdd) {
            final leftCp = checkpoints[(i - 1) ~/ 2];
            final rightCp = checkpoints[(i + 1) ~/ 2];
            final bothMet = _met(leftCp, filter) && _met(rightCp, filter);
            return Expanded(
              child: Container(
                height: 2,
                margin: const EdgeInsets.only(top: 10),
                color: bothMet ? Colors.green : Theme.of(context).dividerColor,
              ),
            );
          }
          final cp = checkpoints[i ~/ 2];
          final met = _met(cp, filter);
          final color = cp.achieved
              ? Colors.green
              : cp.readyToAdvance
              ? Colors.orange
              : Colors.grey;

          return Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                met ? Icons.check_circle : Icons.radio_button_unchecked,
                size: 20,
                color: color,
              ),
              const SizedBox(height: 2),
              SizedBox(
                width: 64,
                child: Text(
                  cp.label,
                  textAlign: TextAlign.center,
                  style: Theme.of(
                    context,
                  ).textTheme.labelSmall?.copyWith(color: color),
                ),
              ),
            ],
          );
        }),
      ),
    );
  }

  static bool _met(CheckpointVm cp, String? filter) {
    if (cp.achieved) return true;
    return switch (filter) {
      'msa' => cp.msaRequirementMet,
      'method' => cp.methodRequirementMet,
      _ => cp.readyToAdvance,
    };
  }
}

/// Per-category content inside a tab: category timeline + bars + lesson list.
final class _CategoryLessonsView extends StatelessWidget {
  final List<LessonItem> lessons;
  final String emptyMessage;
  final List<CheckpointVm> checkpoints;
  final LessonCategory category;
  final double? msaPhasePercent;
  final double? methodLessonPercent;

  const _CategoryLessonsView({
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
          ...lessons.map(_LessonCard.new),
      ],
    );
  }
}

final class _LessonCard extends StatelessWidget {
  final LessonItem lesson;

  const _LessonCard(this.lesson);

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

class ProgressBar extends StatelessWidget {
  final String label;
  final double percent;

  const ProgressBar({super.key, required this.label, required this.percent});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        SizedBox(
          width: 60,
          child: Text(label, style: Theme.of(context).textTheme.bodySmall),
        ),
        Expanded(
          child: LinearProgressIndicator(
            value: (percent / 100).clamp(0.0, 1.0),
            backgroundColor: Theme.of(
              context,
            ).colorScheme.surfaceContainerHighest,
          ),
        ),
        const SizedBox(width: 8),
        SizedBox(
          width: 42,
          child: Text(
            '${percent.toStringAsFixed(0)}%',
            style: Theme.of(context).textTheme.labelSmall,
            textAlign: TextAlign.end,
          ),
        ),
      ],
    );
  }
}

class InfoChip extends StatelessWidget {
  final String label;

  const InfoChip({super.key, required this.label});

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

class _ProgressData {
  final ProgressViewModel vm;
  final String? error;

  _ProgressData(this.vm) : error = null;
  _ProgressData.failure(this.error)
    : vm = ProgressViewModel(
        msaPercent: 0,
        methodPagePercent: 0,
        methodLessonPercent: 0,
        overallPercent: 0,
        meetsYouthService: false,
        meetsOfficialService: false,
        meetsOfficialization: false,
        checkpoints: [],
      );
}
