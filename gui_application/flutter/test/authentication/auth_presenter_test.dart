import 'dart:async';

import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_test/flutter_test.dart';

AuthPresenter buildPresenter({
  Future<LoginResult> Function({
    required String email,
    required String password,
  })?
  loginUseCase,
  Future<RestoreSessionOutcome> Function()? restoreSessionUseCase,
  Future<LogoutOutcome> Function()? logoutUseCase,
}) {
  return AuthPresenter(
    loginUseCase:
        loginUseCase ??
        ({required email, required password}) async =>
            const LoginResult.successful(),
    restoreSessionUseCase:
        restoreSessionUseCase ?? () async => RestoreSessionOutcome.notAvailable,
    logoutUseCase: logoutUseCase ?? () async => LogoutOutcome.successful,
  );
}

void main() {
  group('AuthPresenter', () {
    test('starts idle', () {
      final presenter = buildPresenter();

      expect(presenter.stateValue, isA<AuthIdle>());
    });

    test(
      'restoreSession() transitions Idle -> Loading -> Success when restored',
      () async {
        final completer = Completer<RestoreSessionOutcome>();
        final presenter = buildPresenter(
          restoreSessionUseCase: () => completer.future,
        );

        final restoreFuture = presenter.restoreSession();
        expect(presenter.stateValue, isA<AuthLoading>());

        completer.complete(RestoreSessionOutcome.restored);
        await restoreFuture;

        expect(presenter.stateValue, isA<AuthSuccess>());
        expect(presenter.isAuthenticated, isTrue);
      },
    );

    test(
      'restoreSession() transitions Loading -> Idle when not available',
      () async {
        final presenter = buildPresenter(
          restoreSessionUseCase: () async => RestoreSessionOutcome.notAvailable,
        );

        await presenter.restoreSession();

        expect(presenter.stateValue, isA<AuthIdle>());
        expect(presenter.isAuthenticated, isFalse);
      },
    );

    test(
      'submitLogin() reports missing fields without calling the use case',
      () async {
        var called = false;
        final presenter = buildPresenter(
          loginUseCase: ({required email, required password}) async {
            called = true;
            return const LoginResult.successful();
          },
        );

        await presenter.submitLogin('', '');

        expect(presenter.stateValue, isA<AuthMissingFields>());
        expect(called, isFalse);
      },
    );

    test('submitLogin() transitions to Success on successful login', () async {
      final presenter = buildPresenter(
        loginUseCase: ({required email, required password}) async =>
            const LoginResult.successful(),
      );

      await presenter.submitLogin('user@example.com', 'secret');

      expect(presenter.stateValue, isA<AuthSuccess>());
    });

    test(
      'submitLogin() transitions to Unauthorized on invalid credentials',
      () async {
        final presenter = buildPresenter(
          loginUseCase: ({required email, required password}) async =>
              const LoginResult.invalidEmailOrPassword(),
        );

        await presenter.submitLogin('user@example.com', 'wrong');

        expect(presenter.stateValue, isA<AuthUnauthorized>());
      },
    );

    test(
      'submitLogin() transitions to Failure carrying the mapped error report',
      () async {
        final presenter = buildPresenter(
          loginUseCase: ({required email, required password}) async =>
              const LoginResult.unableToPerformAuthorization(
                ErrorReportDto(
                  kind: ErrorKindDto.network,
                  details: "Request failed for operation 'authentication'",
                ),
              ),
        );

        await presenter.submitLogin('user@example.com', 'secret');

        final state = presenter.stateValue;
        expect(state, isA<AuthFailure>());
        expect(
          (state as AuthFailure).report,
          const ErrorReport(
            userMessage:
                'Não foi possível conectar ao SAM. '
                'Verifique sua conexão com a internet e tente novamente.',
            details: "Request failed for operation 'authentication'",
          ),
        );
      },
    );

    test(
      'submitLogin() reports a failure instead of hanging when the use case throws',
      () async {
        final presenter = buildPresenter(
          loginUseCase: ({required email, required password}) async =>
              throw StateError('bridge down'),
        );

        await presenter.submitLogin('user@example.com', 'secret');

        final state = presenter.stateValue;
        expect(state, isA<AuthFailure>());
        final report = (state as AuthFailure).report;
        expect(report.userMessage, 'Algo deu errado. Tente novamente.');
        expect(report.details, contains('bridge down'));
      },
    );

    test(
      'restoreSession() falls back to the login form when the use case throws',
      () async {
        final presenter = buildPresenter(
          restoreSessionUseCase: () async => throw StateError('bridge down'),
        );

        await presenter.restoreSession();

        expect(presenter.stateValue, isA<AuthIdle>());
      },
    );

    test('signOut() clears the session and returns to Idle', () async {
      var called = false;
      final presenter = buildPresenter(
        restoreSessionUseCase: () async => RestoreSessionOutcome.restored,
        logoutUseCase: () async {
          called = true;
          return LogoutOutcome.successful;
        },
      );
      await presenter.restoreSession();
      expect(presenter.isAuthenticated, isTrue);

      await presenter.signOut();

      expect(called, isTrue);
      expect(presenter.stateValue, isA<AuthIdle>());
      expect(presenter.isAuthenticated, isFalse);
    });

    test('signOut() still returns to Idle when the use case throws', () async {
      final presenter = buildPresenter(
        restoreSessionUseCase: () async => RestoreSessionOutcome.restored,
        logoutUseCase: () async => throw StateError('bridge down'),
      );
      await presenter.restoreSession();

      await presenter.signOut();

      expect(presenter.stateValue, isA<AuthIdle>());
      expect(presenter.isAuthenticated, isFalse);
    });
  });
}
