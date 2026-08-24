import 'package:flutter_application/portal/sam_portal.dart';
import 'package:flutter_application/view_models.dart';

/// Scriptable [SamPortal] fake for presenter/widget tests.
final class FakeSamPortal implements SamPortal {
  Object? loginError;
  bool loggedIn = false;
  List<StudentListItem> students = [];
  StudentLessonsView lessons = const StudentLessonsViewT().view;
  Object? studentsError;
  Object? lessonsError;

  final List<(String, String)> loginCalls = [];

  @override
  Future<void> login({
    required String baseUrl,
    required String username,
    required String password,
  }) async {
    if (loginError != null) throw loginError!;
    loginCalls.add((username, password));
    loggedIn = true;
  }

  @override
  Future<void> logout() async {
    loggedIn = false;
  }

  @override
  Future<bool> isLoggedIn() async => loggedIn;

  bool hasSaved = false;

  @override
  Future<bool> hasSavedCredentials() async => hasSaved;

  @override
  Future<bool> tryRestoreSession() async {
    if (hasSaved) {
      loggedIn = true;
      return true;
    }
    return false;
  }

  @override
  Future<List<StudentListItem>> retrieveStudents() async {
    if (studentsError != null) throw studentsError!;
    return students;
  }

  @override
  Future<StudentLessonsView> retrieveStudentLessons({required String studentId}) async {
    if (lessonsError != null) throw lessonsError!;
    return lessons;
  }
}

/// Helper to build view fixtures without Rust.
final class StudentLessonsViewT {
  final List<LessonItem> msa;
  final List<LessonItem> method;

  const StudentLessonsViewT({this.msa = const [], this.method = const []});

  StudentLessonsView get view => StudentLessonsView(msa: msa, method: method);
}

LessonItem lessonItem({
  required String id,
  required String date,
  LessonKind kind = LessonKind.msa,
  String phase = '',
  String page = '',
  String lesson = '',
  String clef = '',
  String description = '',
  String instructor = 'AUTH',
  String method = '',
}) =>
    LessonItem(
      kind: kind,
      id: id,
      date: date,
      phase: phase,
      page: page,
      lesson: lesson,
      clef: clef,
      description: description,
      instructor: instructor,
      method: method,
    );

StudentListItem studentItem({
  String id = '1',
  String name = 'ALUNA TESTE',
  String position = 'Músico · Candidato(a)',
  String location = 'BAIRRO',
  String rawLevel = 'Candidate',
}) =>
    StudentListItem(id: id, name: name, location: location, position: position, rawLevel: rawLevel);
