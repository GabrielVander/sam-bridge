import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_test/flutter_test.dart';

const Locale portuguese = Locale('pt');
const Locale brazilianPortuguese = Locale('pt', 'BR');

void setOsLocale(WidgetTester tester, Locale locale) {
  tester.platformDispatcher.localesTestValue = [locale];
  addTearDown(tester.platformDispatcher.clearLocalesTestValue);
}

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
