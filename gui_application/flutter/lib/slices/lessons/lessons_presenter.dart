import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/portal/sam_portal.dart';
import 'package:flutter_application/view_models.dart';

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
  final SamPortal _portal;

  LessonsCubitSignal(this._portal) : super(initialState: const LessonsIdle());

  Future<void> load(String studentId) async {
    emit(const LessonsLoading());
    try {
      final view = await _portal.retrieveStudentLessons(studentId: studentId);
      emit(LessonsLoaded(view));
    } catch (e) {
      emit(LessonsFailure(e.toString()));
    }
  }
}
