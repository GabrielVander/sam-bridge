import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/rust/bootstrap/infra/error_view.dart';

class ErrorReportMapper {
  static const String _genericMessage = 'Algo deu errado. Tente novamente.';

  static ErrorReport toViewModel(ErrorReportDto dto) =>
      ErrorReport(userMessage: _userMessage(dto.kind), details: dto.details);

  /// For failures that never made it into a Rust outcome (e.g. the bridge
  /// itself threw), so the UI never shows a raw exception as its message.
  static ErrorReport fromThrown(Object error) =>
      ErrorReport(userMessage: _genericMessage, details: error.toString());

  static String _userMessage(ErrorKindDto kind) => switch (kind) {
    ErrorKindDto.network =>
      'Não foi possível conectar ao SAM. '
          'Verifique sua conexão com a internet e tente novamente.',
    ErrorKindDto.unexpectedResponse =>
      'O SAM respondeu de forma inesperada. Tente novamente em instantes.',
    ErrorKindDto.sessionExpired => 'Sua sessão expirou. Entre novamente.',
    ErrorKindDto.unknown => _genericMessage,
  };
}
