import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/rust/api/error_report.dart';

class ErrorReportMapper {
  static ErrorReport toViewModel(ErrorReportDto dto) =>
      ErrorReport(reason: _reason(dto.kind), details: dto.details);

  static ErrorReport fromThrown(Object error) =>
      ErrorReport(reason: ErrorReason.generic, details: error.toString());

  static ErrorReason _reason(ErrorKindDto kind) => switch (kind) {
    ErrorKindDto.network => ErrorReason.network,
    ErrorKindDto.unexpectedResponse => ErrorReason.unexpectedResponse,
    ErrorKindDto.sessionExpired => ErrorReason.sessionExpired,
    ErrorKindDto.localStorage => ErrorReason.localStorage,
    ErrorKindDto.unknown => ErrorReason.generic,
  };
}
