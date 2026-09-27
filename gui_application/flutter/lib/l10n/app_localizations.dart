import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_pt.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations)!;
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('pt'),
  ];

  /// No description provided for @appTitle.
  ///
  /// In pt, this message translates to:
  /// **'SAM Bridge'**
  String get appTitle;

  /// No description provided for @errorNetwork.
  ///
  /// In pt, this message translates to:
  /// **'Não foi possível conectar ao SAM. Verifique sua conexão com a internet e tente novamente.'**
  String get errorNetwork;

  /// No description provided for @errorUnexpectedResponse.
  ///
  /// In pt, this message translates to:
  /// **'O SAM respondeu de forma inesperada. Tente novamente em instantes.'**
  String get errorUnexpectedResponse;

  /// No description provided for @errorSessionExpired.
  ///
  /// In pt, this message translates to:
  /// **'Sua sessão expirou. Entre novamente.'**
  String get errorSessionExpired;

  /// No description provided for @errorLocalStorage.
  ///
  /// In pt, this message translates to:
  /// **'Não foi possível acessar os dados salvos neste dispositivo.'**
  String get errorLocalStorage;

  /// No description provided for @errorGeneric.
  ///
  /// In pt, this message translates to:
  /// **'Algo deu errado. Tente novamente.'**
  String get errorGeneric;

  /// No description provided for @progressUnavailable.
  ///
  /// In pt, this message translates to:
  /// **'Progresso indisponível. {reason}'**
  String progressUnavailable(String reason);

  /// No description provided for @levelCandidate.
  ///
  /// In pt, this message translates to:
  /// **'Candidato(a)'**
  String get levelCandidate;

  /// No description provided for @levelPractice.
  ///
  /// In pt, this message translates to:
  /// **'Ensaio'**
  String get levelPractice;

  /// No description provided for @levelYouthService.
  ///
  /// In pt, this message translates to:
  /// **'Reunião de Jovens e Menores'**
  String get levelYouthService;

  /// No description provided for @levelOfficialService.
  ///
  /// In pt, this message translates to:
  /// **'Culto Oficial'**
  String get levelOfficialService;

  /// No description provided for @levelOfficialized.
  ///
  /// In pt, this message translates to:
  /// **'Oficialização'**
  String get levelOfficialized;

  /// No description provided for @levelHalfHour.
  ///
  /// In pt, this message translates to:
  /// **'Meia Hora'**
  String get levelHalfHour;

  /// No description provided for @positionGemSecretary.
  ///
  /// In pt, this message translates to:
  /// **'Secretário(a) do GEM'**
  String get positionGemSecretary;

  /// No description provided for @positionMusicSecretary.
  ///
  /// In pt, this message translates to:
  /// **'Secretário(a) de Música'**
  String get positionMusicSecretary;

  /// No description provided for @instrumentViolin.
  ///
  /// In pt, this message translates to:
  /// **'Violino'**
  String get instrumentViolin;

  /// No description provided for @instrumentViola.
  ///
  /// In pt, this message translates to:
  /// **'Viola'**
  String get instrumentViola;

  /// No description provided for @instrumentCello.
  ///
  /// In pt, this message translates to:
  /// **'Violoncelo'**
  String get instrumentCello;

  /// No description provided for @instrumentFlute.
  ///
  /// In pt, this message translates to:
  /// **'Flauta'**
  String get instrumentFlute;

  /// No description provided for @instrumentOboe.
  ///
  /// In pt, this message translates to:
  /// **'Oboé'**
  String get instrumentOboe;

  /// No description provided for @instrumentBassoon.
  ///
  /// In pt, this message translates to:
  /// **'Fagote'**
  String get instrumentBassoon;

  /// No description provided for @instrumentClarinet.
  ///
  /// In pt, this message translates to:
  /// **'Clarinete'**
  String get instrumentClarinet;

  /// No description provided for @instrumentAltoClarinet.
  ///
  /// In pt, this message translates to:
  /// **'Clarinete alto'**
  String get instrumentAltoClarinet;

  /// No description provided for @instrumentBassClarinet.
  ///
  /// In pt, this message translates to:
  /// **'Clarinete baixo'**
  String get instrumentBassClarinet;

  /// No description provided for @instrumentAltoSaxophone.
  ///
  /// In pt, this message translates to:
  /// **'Saxofone alto'**
  String get instrumentAltoSaxophone;

  /// No description provided for @instrumentCurvedSopranoSaxophone.
  ///
  /// In pt, this message translates to:
  /// **'Saxofone soprano curvo'**
  String get instrumentCurvedSopranoSaxophone;

  /// No description provided for @instrumentStraightSopranoSaxophone.
  ///
  /// In pt, this message translates to:
  /// **'Saxofone soprano reto'**
  String get instrumentStraightSopranoSaxophone;

  /// No description provided for @instrumentTenorSaxophone.
  ///
  /// In pt, this message translates to:
  /// **'Saxofone tenor'**
  String get instrumentTenorSaxophone;

  /// No description provided for @instrumentTrumpet.
  ///
  /// In pt, this message translates to:
  /// **'Trompete'**
  String get instrumentTrumpet;

  /// No description provided for @instrumentCornet.
  ///
  /// In pt, this message translates to:
  /// **'Cornet'**
  String get instrumentCornet;

  /// No description provided for @instrumentFlugelhorn.
  ///
  /// In pt, this message translates to:
  /// **'Flugelhorn'**
  String get instrumentFlugelhorn;

  /// No description provided for @instrumentFrenchHorn.
  ///
  /// In pt, this message translates to:
  /// **'Trompa'**
  String get instrumentFrenchHorn;

  /// No description provided for @instrumentTrombone.
  ///
  /// In pt, this message translates to:
  /// **'Trombone'**
  String get instrumentTrombone;

  /// No description provided for @instrumentEuphonium.
  ///
  /// In pt, this message translates to:
  /// **'Eufônio'**
  String get instrumentEuphonium;

  /// No description provided for @instrumentTuba.
  ///
  /// In pt, this message translates to:
  /// **'Tuba'**
  String get instrumentTuba;

  /// No description provided for @instrumentEnglishHorn.
  ///
  /// In pt, this message translates to:
  /// **'Corne inglês'**
  String get instrumentEnglishHorn;

  /// No description provided for @instrumentContraltoViolin.
  ///
  /// In pt, this message translates to:
  /// **'Violino contralto'**
  String get instrumentContraltoViolin;

  /// No description provided for @clefG.
  ///
  /// In pt, this message translates to:
  /// **'Sol'**
  String get clefG;

  /// No description provided for @clefC.
  ///
  /// In pt, this message translates to:
  /// **'Dó'**
  String get clefC;

  /// No description provided for @clefF.
  ///
  /// In pt, this message translates to:
  /// **'Fá'**
  String get clefF;

  /// No description provided for @logOut.
  ///
  /// In pt, this message translates to:
  /// **'Sair'**
  String get logOut;

  /// No description provided for @loginTitle.
  ///
  /// In pt, this message translates to:
  /// **'Entre com seu usuário SAM'**
  String get loginTitle;

  /// No description provided for @loginEmail.
  ///
  /// In pt, this message translates to:
  /// **'Email'**
  String get loginEmail;

  /// No description provided for @loginPassword.
  ///
  /// In pt, this message translates to:
  /// **'Senha'**
  String get loginPassword;

  /// No description provided for @loginShowPassword.
  ///
  /// In pt, this message translates to:
  /// **'Mostrar senha'**
  String get loginShowPassword;

  /// No description provided for @loginHidePassword.
  ///
  /// In pt, this message translates to:
  /// **'Ocultar senha'**
  String get loginHidePassword;

  /// No description provided for @loginSubmit.
  ///
  /// In pt, this message translates to:
  /// **'Entrar'**
  String get loginSubmit;

  /// No description provided for @loginMissingFields.
  ///
  /// In pt, this message translates to:
  /// **'Informe usuário e senha'**
  String get loginMissingFields;

  /// No description provided for @loginUnauthorized.
  ///
  /// In pt, this message translates to:
  /// **'Usuário ou senha inválido(a)'**
  String get loginUnauthorized;

  /// No description provided for @rosterFilterByLocation.
  ///
  /// In pt, this message translates to:
  /// **'Filtrar por local'**
  String get rosterFilterByLocation;

  /// No description provided for @rosterNoLocations.
  ///
  /// In pt, this message translates to:
  /// **'Nenhum local disponível.'**
  String get rosterNoLocations;

  /// No description provided for @cancel.
  ///
  /// In pt, this message translates to:
  /// **'Cancelar'**
  String get cancel;

  /// No description provided for @clear.
  ///
  /// In pt, this message translates to:
  /// **'Limpar'**
  String get clear;

  /// No description provided for @apply.
  ///
  /// In pt, this message translates to:
  /// **'Aplicar'**
  String get apply;

  /// No description provided for @rosterSearchHint.
  ///
  /// In pt, this message translates to:
  /// **'Buscar por nome…'**
  String get rosterSearchHint;

  /// No description provided for @rosterSelectedLocations.
  ///
  /// In pt, this message translates to:
  /// **'{count, plural, =1{1 local} other{{count} locais}}'**
  String rosterSelectedLocations(int count);

  /// No description provided for @rosterNoStudents.
  ///
  /// In pt, this message translates to:
  /// **'Nenhum aluno disponível.'**
  String get rosterNoStudents;

  /// No description provided for @rosterNoResultsFor.
  ///
  /// In pt, this message translates to:
  /// **'Nenhum resultado para \"{query}\"'**
  String rosterNoResultsFor(String query);

  /// No description provided for @rosterNoResults.
  ///
  /// In pt, this message translates to:
  /// **'Nenhum resultado'**
  String get rosterNoResults;

  /// No description provided for @rosterInLocations.
  ///
  /// In pt, this message translates to:
  /// **'em {locations}'**
  String rosterInLocations(String locations);

  /// No description provided for @rosterClearFilters.
  ///
  /// In pt, this message translates to:
  /// **'Limpar filtros'**
  String get rosterClearFilters;

  /// No description provided for @progressNoInstrument.
  ///
  /// In pt, this message translates to:
  /// **'Instrumento ainda não definido para este aluno no SAM. O progresso não pode ser calculado.'**
  String get progressNoInstrument;

  /// No description provided for @progressNotAMusician.
  ///
  /// In pt, this message translates to:
  /// **'O progresso só é calculado para músicos; este aluno tem outra função no SAM.'**
  String get progressNotAMusician;

  /// No description provided for @lessonsMsaTab.
  ///
  /// In pt, this message translates to:
  /// **'MSA ({count})'**
  String lessonsMsaTab(int count);

  /// No description provided for @lessonsMethodTab.
  ///
  /// In pt, this message translates to:
  /// **'Método ({count})'**
  String lessonsMethodTab(int count);

  /// No description provided for @lessonsNoMsa.
  ///
  /// In pt, this message translates to:
  /// **'Nenhuma lição aprovada registrada.'**
  String get lessonsNoMsa;

  /// No description provided for @lessonsNoMethod.
  ///
  /// In pt, this message translates to:
  /// **'Nenhuma lição de método registrada.'**
  String get lessonsNoMethod;

  /// No description provided for @backToStudents.
  ///
  /// In pt, this message translates to:
  /// **'Voltar'**
  String get backToStudents;

  /// No description provided for @studentList.
  ///
  /// In pt, this message translates to:
  /// **'Lista de alunos'**
  String get studentList;

  /// No description provided for @progressTitle.
  ///
  /// In pt, this message translates to:
  /// **'Progresso'**
  String get progressTitle;

  /// No description provided for @progressTowards.
  ///
  /// In pt, this message translates to:
  /// **'Rumo a: {level}'**
  String progressTowards(String level);

  /// No description provided for @msa.
  ///
  /// In pt, this message translates to:
  /// **'MSA'**
  String get msa;

  /// No description provided for @method.
  ///
  /// In pt, this message translates to:
  /// **'Método'**
  String get method;

  /// No description provided for @progressAllLevelsReached.
  ///
  /// In pt, this message translates to:
  /// **'Todos os níveis alcançados'**
  String get progressAllLevelsReached;

  /// No description provided for @checkpointReady.
  ///
  /// In pt, this message translates to:
  /// **'{level} - pronto para a prova'**
  String checkpointReady(String level);

  /// No description provided for @lessonPhase.
  ///
  /// In pt, this message translates to:
  /// **'Fase {phase}'**
  String lessonPhase(String phase);

  /// No description provided for @lessonPage.
  ///
  /// In pt, this message translates to:
  /// **'Pág. {page}'**
  String lessonPage(String page);

  /// No description provided for @lessonNumber.
  ///
  /// In pt, this message translates to:
  /// **'Lição {lesson}'**
  String lessonNumber(String lesson);

  /// No description provided for @lessonClef.
  ///
  /// In pt, this message translates to:
  /// **'Clave: {clef}'**
  String lessonClef(String clef);

  /// No description provided for @unknownLevelTitle.
  ///
  /// In pt, this message translates to:
  /// **'nível não reconhecido'**
  String get unknownLevelTitle;

  /// No description provided for @unknownLevelValue.
  ///
  /// In pt, this message translates to:
  /// **'Valor: {raw}'**
  String unknownLevelValue(String raw);

  /// No description provided for @unknownLevelExplanation.
  ///
  /// In pt, this message translates to:
  /// **'O progresso não pode ser calculado para este nível.'**
  String get unknownLevelExplanation;

  /// No description provided for @retry.
  ///
  /// In pt, this message translates to:
  /// **'Tentar novamente'**
  String get retry;

  /// No description provided for @technicalDetails.
  ///
  /// In pt, this message translates to:
  /// **'Detalhes técnicos'**
  String get technicalDetails;

  /// No description provided for @copyDetails.
  ///
  /// In pt, this message translates to:
  /// **'Copiar detalhes'**
  String get copyDetails;

  /// No description provided for @detailsCopied.
  ///
  /// In pt, this message translates to:
  /// **'Detalhes copiados'**
  String get detailsCopied;

  /// No description provided for @slowConnection.
  ///
  /// In pt, this message translates to:
  /// **'O SAM está demorando para responder. Aguarde mais um pouco…'**
  String get slowConnection;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'pt'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return AppLocalizationsEn();
    case 'pt':
      return AppLocalizationsPt();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
