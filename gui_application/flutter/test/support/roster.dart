import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/roster/students_screen.dart';
import 'package:flutter_application/rust/api/roster.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';

/// The only place roster tests touch the generated bridge types, so a change to
/// them is fixed here rather than in every test.
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

/// A presenter whose successive loads answer with [outcomes], in order.
StudentsPresenter presenterAnswering(
  List<RetrieveAllAvailableStudentsOutcome> outcomes,
) {
  final remaining = [...outcomes];
  return StudentsPresenter(retrieveStudents: () async => remaining.removeAt(0));
}

RetrieveAllAvailableStudentsOutcome loaded(List<StudentSummaryDto> students) =>
    RetrieveAllAvailableStudentsOutcome.success(students: students);

/// Shows the students screen, with the page a student row opens standing in
/// for the real one so tests can see where a tap leads.
Future<void> pumpStudents(
  WidgetTester tester,
  StudentsPresenter presenter,
) async {
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
      child: MaterialApp.router(routerConfig: router),
    ),
  );
  await tester.pumpAndSettle();
}

/// Loads [students] into a fresh presenter and shows the screen.
Future<StudentsPresenter> pumpRoster(
  WidgetTester tester,
  List<StudentSummaryDto> students,
) async {
  final presenter = presenterAnswering([loaded(students)]);
  await pumpStudents(tester, presenter);
  return presenter;
}
