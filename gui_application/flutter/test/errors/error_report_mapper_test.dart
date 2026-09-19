import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ErrorReportMapper.toViewModel', () {
    test('maps a network failure to a connectivity message', () {
      const dto = ErrorReportDto(
        kind: ErrorKindDto.network,
        details: 'Request failed for operation dashboard',
      );

      final report = ErrorReportMapper.toViewModel(dto);

      expect(
        report,
        const ErrorReport(
          userMessage:
              'Não foi possível conectar ao SAM. '
              'Verifique sua conexão com a internet e tente novamente.',
          details: 'Request failed for operation dashboard',
        ),
      );
    });

    test('maps an unexpected response to a message blaming the site', () {
      const dto = ErrorReportDto(
        kind: ErrorKindDto.unexpectedResponse,
        details: 'missing table',
      );

      final report = ErrorReportMapper.toViewModel(dto);

      expect(
        report.userMessage,
        'O SAM respondeu de forma inesperada. Tente novamente em instantes.',
      );
    });

    test('maps an expired session to a message asking to sign in again', () {
      const dto = ErrorReportDto(
        kind: ErrorKindDto.sessionExpired,
        details: 'Session expired',
      );

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.userMessage, 'Sua sessão expirou. Entre novamente.');
    });

    test('maps an unknown failure to a generic message', () {
      const dto = ErrorReportDto(kind: ErrorKindDto.unknown, details: 'boom');

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.userMessage, 'Algo deu errado. Tente novamente.');
    });

    test('passes the technical details through untouched', () {
      const details = 'outer: inner\nsecond line';
      const dto = ErrorReportDto(
        kind: ErrorKindDto.unexpectedResponse,
        details: details,
      );

      final report = ErrorReportMapper.toViewModel(dto);

      expect(report.details, details);
    });
  });

  group('ErrorReportMapper.fromThrown', () {
    test('reports a generic message and keeps the exception as details', () {
      final report = ErrorReportMapper.fromThrown(StateError('bridge down'));

      expect(report.userMessage, 'Algo deu errado. Tente novamente.');
      expect(report.details, contains('bridge down'));
    });
  });
}
