import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/errors.dart';

void main() {
  group('ErrorReportMapper.toViewModel', () {
    test('maps a network failure to a network reason', () {
      final dto = networkFailure('Request failed for operation dashboard');

      final report = ErrorReportMapper.toViewModel(dto);

      expect(
        report,
        const ErrorReport(
          reason: ErrorReason.network,
          details: 'Request failed for operation dashboard',
        ),
      );
    });

    test('maps an unexpected response to an unexpected response reason', () {
      final dto = unexpectedResponseFailure('missing table');

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.reason, ErrorReason.unexpectedResponse);
    });

    test('maps an expired session to a session expired reason', () {
      final dto = sessionExpiredFailure('Session expired');

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.reason, ErrorReason.sessionExpired);
    });

    test('maps a local storage failure to a local storage reason', () {
      final dto = localStorageFailure(
        'Unable to remove the credential file: Permission denied',
      );

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.reason, ErrorReason.localStorage);
    });

    test('maps an unknown failure to a generic reason', () {
      final dto = unknownFailure('boom');

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.reason, ErrorReason.generic);
    });

    test('passes the technical details through untouched', () {
      const details = 'outer: inner\nsecond line';
      final dto = unexpectedResponseFailure(details);

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.details, details);
    });
  });

  group('ErrorReportMapper.fromThrown', () {
    test('reports a generic reason and keeps the exception as details', () {
      final report = ErrorReportMapper.fromThrown(StateError('bridge down'));

      expect(report.reason, ErrorReason.generic);
      expect(report.details, contains('bridge down'));
    });
  });
}
