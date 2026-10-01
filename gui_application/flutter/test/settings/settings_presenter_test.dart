import 'package:flutter/widgets.dart';
import 'package:flutter_application/settings/settings_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('SettingsPresenter', () {
    test('follows the system language when none was chosen', () {
      final presenter = SettingsPresenter();

      expect(presenter.stateValue.appLocale, isNull);
    });

    test('offers the system language and every language the app speaks', () {
      final presenter = SettingsPresenter();

      expect(presenter.stateValue.languages, const [
        SystemLanguage(),
        SpecificLanguage(Locale('en')),
        SpecificLanguage(Locale('pt')),
      ]);
      expect(presenter.stateValue.selectedLanguage, const SystemLanguage());
    });

    test('applies and selects the language the user chooses', () {
      final presenter = SettingsPresenter();

      presenter.choose(const SpecificLanguage(Locale('en')));

      expect(presenter.stateValue.appLocale, const Locale('en'));
      expect(
        presenter.stateValue.selectedLanguage,
        const SpecificLanguage(Locale('en')),
      );
    });

    test('remembers the language the user chooses', () {
      final remembered = <String?>[];
      final presenter = SettingsPresenter(
        rememberLanguage: (languageCode) async => remembered.add(languageCode),
      );

      presenter.choose(const SpecificLanguage(Locale('pt')));

      expect(remembered, ['pt']);
    });

    test('starts in the language the user chose before', () {
      final presenter = SettingsPresenter(rememberedLanguage: 'en');

      expect(presenter.stateValue.appLocale, const Locale('en'));
      expect(
        presenter.stateValue.selectedLanguage,
        const SpecificLanguage(Locale('en')),
      );
    });

    test('forgets the chosen language when the user goes back to the system '
        'language', () {
      final remembered = <String?>[];
      final presenter = SettingsPresenter(
        rememberedLanguage: 'en',
        rememberLanguage: (languageCode) async => remembered.add(languageCode),
      );

      presenter.choose(const SystemLanguage());

      expect(presenter.stateValue.appLocale, isNull);
      expect(remembered, [null]);
    });

    test('follows the system language when the remembered one is no longer '
        'spoken', () {
      final presenter = SettingsPresenter(rememberedLanguage: 'fr');

      expect(presenter.stateValue.appLocale, isNull);
      expect(presenter.stateValue.selectedLanguage, const SystemLanguage());
    });
  });
}
