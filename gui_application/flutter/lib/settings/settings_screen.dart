import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/settings/settings_presenter.dart';
import 'package:go_router/go_router.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSignalBuilder<SettingsPresenter, SettingsState>(
      builder: (context, state) => ListView(
        padding: const EdgeInsets.fromLTRB(8, 8, 8, 16),
        children: [_header(context), _languages(context, state)],
      ),
    );
  }

  Widget _header(BuildContext context) => Row(
    children: [
      IconButton(
        tooltip: context.l10n.back,
        icon: const Icon(Icons.arrow_back),
        onPressed: () => context.pop(),
      ),
      const SizedBox(width: 4),
      Text(
        context.l10n.settings,
        style: Theme.of(context).textTheme.titleMedium,
      ),
    ],
  );

  Widget _languages(BuildContext context, SettingsState state) => Card(
    child: RadioGroup<Language>(
      groupValue: state.selectedLanguage,
      onChanged: (language) =>
          context.read<SettingsPresenter>().choose(language!),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ListTile(title: Text(context.l10n.language)),
          for (final Language language in state.languages)
            RadioListTile<Language>(
              value: language,
              title: Text(_nameOf(context, language)),
            ),
        ],
      ),
    ),
  );

  String _nameOf(BuildContext context, Language language) => switch (language) {
    SystemLanguage() => context.l10n.systemLanguage,
    SpecificLanguage(:final locale) => lookupAppLocalizations(
      locale,
    ).languageName,
  };
}
