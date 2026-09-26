import 'package:flutter_application/authentication/ports/login_use_case.dart';
import 'package:flutter_application/rust/api/authentication.dart';
import 'package:flutter_application/rust/api/error_report.dart';

import 'pending.dart';

LoginOutcomeDto loggedIn() => const LoginOutcomeDto.successful();

LoginOutcomeDto loginRejected() =>
    const LoginOutcomeDto.invalidEmailOrPassword();

LoginOutcomeDto loginFailed(ErrorReportDto report) =>
    LoginOutcomeDto.failure(report: report);

RestoreSessionOutcomeDto sessionRestored() =>
    const RestoreSessionOutcomeDto.restored();

RestoreSessionOutcomeDto noSavedSession() =>
    const RestoreSessionOutcomeDto.notAvailable();

Pending<RestoreSessionOutcomeDto> pendingRestore() => Pending();

LogoutOutcomeDto loggedOut() => const LogoutOutcomeDto.successful();

LogoutOutcomeDto logoutFailed(ErrorReportDto report) =>
    LogoutOutcomeDto.failure(report: report);

LoginUseCase loginAnswering(LoginOutcomeDto outcome) =>
    ({required email, required password}) async => outcome;
