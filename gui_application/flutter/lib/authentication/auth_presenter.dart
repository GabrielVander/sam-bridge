import 'package:bloc_signals/bloc_signals.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_application/authentication/application/use_cases/login_use_case.dart';
import 'package:flutter_application/authentication/application/use_cases/restore_session_use_case.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';

sealed class AuthState extends Equatable {
  const AuthState();

  @override
  List<Object?> get props => [];
}

final class AuthIdle extends AuthState {
  const AuthIdle();
}

final class AuthLoading extends AuthState {
  const AuthLoading();
}

final class AuthSuccess extends AuthState {
  const AuthSuccess();
}

final class AuthMissingFields extends AuthState {
  const AuthMissingFields();
}

final class AuthUnauthorized extends AuthState {
  const AuthUnauthorized();
}

final class AuthFailure extends AuthState {
  final String message;

  const AuthFailure(this.message);

  @override
  List<Object?> get props => [message];
}

class AuthPresenter extends CubitSignal<AuthState> {
  final LoginUseCase loginUseCase;
  final RestoreSessionUseCase restoreSessionUseCase;

  AuthPresenter({
    required this.loginUseCase,
    required this.restoreSessionUseCase,
  }) : super(initialState: const AuthIdle());

  Future<void> restoreSession() async {
    emit(const AuthLoading());
    final RestoreSessionOutcome outcome = await restoreSessionUseCase();

    switch (outcome) {
      case RestoreSessionOutcome.restored:
        emit(const AuthSuccess());
      case RestoreSessionOutcome.notAvailable:
        emit(const AuthIdle());
    }
  }

  Future<void> submitLogin(String username, String password) async {
    if (username.isEmpty || password.isEmpty) {
      emit(const AuthMissingFields());
      return;
    }

    emit(const AuthLoading());
    final LoginResult loginResult = await loginUseCase(
      email: username,
      password: password,
    );

    switch (loginResult) {
      case LoginResult.successful:
        emit(const AuthSuccess());

      case LoginResult.invalidEmailOrPassword:
        emit(const AuthUnauthorized());
      case LoginResult.unableToPerformAuthorization:
        emit(AuthFailure("Algo deu errado"));
    }
  }

  bool get isAuthenticated => stateValue is AuthSuccess;
}
