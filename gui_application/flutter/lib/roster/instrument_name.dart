import 'package:flutter_application/l10n/app_localizations.dart';
import 'package:flutter_application/roster/instrument.dart';

extension InstrumentName on AppLocalizations {
  String reportedInstrumentName(ReportedInstrument instrument) =>
      switch (instrument) {
        KnownInstrument(:final instrument) => instrumentName(instrument),
        UnrecognizedInstrument(:final raw) => raw,
      };

  String instrumentName(Instrument instrument) => switch (instrument) {
    Instrument.violin => instrumentViolin,
    Instrument.viola => instrumentViola,
    Instrument.cello => instrumentCello,
    Instrument.flute => instrumentFlute,
    Instrument.oboe => instrumentOboe,
    Instrument.bassoon => instrumentBassoon,
    Instrument.clarinet => instrumentClarinet,
    Instrument.altoClarinet => instrumentAltoClarinet,
    Instrument.bassClarinet => instrumentBassClarinet,
    Instrument.altoSaxophone => instrumentAltoSaxophone,
    Instrument.curvedSopranoSaxophone => instrumentCurvedSopranoSaxophone,
    Instrument.straightSopranoSaxophone => instrumentStraightSopranoSaxophone,
    Instrument.tenorSaxophone => instrumentTenorSaxophone,
    Instrument.trumpet => instrumentTrumpet,
    Instrument.cornet => instrumentCornet,
    Instrument.flugelhorn => instrumentFlugelhorn,
    Instrument.frenchHorn => instrumentFrenchHorn,
    Instrument.trombone => instrumentTrombone,
    Instrument.euphonium => instrumentEuphonium,
    Instrument.tuba => instrumentTuba,
    Instrument.englishHorn => instrumentEnglishHorn,
    Instrument.contraltoViolin => instrumentContraltoViolin,
  };
}
