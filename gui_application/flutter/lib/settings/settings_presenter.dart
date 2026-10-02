import 'dart:async';

import 'package:bloc_signals/bloc_signals.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/settings/ports/remember_language.dart';

final List<Language> _languages = [
  const SystemLanguage(),
  for (final Locale locale in AppLocalizations.supportedLocales)
    SpecificLanguage(locale),
];

class SettingsPresenter extends CubitSignal<SettingsState> {
  final RememberLanguage _rememberLanguage;

  SettingsPresenter({
    String? rememberedLanguage,
    this._rememberLanguage = _rememberNothing,
  }) : super(initialState: _stateOf(_languageOf(rememberedLanguage)));

  void choose(Language language) {
    emit(_stateOf(language));
    unawaited(_rememberLanguage(language.locale?.languageCode));
  }

  static Language _languageOf(String? languageCode) => _languages.firstWhere(
    (language) => language.locale?.languageCode == languageCode,
    orElse: () => const SystemLanguage(),
  );

  static SettingsState _stateOf(Language language) =>
      SettingsState(languages: _languages, selectedLanguage: language);

  static Future<void> _rememberNothing(String? _) async {}
}

final class SettingsState {
  final List<Language> languages;
  final Language selectedLanguage;

  const SettingsState({
    required this.languages,
    required this.selectedLanguage,
  });

  Locale? get appLocale => selectedLanguage.locale;
}

sealed class Language extends Equatable {
  const Language();

  Locale? get locale;

  @override
  List<Object?> get props => [locale];
}

final class SystemLanguage extends Language {
  const SystemLanguage();

  @override
  Locale? get locale => null;
}

final class SpecificLanguage extends Language {
  @override
  final Locale locale;

  const SpecificLanguage(this.locale);
}
