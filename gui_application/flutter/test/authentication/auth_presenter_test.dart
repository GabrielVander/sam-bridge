import 'dart:async';

import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_test/flutter_test.dart';

AuthPresenter buildPresenter({
  Future<LoginResult> Function({required String email, required String password})?
  loginUseCase,
  Future<RestoreSessionOutcome> Function()? restoreSessionUseCase,
}) {
  return AuthPresenter(
    loginUseCase:
        loginUseCase ??
        ({required email, required password}) async => LoginResult.successful,
    restoreSessionUseCase:
        restoreSessionUseCase ?? () async => RestoreSessionOutcome.notAvailable,
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

    test('submitLogin() reports missing fields without calling the use case', () async {
      var called = false;
      final presenter = buildPresenter(
        loginUseCase: ({required email, required password}) async {
          called = true;
          return LoginResult.successful;
        },
      );

      await presenter.submitLogin('', '');

      expect(presenter.stateValue, isA<AuthMissingFields>());
      expect(called, isFalse);
    });

    test('submitLogin() transitions to Success on successful login', () async {
      final presenter = buildPresenter(
        loginUseCase: ({required email, required password}) async =>
            LoginResult.successful,
      );

      await presenter.submitLogin('user@example.com', 'secret');

      expect(presenter.stateValue, isA<AuthSuccess>());
    });

    test(
      'submitLogin() transitions to Unauthorized on invalid credentials',
      () async {
        final presenter = buildPresenter(
          loginUseCase: ({required email, required password}) async =>
              LoginResult.invalidEmailOrPassword,
        );

        await presenter.submitLogin('user@example.com', 'wrong');

        expect(presenter.stateValue, isA<AuthUnauthorized>());
      },
    );

    test(
      'submitLogin() transitions to Failure when authorization cannot be performed',
      () async {
        final presenter = buildPresenter(
          loginUseCase: ({required email, required password}) async =>
              LoginResult.unableToPerformAuthorization,
        );

        await presenter.submitLogin('user@example.com', 'secret');

        expect(presenter.stateValue, isA<AuthFailure>());
      },
    );
  });
}
