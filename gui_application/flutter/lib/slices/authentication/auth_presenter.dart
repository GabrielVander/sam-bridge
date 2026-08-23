import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/portal/sam_portal.dart';

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

class AuthCubitSignal extends CubitSignal<AuthState> {
  final SamPortal _portal;

  AuthCubitSignal(this._portal) : super(initialState: const AuthIdle());

  Future<void> submitLogin(String username, String password) async {
    if (username.isEmpty || password.isEmpty) {
      emit(const AuthFailure('Informe usuário e senha.'));
      return;
    }

    emit(const AuthLoading());
    try {
      await _portal.login(
        baseUrl: kSamBaseUrl,
        username: username,
        password: password,
      );
      emit(const AuthSuccess());
    } catch (e) {
      emit(AuthFailure(e.toString()));
    }
  }

  void reset() => emit(const AuthIdle());

  bool get isAuthenticated => stateValue is AuthSuccess;
}
