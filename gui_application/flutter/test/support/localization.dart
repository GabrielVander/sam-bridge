import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/app_localizations.dart';

const Locale portuguese = Locale('pt');

MaterialApp localizedApp({required Widget home, Locale locale = portuguese}) =>
    MaterialApp(
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      locale: locale,
      home: home,
    );

MaterialApp localizedRouterApp(
  RouterConfig<Object> router, {
  Locale locale = portuguese,
}) => MaterialApp.router(
  localizationsDelegates: AppLocalizations.localizationsDelegates,
  supportedLocales: AppLocalizations.supportedLocales,
  locale: locale,
  routerConfig: router,
);
