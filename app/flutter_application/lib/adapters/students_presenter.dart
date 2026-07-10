import 'package:flutter_application/adapters/view_models.dart';
import 'package:flutter_application/infra/retrieve_students.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class StudentsPresenter extends Cubit<StudentsViewState> {
  StudentsPresenter() : super(StudentsView());

  Future<void> submitLogin(String username, String password) async {
    emit(StudentsViewLoading());

    try {
      if (username.isEmpty || password.isEmpty) {
        throw Exception("Username and password cannot be empty.");
      }

      emit(
        StudentsViewLoaded(
          students: await retrieveStudentsDefault(
            user: username,
            pass: password,
          ),
        ),
      );
    } catch (e) {
      emit(StudentsViewError(message: e.toString()));
    }
  }

  void returnToLogin() {
    emit(StudentsView());
  }
}

sealed class StudentsViewState {}

final class StudentsView extends StudentsViewState {}

final class StudentsViewLoading extends StudentsViewState {}

final class StudentsViewLoaded extends StudentsViewState {
  final List<SingleStudentViewModel> students;

  StudentsViewLoaded({required this.students});
}

final class StudentsViewError extends StudentsViewState {
  final String message;

  StudentsViewError({required this.message});
}
