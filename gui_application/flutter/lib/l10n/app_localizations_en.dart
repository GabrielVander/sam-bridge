// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'SAM Bridge';

  @override
  String get errorNetwork =>
      'Could not reach SAM. Check your internet connection and try again.';

  @override
  String get errorUnexpectedResponse =>
      'SAM responded unexpectedly. Try again in a moment.';

  @override
  String get errorSessionExpired =>
      'Your session has expired. Please sign in again.';

  @override
  String get errorLocalStorage =>
      'Could not access the data saved on this device.';

  @override
  String get errorGeneric => 'Something went wrong. Try again.';

  @override
  String progressUnavailable(String reason) {
    return 'Progress unavailable. $reason';
  }

  @override
  String get levelCandidate => 'Candidate';

  @override
  String get levelPractice => 'Rehearsal';

  @override
  String get levelYouthService => 'Youth Meeting';

  @override
  String get levelOfficialService => 'Official Service';

  @override
  String get levelOfficialized => 'Officialization';

  @override
  String get levelHalfHour => 'Half Hour';

  @override
  String get positionGemSecretary => 'GEM Secretary';

  @override
  String get positionMusicSecretary => 'Music Secretary';

  @override
  String get clefG => 'G';

  @override
  String get clefC => 'C';

  @override
  String get clefF => 'F';

  @override
  String get logOut => 'Log out';

  @override
  String get loginTitle => 'Sign in with your SAM account';

  @override
  String get loginEmail => 'Email';

  @override
  String get loginPassword => 'Password';

  @override
  String get loginShowPassword => 'Show password';

  @override
  String get loginHidePassword => 'Hide password';

  @override
  String get loginSubmit => 'Sign in';

  @override
  String get loginMissingFields => 'Enter your username and password';

  @override
  String get loginUnauthorized => 'Invalid username or password';

  @override
  String get rosterFilterByLocation => 'Filter by location';

  @override
  String get rosterNoLocations => 'No locations available.';

  @override
  String get cancel => 'Cancel';

  @override
  String get clear => 'Clear';

  @override
  String get apply => 'Apply';

  @override
  String get rosterSearchHint => 'Search by name…';

  @override
  String rosterSelectedLocations(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count locations',
      one: '1 location',
    );
    return '$_temp0';
  }

  @override
  String get rosterNoStudents => 'No students available.';

  @override
  String rosterNoResultsFor(String query) {
    return 'No results for \"$query\"';
  }

  @override
  String get rosterNoResults => 'No results';

  @override
  String rosterInLocations(String locations) {
    return 'in $locations';
  }

  @override
  String get rosterClearFilters => 'Clear filters';

  @override
  String get progressNoInstrument =>
      'This student has no instrument assigned in SAM yet. Progress cannot be calculated.';

  @override
  String get progressNotAMusician =>
      'Progress is only calculated for musicians; this student has another role in SAM.';

  @override
  String lessonsMsaTab(int count) {
    return 'MSA ($count)';
  }

  @override
  String lessonsMethodTab(int count) {
    return 'Method ($count)';
  }

  @override
  String get lessonsNoMsa => 'No approved lessons recorded.';

  @override
  String get lessonsNoMethod => 'No method lessons recorded.';

  @override
  String get backToStudents => 'Back';

  @override
  String get studentList => 'Student list';

  @override
  String get progressTitle => 'Progress';

  @override
  String progressTowards(String level) {
    return 'Next: $level';
  }

  @override
  String get msa => 'MSA';

  @override
  String get method => 'Method';

  @override
  String get progressAllLevelsReached => 'All levels reached';

  @override
  String checkpointReady(String level) {
    return '$level - ready for the exam';
  }

  @override
  String lessonPhase(String phase) {
    return 'Phase $phase';
  }

  @override
  String lessonPage(String page) {
    return 'p. $page';
  }

  @override
  String lessonNumber(String lesson) {
    return 'Lesson $lesson';
  }

  @override
  String lessonClef(String clef) {
    return '$clef clef';
  }

  @override
  String get unknownLevelTitle => 'unrecognized level';

  @override
  String unknownLevelValue(String raw) {
    return 'Value: $raw';
  }

  @override
  String get unknownLevelExplanation =>
      'Progress cannot be calculated for this level.';

  @override
  String get retry => 'Try again';

  @override
  String get technicalDetails => 'Technical details';

  @override
  String get copyDetails => 'Copy details';

  @override
  String get detailsCopied => 'Details copied';

  @override
  String get slowConnection =>
      'SAM is taking a while to respond. Please wait a little longer…';
}
