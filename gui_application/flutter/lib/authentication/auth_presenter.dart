import 'package:bloc_signals/bloc_signals.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/authentication/ports/login_use_case.dart';
import 'package:flutter_application/authentication/ports/logout_use_case.dart';
import 'package:flutter_application/authentication/ports/restore_session_use_case.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/rust/api/authentication.dart';
import 'package:flutter_application/rust/api/error_report.dart';

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
  final ErrorReport? notRemembered;

  const AuthSuccess({this.notRemembered});

  @override
  List<Object?> get props => [notRemembered];
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
  final LoginUseCase _loginUseCase;
  final RestoreSessionUseCase _restoreSessionUseCase;
  final LogoutUseCase _logoutUseCase;

  AuthPresenter({
    required this._loginUseCase,
    required this._restoreSessionUseCase,
    required this._logoutUseCase,
  }) : super(initialState: const AuthIdle());

  Future<void> restoreSession() async {
    emit(const AuthLoading());
    try {
      final RestoreSessionOutcomeDto outcome = await _restoreSessionUseCase();

      switch (outcome) {
        case RestoreSessionOutcomeDto_Restored():
          emit(const AuthSuccess());
        case RestoreSessionOutcomeDto_NotAvailable():
          emit(const AuthIdle());
        case RestoreSessionOutcomeDto_Failure():
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
      final LoginOutcomeDto outcome = await _loginUseCase(
        email: username,
        password: password,
      );

      switch (outcome) {
        case LoginOutcomeDto_Successful():
          emit(const AuthSuccess());
        case LoginOutcomeDto_SuccessfulWithoutRemembering(:final report):
          emit(
            AuthSuccess(notRemembered: ErrorReportMapper.toViewModel(report)),
          );
        case LoginOutcomeDto_InvalidEmailOrPassword():
          emit(const AuthUnauthorized());
        case LoginOutcomeDto_Failure(:final report):
          emit(AuthFailure(ErrorReportMapper.toViewModel(report)));
      }
    } catch (e) {
      emit(AuthFailure(ErrorReportMapper.fromThrown(e)));
    }
  }

  Future<void> signOut() async {
    emit(const AuthLoading());

    try {
      final LogoutOutcomeDto outcome = await _logoutUseCase();

      switch (outcome) {
        case LogoutOutcomeDto_Successful():
          emit(const AuthIdle());
        case LogoutOutcomeDto_Failure(:final ErrorReportDto report):
          emit(AuthFailure(ErrorReportMapper.toViewModel(report)));
      }
    } catch (_) {
      emit(const AuthIdle());
    }
  }

  bool get isAuthenticated => stateValue is AuthSuccess;
}
