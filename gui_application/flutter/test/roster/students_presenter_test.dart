import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/errors.dart';
import '../support/roster.dart';

void main() {
  group('StudentsPresenter', () {
    test('starts idle', () {
      final cubit = StudentsPresenter(
        retrieveStudents: () async => rosterLoaded([]),
      );

      expect(cubit.stateValue, isA<StudentsIdle>());
    });

    test('load() transitions Idle -> Loading -> Loaded on success', () async {
      final roster = pendingRoster();
      final cubit = StudentsPresenter(retrieveStudents: () => roster.future);

      final loadFuture = cubit.load();
      expect(cubit.stateValue, isA<StudentsLoading>());

      roster.complete(
        rosterLoaded([
          studentSummary(
            id: '1',
            name: 'Ana',
            position: Positions.candidate,
            location: 'Loc A',
          ),
          studentSummary(
            id: '2',
            name: 'Beto',
            position: Positions.youthService,
            location: 'Loc B',
          ),
        ]),
      );
      await loadFuture;

      final state = cubit.stateValue;
      expect(state, isA<StudentsLoaded>());
      final loaded = state as StudentsLoaded;
      expect(loaded.students.map((s) => s.id).toList(), ['1', '2']);
      expect(loaded.allStudents.length, 2);
    });

    test(
      'load() transitions Loading -> Failure carrying the mapped error report',
      () async {
        final cubit = StudentsPresenter(
          retrieveStudents: () async =>
              rosterFailed(unexpectedResponseFailure('missing table')),
        );

        await cubit.load();

        final state = cubit.stateValue;
        expect(state, isA<StudentsFailure>());
        expect(
          (state as StudentsFailure).report,
          const ErrorReport(
            userMessage:
                'O SAM respondeu de forma inesperada. '
                'Tente novamente em instantes.',
            details: 'missing table',
          ),
        );
      },
    );

    test(
      'load() reports a failure instead of hanging when the use case throws',
      () async {
        final cubit = StudentsPresenter(
          retrieveStudents: () async => throw StateError('bridge down'),
        );

        await cubit.load();

        final state = cubit.stateValue;
        expect(state, isA<StudentsFailure>());
        final report = (state as StudentsFailure).report;
        expect(report.userMessage, 'Algo deu errado. Tente novamente.');
        expect(report.details, contains('bridge down'));
      },
    );

    test('filter() narrows students by name query', () async {
      final cubit = StudentsPresenter(
        retrieveStudents: () async => rosterLoaded([
          studentSummary(
            id: '1',
            name: 'Ana Silva',
            position: Positions.candidate,
            location: 'Loc A',
          ),
          studentSummary(
            id: '2',
            name: 'Beto Souza',
            position: Positions.candidate,
            location: 'Loc B',
          ),
        ]),
      );

      await cubit.load();
      cubit.filter(nameQuery: 'Ana');

      final loaded = cubit.stateValue as StudentsLoaded;
      expect(loaded.students.map((s) => s.name).toList(), ['Ana Silva']);
      expect(loaded.allStudents.length, 2);
    });

    test('filter() narrows students by selected locations', () async {
      final cubit = StudentsPresenter(
        retrieveStudents: () async => rosterLoaded([
          studentSummary(
            id: '1',
            name: 'Ana',
            position: Positions.candidate,
            location: 'Loc A',
          ),
          studentSummary(
            id: '2',
            name: 'Beto',
            position: Positions.candidate,
            location: 'Loc B',
          ),
        ]),
      );

      await cubit.load();
      cubit.filter(selectedLocations: {'Loc B'});

      final loaded = cubit.stateValue as StudentsLoaded;
      expect(loaded.students.map((s) => s.id).toList(), ['2']);
    });

    test('clearFilters() resets query and locations', () async {
      final cubit = StudentsPresenter(
        retrieveStudents: () async => rosterLoaded([
          studentSummary(
            id: '1',
            name: 'Ana',
            position: Positions.candidate,
            location: 'Loc A',
          ),
        ]),
      );

      await cubit.load();
      cubit.filter(nameQuery: 'nobody-matches-this');
      cubit.clearFilters();

      final loaded = cubit.stateValue as StudentsLoaded;
      expect(loaded.nameQuery, '');
      expect(loaded.selectedLocations, isEmpty);
      expect(loaded.students.length, 1);
    });
  });
}
