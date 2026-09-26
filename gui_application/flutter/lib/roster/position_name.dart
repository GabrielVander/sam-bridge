import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/roster/student_position.dart';
import 'package:flutter_application/shared/level_name.dart';

extension PositionName on AppLocalizations {
  String positionName(StudentPosition position) => switch (position) {
    MusicianPosition(:final levels) => levels.map(levelName).join(' / '),
    GemSecretaryPosition() => positionGemSecretary,
    MusicSecretaryPosition() => positionMusicSecretary,
    UnrecognizedPosition(:final raw) => raw,
  };
}
