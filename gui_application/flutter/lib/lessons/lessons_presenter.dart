import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/lessons/ports/assess_student_progress_use_case.dart';
import 'package:flutter_application/lessons/ports/retrieve_student_lessons_use_case.dart';
import 'package:flutter_application/lessons/lessons_mapper.dart';
import 'package:flutter_application/lessons/progress_mapper.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/rust/api/lessons.dart';
import 'package:flutter_application/rust/api/progress.dart';

class LessonsPresenter extends CubitSignal<LessonsState> {
  final RetrieveStudentLessonsUseCase _retrieveStudentLessons;
  final AssessStudentProgressUseCase _assessStudentProgress;

  LessonsPresenter({
    required this._retrieveStudentLessons,
    required this._assessStudentProgress,
  }) : super(initialState: const LessonsIdle());

  Future<void> load(String studentId) async {
    emit(const LessonsLoading());
    final progressFuture = _assessProgress(studentId);
    try {
      final lessonsOutcome = await _retrieveStudentLessons(
        studentId: studentId,
      );

      switch (lessonsOutcome) {
        case RetrieveStudentLessonsOutcomeDto_Success(:final lessons):
          emit(
            LessonsLoaded(
              LessonsMapper.toViewModel(lessons),
              await progressFuture,
            ),
          );
        case RetrieveStudentLessonsOutcomeDto_Failure(:final report):
          emit(LessonsFailure(ErrorReportMapper.toViewModel(report)));
      }
    } catch (e) {
      emit(LessonsFailure(ErrorReportMapper.fromThrown(e)));
    }
  }

  Future<ProgressStatus> _assessProgress(String studentId) async {
    try {
      return _toProgressStatus(
        await _assessStudentProgress(studentId: studentId),
      );
    } catch (e) {
      return ProgressUnavailable(ErrorReportMapper.fromThrown(e));
    }
  }

  ProgressStatus _toProgressStatus(AssessStudentProgressOutcomeDto outcome) =>
      switch (outcome) {
        AssessStudentProgressOutcomeDto_Success(:final assessment) =>
          ProgressAvailable(ProgressMapper.toViewModel(assessment)),
        AssessStudentProgressOutcomeDto_NoInstrumentAssigned() =>
          const ProgressNoInstrumentAssigned(),
        AssessStudentProgressOutcomeDto_UnknownLevel(:final rawLevel) =>
          ProgressUnknownLevel(rawLevel),
        AssessStudentProgressOutcomeDto_NotAMusician() =>
          const ProgressNotAMusician(),
        AssessStudentProgressOutcomeDto_Failure(:final report) =>
          ProgressUnavailable(ErrorReportMapper.toViewModel(report)),
      };
}

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
