import 'package:flutter_application/shared/level.dart';

sealed class StudentPosition {
  const StudentPosition();
}

final class LeveledPosition extends StudentPosition {
  final List<Level> levels;

  const LeveledPosition(this.levels);
}

final class GemSecretaryPosition extends StudentPosition {
  const GemSecretaryPosition();
}

final class MusicSecretaryPosition extends StudentPosition {
  const MusicSecretaryPosition();
}

final class UnrecognizedPosition extends StudentPosition {
  final String raw;

  const UnrecognizedPosition(this.raw);
}
