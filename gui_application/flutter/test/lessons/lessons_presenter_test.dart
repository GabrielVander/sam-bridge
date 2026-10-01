import 'package:flutter_application/lessons/ports/assess_student_progress_use_case.dart';
import 'package:flutter_application/lessons/ports/retrieve_student_lessons_use_case.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/shared/level.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/errors.dart';
import '../support/lessons.dart';

LessonsPresenter _buildCubit({
  RetrieveStudentLessonsUseCase? retrieveStudentLessons,
  AssessStudentProgressUseCase? assessStudentProgress,
}) => LessonsPresenter(
  retrieveStudentLessons:
      retrieveStudentLessons ??
      ({required studentId}) async => lessonsRetrieved(),
  assessStudentProgress:
      assessStudentProgress ??
      ({required studentId}) async => noInstrumentAssigned(),
);

void main() {
  group('LessonsCubitSignal', () {
    test('starts idle', () {
      final cubit = _buildCubit();

      expect(cubit.stateValue, isA<LessonsIdle>());
    });

    test('load() transitions Idle -> Loading -> Loaded on success', () async {
      final lessons = pendingLessons();
      final cubit = _buildCubit(
        retrieveStudentLessons: ({required studentId}) => lessons.future,
      );

      final loadFuture = cubit.load('500132');
      expect(cubit.stateValue, isA<LessonsLoading>());

      lessons.complete(
        lessonsRetrieved(
          studentLessons(
            msa: [lesson(id: '1')],
            method: [
              lesson(id: '2'),
              lesson(id: '3'),
            ],
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
      'load() transitions Loading -> Failure carrying the mapped error report',
      () async {
        final cubit = _buildCubit(
          retrieveStudentLessons: ({required studentId}) async => lessonsFailed(
            networkFailure("Request failed for operation 'student_lessons'"),
          ),
        );

        await cubit.load('500132');

        final state = cubit.stateValue;
        expect(state, isA<LessonsFailure>());
        expect(
          (state as LessonsFailure).report,
          const ErrorReport(
            reason: ErrorReason.network,
            details: "Request failed for operation 'student_lessons'",
          ),
        );
      },
    );

    test('load() never shows a raw exception as the message when a use case throws', () async {
      final cubit = _buildCubit(
        retrieveStudentLessons: ({required studentId}) async =>
            throw StateError('bridge down'),
      );

      await cubit.load('500132');

      final state = cubit.stateValue;
      expect(state, isA<LessonsFailure>());
      final report = (state as LessonsFailure).report;
      expect(report.reason, ErrorReason.generic);
      expect(report.details, contains('bridge down'));
    });

    test(
      'load() fails with the lessons error when both use cases throw',
      () async {
        final cubit = _buildCubit(
          retrieveStudentLessons: ({required studentId}) async =>
              throw StateError('lessons down'),
          assessStudentProgress: ({required studentId}) async =>
              throw StateError('progress down'),
        );

        await cubit.load('500132');

        final state = cubit.stateValue as LessonsFailure;
        expect(state.report.details, contains('lessons down'));
      },
    );

    test('load() passes the student id through to both use cases', () async {
      String? receivedLessonsId;
      String? receivedProgressId;
      final cubit = _buildCubit(
        retrieveStudentLessons: ({required studentId}) async {
          receivedLessonsId = studentId;
          return lessonsRetrieved();
        },
        assessStudentProgress: ({required studentId}) async {
          receivedProgressId = studentId;
          return noInstrumentAssigned();
        },
      );

      await cubit.load('999999');

      expect(receivedLessonsId, '999999');
      expect(receivedProgressId, '999999');
    });

    test('a successful progress outcome maps to ProgressAvailable', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async => progressAssessed(
          progressAssessment(
            msaRelativePercent: 50,
            methodRelativePercent: 25,
            combinedPercent: 37.5,
            overallCheckpointPercent: 60,
            nextLevel: Levels.officialService,
          ),
        ),
      );

      await cubit.load('500132');

      final loaded = cubit.stateValue as LessonsLoaded;
      expect(loaded.progress, isA<ProgressAvailable>());
      final progress = loaded.progress as ProgressAvailable;
      expect(progress.view.nextLevel, const KnownLevel(Level.officialService));
      expect(progress.view.overallCheckpointPercent, 60);
    });

    test('an unknown-level progress outcome maps to ProgressUnknownLevel without failing the whole screen', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async =>
            levelNotRecognized('EXÓTICO'),
      );

      await cubit.load('500132');

      final state = cubit.stateValue;
      expect(state, isA<LessonsLoaded>());
      final loaded = state as LessonsLoaded;
      expect(loaded.progress, isA<ProgressUnknownLevel>());
      expect((loaded.progress as ProgressUnknownLevel).raw, 'EXÓTICO');
    });

    test('a progress failure maps to ProgressUnavailable carrying the error report, without failing the whole screen', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async =>
            progressFailed(sessionExpiredFailure('Session expired')),
      );

      await cubit.load('500132');

      final loaded = cubit.stateValue as LessonsLoaded;
      expect(loaded.progress, isA<ProgressUnavailable>());
      expect(
        (loaded.progress as ProgressUnavailable).report,
        const ErrorReport(
          reason: ErrorReason.sessionExpired,
          details: 'Session expired',
        ),
      );
    });

    test('a thrown progress assessment maps to ProgressUnavailable, without failing the whole screen', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async =>
            throw StateError('bridge down'),
      );

      await cubit.load('500132');

      final loaded = cubit.stateValue as LessonsLoaded;
      final report = (loaded.progress as ProgressUnavailable).report;
      expect(report.reason, ErrorReason.generic);
      expect(report.details, contains('bridge down'));
    });

    test('a non-musician maps to ProgressNotAMusician', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async => notAMusician(),
      );

      await cubit.load('500132');

      final loaded = cubit.stateValue as LessonsLoaded;
      expect(loaded.progress, isA<ProgressNotAMusician>());
    });
  });
}
