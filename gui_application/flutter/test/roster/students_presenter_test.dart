import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/errors.dart';
import '../support/roster.dart';

List<String> listedIds(StudentsPresenter presenter) =>
    switch ((presenter.stateValue as StudentsLoaded).listing) {
      Matches(:final students) => students.map((s) => s.id).toList(),
      _ => [],
    };

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

      expect(listedIds(cubit), ['1', '2']);
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
            reason: ErrorReason.unexpectedResponse,
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
        expect(report.reason, ErrorReason.generic);
        expect(report.details, contains('bridge down'));
      },
    );

    test('an empty roster has no students to list', () async {
      final cubit = presenterAnswering([rosterLoaded([])]);

      await cubit.load();

      expect((cubit.stateValue as StudentsLoaded).listing, isA<NoStudents>());
    });

    test('filters that match nobody leave no matches to list', () async {
      final cubit = presenterAnswering([
        rosterLoaded([studentSummary(name: 'Ana')]),
      ]);

      await cubit.load();
      cubit.filter(nameQuery: 'zzzz');

      expect((cubit.stateValue as StudentsLoaded).listing, isA<NoMatches>());
    });

    test('lists the students that match the filters', () async {
      final cubit = presenterAnswering([
        rosterLoaded([
          studentSummary(id: '1', name: 'Ana'),
          studentSummary(id: '2', name: 'Beto'),
        ]),
      ]);

      await cubit.load();
      cubit.filter(nameQuery: 'Beto');

      expect(listedIds(cubit), ['2']);
    });

    test('is filtering only while a name or a location is set', () async {
      final cubit = presenterAnswering([
        rosterLoaded([studentSummary(location: 'Loc A')]),
      ]);
      bool isFiltering() => (cubit.stateValue as StudentsLoaded).isFiltering;

      await cubit.load();
      expect(isFiltering(), isFalse);

      cubit.filter(nameQuery: 'Ana');
      expect(isFiltering(), isTrue);

      cubit.filter(nameQuery: '', selectedLocations: {'Loc A'});
      expect(isFiltering(), isTrue);
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

      expect(listedIds(cubit), ['2']);
    });

    test('removing a location keeps the other selected ones', () async {
      final cubit = presenterAnswering([
        rosterLoaded([
          studentSummary(id: '1', location: 'Loc A'),
          studentSummary(id: '2', location: 'Loc B'),
        ]),
      ]);

      await cubit.load();
      cubit.filter(selectedLocations: {'Loc A', 'Loc B'});
      cubit.removeLocation('Loc A');

      expect((cubit.stateValue as StudentsLoaded).selectedLocations, {'Loc B'});
      expect(listedIds(cubit), ['2']);
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
      expect(listedIds(cubit), ['1']);
    });
  });
}
