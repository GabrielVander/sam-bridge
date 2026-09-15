import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/lessons/application/use_cases/retrieve_student_lessons_use_case.dart';
import 'package:flutter_application/lessons/lessons_mapper.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';

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
  const LessonsLoaded(this.view);
}

final class LessonsFailure extends LessonsState {
  final String message;
  const LessonsFailure(this.message);
}

class LessonsCubitSignal extends CubitSignal<LessonsState> {
  final RetrieveStudentLessonsUseCase retrieveStudentLessons;

  LessonsCubitSignal({required this.retrieveStudentLessons})
    : super(initialState: const LessonsIdle());

  Future<void> load(String studentId) async {
    emit(const LessonsLoading());
    try {
      final outcome = await retrieveStudentLessons(studentId: studentId);
      switch (outcome) {
        case RetrieveStudentLessonsOutcome_Success(:final field0):
          emit(LessonsLoaded(LessonsMapper.toViewModel(field0)));
        case RetrieveStudentLessonsOutcome_Failure(:final field0):
          emit(LessonsFailure(field0));
      }
    } catch (e) {
      emit(LessonsFailure(e.toString()));
    }
  }
}
