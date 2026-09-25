import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/rust/api/error_report.dart';

class ErrorReportMapper {
  static const String _genericMessage = 'Algo deu errado. Tente novamente.';

  static ErrorReport toViewModel(ErrorReportDto dto) =>
      ErrorReport(userMessage: _userMessage(dto.kind), details: dto.details);

  static ErrorReport fromThrown(Object error) =>
      ErrorReport(userMessage: _genericMessage, details: error.toString());

  static String _userMessage(ErrorKindDto kind) => switch (kind) {
    ErrorKindDto.network =>
      'Não foi possível conectar ao SAM. '
          'Verifique sua conexão com a internet e tente novamente.',
    ErrorKindDto.unexpectedResponse =>
      'O SAM respondeu de forma inesperada. Tente novamente em instantes.',
    ErrorKindDto.sessionExpired => 'Sua sessão expirou. Entre novamente.',
    ErrorKindDto.localStorage =>
      'Não foi possível acessar os dados salvos neste dispositivo.',
    ErrorKindDto.unknown => _genericMessage,
  };
}
