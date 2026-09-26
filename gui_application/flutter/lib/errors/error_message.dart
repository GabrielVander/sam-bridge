import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/l10n/app_localizations.dart';

extension ErrorMessage on AppLocalizations {
  String errorMessage(ErrorReason reason) => switch (reason) {
    ErrorReason.network => errorNetwork,
    ErrorReason.unexpectedResponse => errorUnexpectedResponse,
    ErrorReason.sessionExpired => errorSessionExpired,
    ErrorReason.localStorage => errorLocalStorage,
    ErrorReason.generic => errorGeneric,
  };
}
