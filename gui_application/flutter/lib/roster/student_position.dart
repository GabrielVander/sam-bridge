import 'package:equatable/equatable.dart';
import 'package:flutter_application/shared/level.dart';

sealed class StudentPosition extends Equatable {
  const StudentPosition();

  @override
  List<Object?> get props => [];
}

final class MusicianPosition extends StudentPosition {
  final List<Level> levels;

  const MusicianPosition(this.levels);

  @override
  List<Object?> get props => [levels];
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

  @override
  List<Object?> get props => [raw];
}
