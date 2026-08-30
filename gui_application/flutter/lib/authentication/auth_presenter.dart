import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/authentication/application/use_cases/login_use_case.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';

sealed class AuthState {
  const AuthState();
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

final class AuthFailure extends AuthState {
  final String message;
  const AuthFailure(this.message);
}

class AuthPresenter extends CubitSignal<AuthState> {
  final LoginUseCase loginUseCase;

  AuthPresenter({required this.loginUseCase})
    : super(initialState: const AuthIdle());

  Future<void> submitLogin(String username, String password) async {
    if (username.isEmpty || password.isEmpty) {
      emit(const AuthFailure('Informe usuário e senha.'));
      return;
    }

    emit(const AuthLoading());
    final LoginResult loginResult = await loginUseCase(
      email: username,
      password: password,
    );

    switch (loginResult) {
      case LoginResult_Successful():
        emit(const AuthSuccess());

      case LoginResult_InvalidEmailOrPassword():
        emit(const AuthFailure('Usuárrio ou Senha inválido(a)'));
      case LoginResult_UnableToPerformAuthorization(:final String context):
        emit(AuthFailure(context));
    }
  }

  void reset() => emit(const AuthIdle());

  void markRestored() {
    if (stateValue is! AuthSuccess) emit(const AuthSuccess());
  }

  bool get isAuthenticated => stateValue is AuthSuccess;
}
