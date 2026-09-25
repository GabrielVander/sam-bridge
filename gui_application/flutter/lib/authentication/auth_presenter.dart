import 'package:bloc_signals/bloc_signals.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/authentication/ports/login_use_case.dart';
import 'package:flutter_application/authentication/ports/logout_use_case.dart';
import 'package:flutter_application/authentication/ports/restore_session_use_case.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/rust/api/authentication.dart';

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
  final ErrorReport report;

  const AuthFailure(this.report);

  @override
  List<Object?> get props => [report];
}

class AuthPresenter extends CubitSignal<AuthState> {
  final LoginUseCase loginUseCase;
  final RestoreSessionUseCase restoreSessionUseCase;
  final LogoutUseCase logoutUseCase;

  AuthPresenter({
    required this.loginUseCase,
    required this.restoreSessionUseCase,
    required this.logoutUseCase,
  }) : super(initialState: const AuthIdle());

  Future<void> restoreSession() async {
    emit(const AuthLoading());
    try {
      final RestoreSessionOutcome outcome = await restoreSessionUseCase();

      switch (outcome) {
        case RestoreSessionOutcome_Restored():
          emit(const AuthSuccess());
        case RestoreSessionOutcome_NotAvailable():
          emit(const AuthIdle());
        case RestoreSessionOutcome_Failure():
          emit(const AuthIdle());
      }
    } catch (_) {
      // A failed restore must never block startup: fall back to the login form.
      emit(const AuthIdle());
    }
  }

  Future<void> submitLogin(String username, String password) async {
    if (username.isEmpty || password.isEmpty) {
      emit(const AuthMissingFields());
      return;
    }

    emit(const AuthLoading());
    try {
      final LoginOutcome outcome = await loginUseCase(
        email: username,
        password: password,
      );

      switch (outcome) {
        case LoginOutcome_Successful():
          emit(const AuthSuccess());
        case LoginOutcome_InvalidEmailOrPassword():
          emit(const AuthUnauthorized());
        case LoginOutcome_Failure(:final report):
          emit(AuthFailure(ErrorReportMapper.toViewModel(report)));
      }
    } catch (e) {
      emit(AuthFailure(ErrorReportMapper.fromThrown(e)));
    }
  }

  Future<void> signOut() async {
    emit(const AuthLoading());
    try {
      await logoutUseCase();
    } catch (_) {
      // Local logout must always succeed from the UI's perspective; the
      // stored credential file is best-effort cleanup.
    }
    emit(const AuthIdle());
  }

  bool get isAuthenticated => stateValue is AuthSuccess;
}
