import 'package:flutter_application/adapters/view_models.dart';
import 'package:flutter_application/infra/retrieve_student_lessons.dart';
import 'package:flutter_application/infra/retrieve_students.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:fuzzy/fuzzy.dart';

class StudentsPresenter extends Cubit<StudentsViewState> {
  String? username;
  String? password;

  StudentsPresenter() : super(StudentsView());

  Future<void> submitLogin(String username, String password) async {
    emit(StudentsViewLoading());

    try {
      if (username.isEmpty || password.isEmpty) {
        throw Exception("Username and password cannot be empty.");
      }

      this.username = username;
      this.password = password;

      List<SingleStudentViewModel> students = await retrieveStudentsDefault(
        user: username,
        pass: password,
      );

      emit(StudentsViewLoaded(allStudents: students, students: students));
    } catch (e) {
      emit(StudentsViewError(message: e.toString()));
    }
  }

  void returnToLogin() {
    emit(StudentsView());
  }

  void search(String query) {
    final StudentsViewState currentState = state;

    if (currentState is StudentsViewLoaded) {
      if (query.isEmpty) {
        emit(
          StudentsViewLoaded(
            allStudents: currentState.allStudents,
            students: currentState.allStudents,
          ),
        );
        return;
      }

      final Fuzzy<SingleStudentViewModel> fuse = Fuzzy<SingleStudentViewModel>(
        currentState.allStudents,
        options: FuzzyOptions(
          keys: [
            WeightedKey(
              name: 'name',
              getter: (SingleStudentViewModel s) => s.name,
              weight: 2,
            ),
            WeightedKey(
              name: 'location',
              getter: (SingleStudentViewModel s) => s.location,
              weight: 1,
            ),
          ],
          threshold: 0.3,
        ),
      );

      final List<SingleStudentViewModel> filteredStudents = fuse
          .search(query)
          .map((result) => result.item)
          .toList();

      emit(
        StudentsViewLoaded(
          allStudents: currentState.allStudents,
          students: filteredStudents,
        ),
      );
    }
  }

  void fetchLessons(String studentId) async {
    emit(StudentsViewLoading());

    try {
      List<SingleLessonViewModel> lessons = await retrieveStudentLessons(
        user: username!,
        pass: password!,
        student: studentId,
      );

      emit(StudentsViewLessonsLoaded(lessons: lessons));
    } catch (e) {
      emit(StudentsViewError(message: e.toString()));
    }
  }
}

sealed class StudentsViewState {}

final class StudentsView extends StudentsViewState {}

final class StudentsViewLoading extends StudentsViewState {}

final class StudentsViewLoaded extends StudentsViewState {
  final List<SingleStudentViewModel> allStudents;
  final List<SingleStudentViewModel> students;

  StudentsViewLoaded({required this.allStudents, required this.students});
}

final class StudentsViewLessonsLoaded extends StudentsViewState {
  final List<SingleLessonViewModel> lessons;

  StudentsViewLessonsLoaded({required this.lessons});
}

final class StudentsViewError extends StudentsViewState {
  final String message;

  StudentsViewError({required this.message});
}
