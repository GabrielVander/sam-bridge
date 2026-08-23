import 'package:flutter_application/api.dart' as api;
import 'package:flutter_application/view_models.dart';

/// Base URL of the SAM portal.
const String kSamBaseUrl = 'https://musical.congregacao.org.br';

/// Boundary the UI layer talks to. Implemented by Rust bindings in
/// production and by fakes in tests — keeps presenters pure Dart-testable.
abstract interface class SamPortal {
  Future<void> login({
    required String baseUrl,
    required String username,
    required String password,
  });
  Future<void> logout();
  Future<bool> isLoggedIn();
  Future<List<StudentListItem>> retrieveStudents();
  Future<StudentLessonsView> retrieveStudentLessons({
    required String studentId,
  });
}

/// Production implementation delegating to the generated FRB bindings.
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
  Future<List<StudentListItem>> retrieveStudents() => api.retrieveStudents();

  @override
  Future<StudentLessonsView> retrieveStudentLessons({
    required String studentId,
  }) => api.retrieveStudentLessons(studentId: studentId);
}
