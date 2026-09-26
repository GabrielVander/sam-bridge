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

  @override
  String get logOut => 'Sair';

  @override
  String get loginTitle => 'Entre com seu usuário SAM';

  @override
  String get loginEmail => 'Email';

  @override
  String get loginPassword => 'Senha';

  @override
  String get loginShowPassword => 'Mostrar senha';

  @override
  String get loginHidePassword => 'Ocultar senha';

  @override
  String get loginSubmit => 'Entrar';

  @override
  String get loginMissingFields => 'Informe usuário e senha';

  @override
  String get loginUnauthorized => 'Usuário ou senha inválido(a)';

  @override
  String get rosterFilterByLocation => 'Filtrar por local';

  @override
  String get rosterNoLocations => 'Nenhum local disponível.';

  @override
  String get cancel => 'Cancelar';

  @override
  String get clear => 'Limpar';

  @override
  String get apply => 'Aplicar';

  @override
  String get rosterSearchHint => 'Buscar por nome…';

  @override
  String rosterSelectedLocations(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count locais',
      one: '1 local',
    );
    return '$_temp0';
  }

  @override
  String get rosterNoStudents => 'Nenhum aluno disponível.';

  @override
  String rosterNoResultsFor(String query) {
    return 'Nenhum resultado para \"$query\"';
  }

  @override
  String get rosterNoResults => 'Nenhum resultado';

  @override
  String rosterInLocations(String locations) {
    return 'em $locations';
  }

  @override
  String get rosterClearFilters => 'Limpar filtros';
}
