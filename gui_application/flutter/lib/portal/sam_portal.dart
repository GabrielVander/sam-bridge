import 'package:flutter_application/api.dart' as api;
import 'package:flutter_application/view_models.dart';

const String kSamBaseUrl = 'https://musical.congregacao.org.br';

abstract interface class SamPortal {
  Future<void> login({
    required String baseUrl,
    required String username,
    required String password,
  });
  Future<void> logout();
  Future<bool> isLoggedIn();
  Future<bool> hasSavedCredentials();
  Future<bool> tryRestoreSession();
  Future<List<StudentListItem>> retrieveStudents();
  Future<StudentLessonsView> retrieveStudentLessons({
    required String studentId,
  });
}

final class RustSamPortal implements SamPortal {
  const RustSamPortal();

  @override
  Future<void> login({
    required String baseUrl,
    required String username,
    required String password,
  }) => api.login(baseUrl: baseUrl, username: username, password: password);

  @override
  Future<void> logout() => api.logout();

  @override
  Future<bool> isLoggedIn() => api.isLoggedIn();

  @override
  Future<bool> hasSavedCredentials() => api.hasSavedCredentials();

  @override
  Future<bool> tryRestoreSession() => api.tryRestoreSession();

  @override
  Future<List<StudentListItem>> retrieveStudents() => api.retrieveStudents();

  @override
  Future<StudentLessonsView> retrieveStudentLessons({
    required String studentId,
  }) => api.retrieveStudentLessons(studentId: studentId);
}
