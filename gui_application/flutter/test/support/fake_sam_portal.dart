import 'package:flutter_application/dto.dart';
import 'package:flutter_application/portal/sam_portal.dart';
import 'package:flutter_application/view_models.dart';

final class FakeSamPortal implements SamPortal {
  Object? loginError;
  bool loggedIn = false;
  List<StudentSummaryDto> students = [];
  StudentLessonsDto lessons = const StudentLessonsViewT().dto;
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
  Future<List<StudentSummaryDto>> retrieveStudents() async {
    if (studentsError != null) throw studentsError!;
    return students;
  }

  @override
  Future<StudentLessonsDto> retrieveStudentLessons({required String studentId}) async {
    if (lessonsError != null) throw lessonsError!;
    return lessons;
  }

  ProgressResult progress = ProgressResult(
    isUnknown: false,
    progress: ProgressViewModel(
      msaRelativePercent: 0,
      methodRelativePercent: 0,
      combinedPercent: 0,
      overallCheckpointPercent: 0,
      nextLevelLabel: '',
      meetsYouthService: false,
      meetsOfficialService: false,
      meetsOfficialization: false,
      checkpoints: [],
    ),
    unknown: UnknownLevelVm(raw: '', message: ''),
  );
  Object? progressError;

  @override
  Future<ProgressResult> retrieveStudentProgress({
    required String studentId,
    required String assignedLevel,
  }) async {
    if (progressError != null) throw progressError!;
    return progress;
  }
}

final class StudentLessonsViewT {
  final List<LessonDto> msa;
  final List<LessonDto> method;

  const StudentLessonsViewT({this.msa = const [], this.method = const []});

  StudentLessonsDto get dto => StudentLessonsDto(approved: msa, method: method);
}

LessonDto lessonItem({
  required String id,
  required String date,
  String phase = '',
  String page = '',
  String lesson = '',
  String clef = '',
  String description = '',
  String instructor = 'AUTH',
  String method = '',
}) =>
    LessonDto(
      id: id,
      date: date,
      phase: phase.isEmpty ? null : RangeDto(from: phase, to: phase),
      page: page.isEmpty ? null : RangeDto(from: page, to: page),
      lesson: lesson.isEmpty ? null : RangeDto(from: lesson, to: lesson),
      clef: clef.isEmpty ? null : clef,
      description: description,
      instructor: instructor,
      method: method,
    );

StudentSummaryDto studentItem({
  String id = '1',
  String name = 'ALUNA TESTE',
  String levelName = 'Candidate',
  DtoPositionKind kind = DtoPositionKind.musician,
  String location = 'BAIRRO',
}) =>
    StudentSummaryDto(
      id: id,
      name: name,
      location: location,
      position: switch (kind) {
        DtoPositionKind.musician => DtoPosition.musician(levelName: levelName),
        DtoPositionKind.organist => DtoPosition.organist(levelName: levelName),
        DtoPositionKind.secretary => DtoPosition.secretary(typeName: levelName),
        DtoPositionKind.unknown => DtoPosition.unknown(raw: levelName),
      },
      region: const DtoRegion.araraquaraSaoCarlos(),
    );

enum DtoPositionKind { musician, organist, secretary, unknown }
