import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/roster/students_screen.dart';
import 'package:flutter_application/rust/api/error_report.dart';
import 'package:flutter_application/rust/api/roster.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';

import 'localization.dart';
import 'pending.dart';

StudentSummaryDto studentSummary({
  String id = '1',
  String name = 'Jane Doe',
  String location = 'Some Location',
  String? instrumentName,
  StudentPositionDto position = const StudentPositionDto.practice(),
}) => StudentSummaryDto(
  id: id,
  name: name,
  position: position,
  location: location,
  instrumentName: instrumentName,
);

StudentsPresenter presenterAnswering(
  List<RetrieveAllAvailableStudentsOutcomeDto> outcomes,
) {
  final List<RetrieveAllAvailableStudentsOutcomeDto> remaining = [...outcomes];

  return StudentsPresenter(retrieveStudents: () async => remaining.removeAt(0));
}

RetrieveAllAvailableStudentsOutcomeDto rosterLoaded(
  List<StudentSummaryDto> students,
) => RetrieveAllAvailableStudentsOutcomeDto.success(students: students);

RetrieveAllAvailableStudentsOutcomeDto rosterFailed(ErrorReportDto report) =>
    RetrieveAllAvailableStudentsOutcomeDto.failure(report: report);

Pending<RetrieveAllAvailableStudentsOutcomeDto> pendingRoster() => Pending();

abstract final class Positions {
  static const StudentPositionDto candidate = StudentPositionDto.candidate();
  static const StudentPositionDto practice = StudentPositionDto.practice();
  static const StudentPositionDto youthService =
      StudentPositionDto.youthService();
  static const StudentPositionDto officialService =
      StudentPositionDto.officialService();
  static const StudentPositionDto officialized =
      StudentPositionDto.officialized();
  static const StudentPositionDto halfHour = StudentPositionDto.halfHour();
  static const StudentPositionDto youthServiceHalfHour =
      StudentPositionDto.youthServiceHalfHour();
  static const StudentPositionDto youthServicePractice =
      StudentPositionDto.youthServicePractice();
  static const StudentPositionDto youthServiceOfficialService =
      StudentPositionDto.youthServiceOfficialService();
  static const StudentPositionDto youthServiceOfficialized =
      StudentPositionDto.youthServiceOfficialized();
  static const StudentPositionDto gemSecretary =
      StudentPositionDto.gemSecretary();
  static const StudentPositionDto musicSecretary =
      StudentPositionDto.musicSecretary();

  static StudentPositionDto invalid(String raw) =>
      StudentPositionDto.invalid(raw: raw);
}

Future<void> pumpStudents(
  WidgetTester tester,
  StudentsPresenter presenter, {
  Locale locale = portuguese,
}) async {
  final router = GoRouter(
    initialLocation: '/students',
    routes: [
      GoRoute(
        path: '/students',
        builder: (_, _) => const Scaffold(body: StudentsScreen()),
        routes: [
          GoRoute(
            path: ':studentId',
            builder: (_, state) => Scaffold(
              body: Text(
                'página do aluno ${state.pathParameters['studentId']} '
                '(${state.extra})',
              ),
            ),
          ),
        ],
      ),
    ],
  );

  await tester.pumpWidget(
    BlocSignalProvider<StudentsPresenter>.value(
      value: presenter,
      child: localizedRouterApp(router, locale: locale),
    ),
  );
  await tester.pumpAndSettle();
}

Future<StudentsPresenter> pumpRoster(
  WidgetTester tester,
  List<StudentSummaryDto> students, {
  Locale locale = portuguese,
}) async {
  final StudentsPresenter presenter = presenterAnswering([
    rosterLoaded(students),
  ]);

  await pumpStudents(tester, presenter, locale: locale);

  return presenter;
}
