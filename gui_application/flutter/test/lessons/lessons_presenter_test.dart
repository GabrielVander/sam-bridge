import 'dart:async';

import 'package:flutter_application/lessons/application/use_cases/assess_student_progress_use_case.dart';
import 'package:flutter_application/lessons/application/use_cases/retrieve_student_lessons_use_case.dart';
import 'package:flutter_application/lessons/lessons_presenter.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/lessons_view.dart';
import 'package:flutter_application/rust/bootstrap/infra/progress_view.dart';
import 'package:flutter_test/flutter_test.dart';

const _emptyLessonsSuccess = RetrieveStudentLessonsOutcome.success(
  StudentLessonsDto(approved: [], method: []),
);

const _noInstrumentProgress =
    AssessStudentProgressOutcome.noInstrumentAssigned();

LessonsPresenter _buildCubit({
  RetrieveStudentLessonsUseCase? retrieveStudentLessons,
  AssessStudentProgressUseCase? assessStudentProgress,
}) => LessonsPresenter(
  retrieveStudentLessons:
      retrieveStudentLessons ??
      ({required studentId}) async => _emptyLessonsSuccess,
  assessStudentProgress:
      assessStudentProgress ??
      ({required studentId}) async => _noInstrumentProgress,
);

void main() {
  group('LessonsCubitSignal', () {
    test('starts idle', () {
      final cubit = _buildCubit();

      expect(cubit.stateValue, isA<LessonsIdle>());
    });

    test('load() transitions Idle -> Loading -> Loaded on success', () async {
      final completer = Completer<RetrieveStudentLessonsOutcome>();
      final cubit = _buildCubit(
        retrieveStudentLessons: ({required studentId}) => completer.future,
      );

      final loadFuture = cubit.load('500132');
      expect(cubit.stateValue, isA<LessonsLoading>());

      completer.complete(
        const RetrieveStudentLessonsOutcome.success(
          StudentLessonsDto(
            approved: [LessonDto(id: '1')],
            method: [
              LessonDto(id: '2'),
              LessonDto(id: '3'),
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
          retrieveStudentLessons: ({required studentId}) async =>
              const RetrieveStudentLessonsOutcome.failure(
                ErrorReportDto(
                  kind: ErrorKindDto.network,
                  details: "Request failed for operation 'student_lessons'",
                ),
              ),
        );

        await cubit.load('500132');

        final state = cubit.stateValue;
        expect(state, isA<LessonsFailure>());
        expect(
          (state as LessonsFailure).report,
          const ErrorReport(
            userMessage:
                'Não foi possível conectar ao SAM. '
                'Verifique sua conexão com a internet e tente novamente.',
            details: "Request failed for operation 'student_lessons'",
          ),
        );
      },
    );

    test(
      'load() never shows a raw exception as the message when a use case throws',
      () async {
        final cubit = _buildCubit(
          retrieveStudentLessons: ({required studentId}) async =>
              throw StateError('bridge down'),
        );

        await cubit.load('500132');

        final state = cubit.stateValue;
        expect(state, isA<LessonsFailure>());
        final report = (state as LessonsFailure).report;
        expect(report.userMessage, 'Algo deu errado. Tente novamente.');
        expect(report.details, contains('bridge down'));
      },
    );

    test('load() passes the student id through to both use cases', () async {
      String? receivedLessonsId;
      String? receivedProgressId;
      final cubit = _buildCubit(
        retrieveStudentLessons: ({required studentId}) async {
          receivedLessonsId = studentId;
          return _emptyLessonsSuccess;
        },
        assessStudentProgress: ({required studentId}) async {
          receivedProgressId = studentId;
          return _noInstrumentProgress;
        },
      );

      await cubit.load('999999');

      expect(receivedLessonsId, '999999');
      expect(receivedProgressId, '999999');
    });

    test('a successful progress outcome maps to ProgressAvailable', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async =>
            const AssessStudentProgressOutcome.success(
              ProgressAssessmentDto(
                checkpoints: [],
                msaRelativePercent: 50,
                methodRelativePercent: 25,
                combinedPercent: 37.5,
                overallCheckpointPercent: 60,
                nextLevel: 'OfficialService',
              ),
            ),
      );

      await cubit.load('500132');

      final loaded = cubit.stateValue as LessonsLoaded;
      expect(loaded.progress, isA<ProgressAvailable>());
      final progress = loaded.progress as ProgressAvailable;
      expect(progress.view.nextLevelLabel, 'Culto Oficial');
      expect(progress.view.overallCheckpointPercent, 60);
    });

    test(
      'an unknown-level progress outcome maps to ProgressUnknownLevel without failing the whole screen',
      () async {
        final cubit = _buildCubit(
          assessStudentProgress: ({required studentId}) async =>
              const AssessStudentProgressOutcome.unknownLevel('EXÓTICO'),
        );

        await cubit.load('500132');

        final state = cubit.stateValue;
        expect(state, isA<LessonsLoaded>());
        final loaded = state as LessonsLoaded;
        expect(loaded.progress, isA<ProgressUnknownLevel>());
        expect((loaded.progress as ProgressUnknownLevel).raw, 'EXÓTICO');
      },
    );

    test(
      'a progress failure maps to ProgressUnavailable carrying the error report, without failing the whole screen',
      () async {
        final cubit = _buildCubit(
          assessStudentProgress: ({required studentId}) async =>
              const AssessStudentProgressOutcome.failure(
                ErrorReportDto(
                  kind: ErrorKindDto.sessionExpired,
                  details: 'Session expired',
                ),
              ),
        );

        await cubit.load('500132');

        final loaded = cubit.stateValue as LessonsLoaded;
        expect(loaded.progress, isA<ProgressUnavailable>());
        expect(
          (loaded.progress as ProgressUnavailable).report,
          const ErrorReport(
            userMessage: 'Sua sessão expirou. Entre novamente.',
            details: 'Session expired',
          ),
        );
      },
    );

    test('a non-musician maps to ProgressNotAMusician', () async {
      final cubit = _buildCubit(
        assessStudentProgress: ({required studentId}) async =>
            const AssessStudentProgressOutcome.notAMusician(),
      );

      await cubit.load('500132');

      final loaded = cubit.stateValue as LessonsLoaded;
      expect(loaded.progress, isA<ProgressNotAMusician>());
    });
  });
}
