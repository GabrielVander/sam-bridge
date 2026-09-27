import 'package:equatable/equatable.dart';

enum Instrument {
  violin,
  viola,
  cello,
  flute,
  oboe,
  bassoon,
  clarinet,
  altoClarinet,
  bassClarinet,
  altoSaxophone,
  curvedSopranoSaxophone,
  straightSopranoSaxophone,
  tenorSaxophone,
  trumpet,
  cornet,
  flugelhorn,
  frenchHorn,
  trombone,
  euphonium,
  tuba,
  englishHorn,
  contraltoViolin,
}

sealed class ReportedInstrument extends Equatable {
  const ReportedInstrument();
}

final class KnownInstrument extends ReportedInstrument {
  final Instrument instrument;

  const KnownInstrument(this.instrument);

  @override
  List<Object?> get props => [instrument];
}

final class UnrecognizedInstrument extends ReportedInstrument {
  final String raw;

  const UnrecognizedInstrument(this.raw);

  @override
  List<Object?> get props => [raw];
}
