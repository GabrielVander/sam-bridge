import 'package:flutter_application/rust/api/error_report.dart';

ErrorReportDto networkFailure(String details) =>
    ErrorReportDto(kind: ErrorKindDto.network, details: details);

ErrorReportDto unexpectedResponseFailure(String details) =>
    ErrorReportDto(kind: ErrorKindDto.unexpectedResponse, details: details);

ErrorReportDto sessionExpiredFailure(String details) =>
    ErrorReportDto(kind: ErrorKindDto.sessionExpired, details: details);

ErrorReportDto localStorageFailure(String details) =>
    ErrorReportDto(kind: ErrorKindDto.localStorage, details: details);

ErrorReportDto unknownFailure(String details) =>
    ErrorReportDto(kind: ErrorKindDto.unknown, details: details);
