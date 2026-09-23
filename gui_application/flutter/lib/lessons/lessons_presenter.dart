import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/lessons/application/use_cases/assess_student_progress_use_case.dart';
import 'package:flutter_application/lessons/application/use_cases/retrieve_student_lessons_use_case.dart';
import 'package:flutter_application/lessons/lessons_mapper.dart';
import 'package:flutter_application/lessons/progress_mapper.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';

sealed class LessonsState {
  const LessonsState();
}

final class LessonsIdle extends LessonsState {
  const LessonsIdle();
}

final class LessonsLoading extends LessonsState {
  const LessonsLoading();
}

final class LessonsLoaded extends LessonsState {
  final StudentLessonsView view;
  final ProgressStatus progress;
  const LessonsLoaded(this.view, this.progress);
}

final class LessonsFailure extends LessonsState {
  final ErrorReport report;
  const LessonsFailure(this.report);
}

sealed class ProgressStatus {
  const ProgressStatus();
}

final class ProgressAvailable extends ProgressStatus {
  final ProgressView view;
  const ProgressAvailable(this.view);
}

final class ProgressNoInstrumentAssigned extends ProgressStatus {
  const ProgressNoInstrumentAssigned();
}

final class ProgressUnknownLevel extends ProgressStatus {
  final String raw;
  const ProgressUnknownLevel(this.raw);
}

final class ProgressNotAMusician extends ProgressStatus {
  const ProgressNotAMusician();
}

final class ProgressUnavailable extends ProgressStatus {
  final ErrorReport report;
  const ProgressUnavailable(this.report);
}

class LessonsPresenter extends CubitSignal<LessonsState> {
  final RetrieveStudentLessonsUseCase retrieveStudentLessons;
  final AssessStudentProgressUseCase assessStudentProgress;

  LessonsPresenter({
    required this.retrieveStudentLessons,
    required this.assessStudentProgress,
  }) : super(initialState: const LessonsIdle());

  Future<void> load(String studentId) async {
    emit(const LessonsLoading());
    try {
      final lessonsFuture = retrieveStudentLessons(studentId: studentId);
      final progressFuture = assessStudentProgress(studentId: studentId);

      final lessonsOutcome = await lessonsFuture;
      final progressOutcome = await progressFuture;

      switch (lessonsOutcome) {
        case RetrieveStudentLessonsOutcome_Success(:final field0):
          emit(
            LessonsLoaded(
              LessonsMapper.toViewModel(field0),
              _toProgressStatus(progressOutcome),
            ),
          );
        case RetrieveStudentLessonsOutcome_Failure(:final field0):
          emit(LessonsFailure(ErrorReportMapper.toViewModel(field0)));
      }
    } catch (e) {
      emit(LessonsFailure(ErrorReportMapper.fromThrown(e)));
    }
  }

  ProgressStatus _toProgressStatus(AssessStudentProgressOutcome outcome) =>
      switch (outcome) {
        AssessStudentProgressOutcome_Success(:final field0) =>
          ProgressAvailable(ProgressMapper.toViewModel(field0)),
        AssessStudentProgressOutcome_NoInstrumentAssigned() =>
          const ProgressNoInstrumentAssigned(),
        AssessStudentProgressOutcome_UnknownLevel(:final field0) =>
          ProgressUnknownLevel(field0),
        AssessStudentProgressOutcome_NotAMusician() =>
          const ProgressNotAMusician(),
        AssessStudentProgressOutcome_Failure(:final field0) =>
          ProgressUnavailable(ErrorReportMapper.toViewModel(field0)),
      };
}
