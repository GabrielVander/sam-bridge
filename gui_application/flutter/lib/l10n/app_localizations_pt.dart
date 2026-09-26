// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Portuguese (`pt`).
class AppLocalizationsPt extends AppLocalizations {
  AppLocalizationsPt([String locale = 'pt']) : super(locale);

  @override
  String get appTitle => 'SAM Bridge';

  @override
  String get errorNetwork =>
      'Não foi possível conectar ao SAM. Verifique sua conexão com a internet e tente novamente.';

  @override
  String get errorUnexpectedResponse =>
      'O SAM respondeu de forma inesperada. Tente novamente em instantes.';

  @override
  String get errorSessionExpired => 'Sua sessão expirou. Entre novamente.';

  @override
  String get errorLocalStorage =>
      'Não foi possível acessar os dados salvos neste dispositivo.';

  @override
  String get errorGeneric => 'Algo deu errado. Tente novamente.';

  @override
  String progressUnavailable(String reason) {
    return 'Progresso indisponível. $reason';
  }

  @override
  String get levelCandidate => 'Candidato(a)';

  @override
  String get levelPractice => 'Ensaio';

  @override
  String get levelYouthService => 'Reunião de Jovens e Menores';

  @override
  String get levelOfficialService => 'Culto Oficial';

  @override
  String get levelOfficialized => 'Oficialização';

  @override
  String get levelHalfHour => 'Meia Hora';

  @override
  String get positionGemSecretary => 'Secretário(a) do GEM';

  @override
  String get positionMusicSecretary => 'Secretário(a) de Música';

  @override
  String get clefG => 'Sol';

  @override
  String get clefC => 'Dó';

  @override
  String get clefF => 'Fá';
}
