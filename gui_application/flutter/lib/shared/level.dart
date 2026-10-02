sealed class ReportedLevel {
  const ReportedLevel();
}

final class KnownLevel extends ReportedLevel {
  final Level level;

  const KnownLevel(this.level);
}

final class UnrecognizedLevel extends ReportedLevel {
  final String raw;

  const UnrecognizedLevel(this.raw);
}

enum Level {
  candidate,
  practice,
  youthService,
  officialService,
  officialized,
  halfHour,
}
