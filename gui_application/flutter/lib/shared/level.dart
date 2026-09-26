import 'package:equatable/equatable.dart';

enum Level {
  candidate,
  practice,
  youthService,
  officialService,
  officialized,
  halfHour,
}

sealed class ReportedLevel extends Equatable {
  const ReportedLevel();
}

final class KnownLevel extends ReportedLevel {
  final Level level;

  const KnownLevel(this.level);

  @override
  List<Object?> get props => [level];
}

final class UnrecognizedLevel extends ReportedLevel {
  final String raw;

  const UnrecognizedLevel(this.raw);

  @override
  List<Object?> get props => [raw];
}
