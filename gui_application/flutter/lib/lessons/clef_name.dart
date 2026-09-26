import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/lessons/lessons_view_models.dart';

extension ClefName on AppLocalizations {
  String clefName(Clef clef) => switch (clef) {
    Clef.g => clefG,
    Clef.c => clefC,
    Clef.f => clefF,
  };
}
