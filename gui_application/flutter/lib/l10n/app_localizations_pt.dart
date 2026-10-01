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
  String get instrumentViolin => 'Violino';

  @override
  String get instrumentViola => 'Viola';

  @override
  String get instrumentCello => 'Violoncelo';

  @override
  String get instrumentFlute => 'Flauta';

  @override
  String get instrumentOboe => 'Oboé';

  @override
  String get instrumentBassoon => 'Fagote';

  @override
  String get instrumentClarinet => 'Clarinete';

  @override
  String get instrumentAltoClarinet => 'Clarinete alto';

  @override
  String get instrumentBassClarinet => 'Clarinete baixo';

  @override
  String get instrumentAltoSaxophone => 'Saxofone alto';

  @override
  String get instrumentCurvedSopranoSaxophone => 'Saxofone soprano curvo';

  @override
  String get instrumentStraightSopranoSaxophone => 'Saxofone soprano reto';

  @override
  String get instrumentTenorSaxophone => 'Saxofone tenor';

  @override
  String get instrumentTrumpet => 'Trompete';

  @override
  String get instrumentCornet => 'Cornet';

  @override
  String get instrumentFlugelhorn => 'Flugelhorn';

  @override
  String get instrumentFrenchHorn => 'Trompa';

  @override
  String get instrumentTrombone => 'Trombone';

  @override
  String get instrumentEuphonium => 'Eufônio';

  @override
  String get instrumentTuba => 'Tuba';

  @override
  String get instrumentEnglishHorn => 'Corne inglês';

  @override
  String get instrumentContraltoViolin => 'Violino contralto';

  @override
  String get clefG => 'Sol';

  @override
  String get clefC => 'Dó';

  @override
  String get clefF => 'Fá';

  @override
  String get windowMinimize => 'Minimizar';

  @override
  String get windowMaximize => 'Maximizar';

  @override
  String get windowClose => 'Fechar';

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
  String get loginNotRemembered =>
      'Você entrou, mas não foi possível lembrar o login neste dispositivo. Na próxima vez será preciso entrar de novo.';

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

  @override
  String get progressNoInstrument =>
      'Instrumento ainda não definido para este aluno no SAM. O progresso não pode ser calculado.';

  @override
  String get progressNotAMusician =>
      'O progresso só é calculado para músicos; este aluno tem outra função no SAM.';

  @override
  String lessonsMsaTab(int count) {
    return 'MSA ($count)';
  }

  @override
  String lessonsMethodTab(int count) {
    return 'Método ($count)';
  }

  @override
  String get lessonsNoMsa => 'Nenhuma lição aprovada registrada.';

  @override
  String get lessonsNoMethod => 'Nenhuma lição de método registrada.';

  @override
  String get backToStudents => 'Voltar';

  @override
  String get studentList => 'Lista de alunos';

  @override
  String get progressTitle => 'Progresso';

  @override
  String progressTowards(String level) {
    return 'Rumo a: $level';
  }

  @override
  String get msa => 'MSA';

  @override
  String get method => 'Método';

  @override
  String get progressAllLevelsReached => 'Todos os níveis alcançados';

  @override
  String progressPercent(int percent) {
    return '$percent%';
  }

  @override
  String checkpointReady(String level) {
    return '$level - pronto para a prova';
  }

  @override
  String lessonPhase(String phase) {
    return 'Fase $phase';
  }

  @override
  String lessonPage(String page) {
    return 'Pág. $page';
  }

  @override
  String lessonNumber(String lesson) {
    return 'Lição $lesson';
  }

  @override
  String lessonClef(String clef) {
    return 'Clave: $clef';
  }

  @override
  String get unknownLevelTitle => 'nível não reconhecido';

  @override
  String unknownLevelValue(String raw) {
    return 'Valor: $raw';
  }

  @override
  String get unknownLevelExplanation =>
      'O progresso não pode ser calculado para este nível.';

  @override
  String get retry => 'Tentar novamente';

  @override
  String get technicalDetails => 'Detalhes técnicos';

  @override
  String get copyDetails => 'Copiar detalhes';

  @override
  String get detailsCopied => 'Detalhes copiados';

  @override
  String get slowConnection =>
      'O SAM está demorando para responder. Aguarde mais um pouco…';

  @override
  String get settings => 'Configurações';

  @override
  String get language => 'Idioma';
}
