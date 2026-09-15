import 'dart:async';

import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('StudentsPresenter', () {
    test('starts idle', () {
      final cubit = StudentsPresenter(
        retrieveStudents: () async =>
            const RetrieveAllAvailableStudentsOutcome.success([]),
      );

      expect(cubit.stateValue, isA<StudentsIdle>());
    });

    test('load() transitions Idle -> Loading -> Loaded on success', () async {
      final completer = Completer<RetrieveAllAvailableStudentsOutcome>();
      final cubit = StudentsPresenter(
        retrieveStudents: () => completer.future,
      );

      final loadFuture = cubit.load();
      expect(cubit.stateValue, isA<StudentsLoading>());

      completer.complete(
        const RetrieveAllAvailableStudentsOutcome.success([
          StudentSummaryDto(
            id: '1',
            name: 'Ana',
            position: StudentPositionDto.candidate(),
            location: 'Loc A',
          ),
          StudentSummaryDto(
            id: '2',
            name: 'Beto',
            position: StudentPositionDto.youthService(),
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
      'load() transitions Loading -> Failure when the outcome reports a failure',
      () async {
        final cubit = StudentsPresenter(
          retrieveStudents: () async =>
              const RetrieveAllAvailableStudentsOutcome.failure('boom'),
        );

        await cubit.load();

        final state = cubit.stateValue;
        expect(state, isA<StudentsFailure>());
        expect((state as StudentsFailure).message, 'boom');
      },
    );

    test('filter() narrows students by name query', () async {
      final cubit = StudentsPresenter(
        retrieveStudents: () async =>
            const RetrieveAllAvailableStudentsOutcome.success([
              StudentSummaryDto(
                id: '1',
                name: 'Ana Silva',
                position: StudentPositionDto.candidate(),
                location: 'Loc A',
              ),
              StudentSummaryDto(
                id: '2',
                name: 'Beto Souza',
                position: StudentPositionDto.candidate(),
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
        retrieveStudents: () async =>
            const RetrieveAllAvailableStudentsOutcome.success([
              StudentSummaryDto(
                id: '1',
                name: 'Ana',
                position: StudentPositionDto.candidate(),
                location: 'Loc A',
              ),
              StudentSummaryDto(
                id: '2',
                name: 'Beto',
                position: StudentPositionDto.candidate(),
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
        retrieveStudents: () async =>
            const RetrieveAllAvailableStudentsOutcome.success([
              StudentSummaryDto(
                id: '1',
                name: 'Ana',
                position: StudentPositionDto.candidate(),
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
