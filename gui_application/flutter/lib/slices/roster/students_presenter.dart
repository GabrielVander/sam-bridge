import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/portal/sam_portal.dart';
import 'package:flutter_application/view_models.dart';

sealed class StudentsState {
  const StudentsState();
}

final class StudentsIdle extends StudentsState {
  const StudentsIdle();
}

final class StudentsLoading extends StudentsState {
  const StudentsLoading();
}

final class StudentsLoaded extends StudentsState {
  final List<StudentListItem> students;
  const StudentsLoaded(this.students);
}

final class StudentsFailure extends StudentsState {
  final String message;
  const StudentsFailure(this.message);
}

class StudentsCubitSignal extends CubitSignal<StudentsState> {
  final SamPortal _portal;

  StudentsCubitSignal(this._portal) : super(initialState: const StudentsIdle());

  Future<void> load() async {
    emit(const StudentsLoading());
    try {
      final students = await _portal.retrieveStudents();
      emit(StudentsLoaded(students));
    } catch (e) {
      emit(StudentsFailure(e.toString()));
    }
  }
}
