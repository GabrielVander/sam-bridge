import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/lessons/widgets/back_bar.dart';
import 'package:flutter_application/lessons/widgets/category_lessons_view.dart';
import 'package:flutter_application/lessons/widgets/student_detail_content.dart';
import 'package:flutter_application/lessons/widgets/unknown_level_banner.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/presentation_models.dart';

export 'package:flutter_application/lessons/widgets/category_lessons_view.dart'
    show LessonCategory;
export 'package:flutter_application/widgets/checkpoint_timeline.dart'
    show CheckpointTimeline;
export 'package:flutter_application/widgets/info_chip.dart' show InfoChip;
export 'package:flutter_application/widgets/progress_bar.dart' show ProgressBar;

class StudentScreen extends StatefulWidget {
  final String studentId;

  const StudentScreen({super.key, required this.studentId});

  @override
  State<StudentScreen> createState() => _StudentScreenState();
}

sealed class ProgressState {
  const ProgressState();
}

final class ProgressLoading extends ProgressState {
  const ProgressLoading();
}

final class ProgressAvailable extends ProgressState {
  const ProgressAvailable();
}

final class ProgressUnknown extends ProgressState {
  final String raw;
  const ProgressUnknown(this.raw);
}

final class ProgressFailure extends ProgressState {
  final String message;
  const ProgressFailure(this.message);
}

final class _StudentScreenState extends State<StudentScreen> {
  ProgressState _progress = const ProgressLoading();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      if (!mounted) return;
      final studentsState = context.read<StudentsCubitSignal>().stateValue;
      String assignedLevel = '__UNKNOWN__';
      bool found = false;
      if (studentsState is StudentsLoaded) {
        final list = studentsState.allStudents.isNotEmpty
            ? studentsState.allStudents
            : studentsState.students;
        for (final s in list) {
          if (s.id == widget.studentId) {
            assignedLevel = s.rawLevel;
            found = true;
            break;
          }
        }
      }
      context.read<LessonsCubitSignal>().load(widget.studentId);
      if (!found) {
        if (mounted) setState(() => _progress = const ProgressUnknown(''));
        return;
      }

      // try {
      //   final result = await portal.retrieveStudentProgress(
      //     studentId: widget.studentId,
      //     assignedLevel: assignedLevel,
      //   );
      //   if (!mounted) return;
      //   if (result.isUnknown) {
      //     setState(() => _progress = ProgressUnknown(result.unknown.raw));
      //   } else {
      //     setState(() => _progress = ProgressAvailable(result.progress));
      //   }
      // } catch (e) {
      //   if (mounted) {
      //     setState(() => _progress = ProgressFailure(e.toString()));
      //   }
      // }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          const BackBar(),
          Expanded(
            child: BlocSignalBuilder<LessonsCubitSignal, LessonsState>(
              builder: (context, state) => switch (state) {
                LessonsLoading() => const Center(
                  child: CircularProgressIndicator(),
                ),
                LessonsLoaded(:final view) => _buildLoaded(view),
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

  Widget _buildLoaded(StudentLessonsView view) {
    return switch (_progress) {
      ProgressUnknown(:final raw) => Column(
        children: [
          UnknownLevelBanner(raw: raw),
          const SizedBox(height: 8),
          Expanded(
            child: DefaultTabController(
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
                          // checkpoints: const [],
                          category: LessonCategory.msa,
                        ),
                        CategoryLessonsView(
                          lessons: view.method,
                          emptyMessage: 'Nenhuma lição de método registrada.',
                          // checkpoints: const [],
                          category: LessonCategory.method,
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
      _ => CircularProgressIndicator(),
      // ProgressAvailable(:final vm) => StudentDetailContent(
      //   view: view,
      //   progress: vm,
      // ),
      // ProgressLoading() => StudentDetailContent(
      //   view: view,
      //   progress: emptyProgress,
      // ),
      // ProgressFailure(:final message) => Column(
      //   children: [
      //     Padding(padding: const EdgeInsets.all(24), child: Text(message)),
      //     Expanded(
      //       child: StudentDetailContent(view: view, progress: emptyProgress),
      //     ),
      //   ],
      // ),
    };
  }
}
