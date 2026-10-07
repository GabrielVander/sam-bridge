import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/shared/level.dart';

extension LevelName on AppLocalizations {
  String levelName(Level level) => switch (level) {
    Level.candidate => levelCandidate,
    Level.practice => levelPractice,
    Level.youthService => levelYouthService,
    Level.officialService => levelOfficialService,
    Level.officialized => levelOfficialized,
    Level.halfHour => levelHalfHour,
  };
}
