import 'dart:async';

import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('LessonsCubitSignal', () {
    test('starts idle', () {
      final cubit = LessonsCubitSignal(
        retrieveStudentLessons: ({required studentId}) async =>
            const RetrieveStudentLessonsOutcome.success(
              StudentLessonsDto(approved: [], method: []),
            ),
      );

      expect(cubit.stateValue, isA<LessonsIdle>());
    });

    test('load() transitions Idle -> Loading -> Loaded on success', () async {
      final completer = Completer<RetrieveStudentLessonsOutcome>();
      final cubit = LessonsCubitSignal(
        retrieveStudentLessons: ({required studentId}) => completer.future,
      );

      final loadFuture = cubit.load('500132');
      expect(cubit.stateValue, isA<LessonsLoading>());

      completer.complete(
        const RetrieveStudentLessonsOutcome.success(
          StudentLessonsDto(
            approved: [LessonDto(id: '1')],
            method: [LessonDto(id: '2'), LessonDto(id: '3')],
          ),
        ),
      );
      await loadFuture;

      final state = cubit.stateValue;
      expect(state, isA<LessonsLoaded>());
      final loaded = state as LessonsLoaded;
      expect(loaded.view.msa, hasLength(1));
      expect(loaded.view.method, hasLength(2));
    });

    test(
      'load() transitions Loading -> Failure when the outcome reports a failure',
      () async {
        final cubit = LessonsCubitSignal(
          retrieveStudentLessons: ({required studentId}) async =>
              const RetrieveStudentLessonsOutcome.failure('boom'),
        );

        await cubit.load('500132');

        final state = cubit.stateValue;
        expect(state, isA<LessonsFailure>());
        expect((state as LessonsFailure).message, 'boom');
      },
    );

    test('load() passes the student id through to the use case', () async {
      String? receivedId;
      final cubit = LessonsCubitSignal(
        retrieveStudentLessons: ({required studentId}) async {
          receivedId = studentId;
          return const RetrieveStudentLessonsOutcome.success(
            StudentLessonsDto(approved: [], method: []),
          );
        },
      );

      await cubit.load('999999');

      expect(receivedId, '999999');
    });
  });
}
