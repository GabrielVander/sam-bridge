import 'package:flutter_application/roster/instrument.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/rust/api/roster.dart';
import 'package:flutter_application/roster/student_position.dart';
import 'package:flutter_application/shared/level.dart';

class RosterMapper {
  static List<StudentListItem> toViewModels(List<StudentSummaryDto> dtos) =>
      dtos.map(toViewModel).toList();

  static StudentListItem toViewModel(StudentSummaryDto dto) => StudentListItem(
    id: dto.id,
    name: dto.name,
    initial: _initial(dto.name),
    location: dto.location,
    position: _position(dto.position),
    instrument: _instrument(dto.instrument),
    canOpen: dto.id.isNotEmpty,
  );

  static String _initial(String name) =>
      name.isEmpty ? '?' : String.fromCharCode(name.runes.first).toUpperCase();

  static StudentPosition _position(
    StudentPositionDto position,
  ) => switch (position) {
    StudentPositionDto_Candidate() => const LeveledPosition([Level.candidate]),
    StudentPositionDto_Practice() => const LeveledPosition([Level.practice]),
    StudentPositionDto_YouthService() => const LeveledPosition([
      Level.youthService,
    ]),
    StudentPositionDto_OfficialService() => const LeveledPosition([
      Level.officialService,
    ]),
    StudentPositionDto_Officialized() => const LeveledPosition([
      Level.officialized,
    ]),
    StudentPositionDto_YouthServiceHalfHour() => const LeveledPosition([
      Level.youthService,
      Level.halfHour,
    ]),
    StudentPositionDto_YouthServicePractice() => const LeveledPosition([
      Level.youthService,
      Level.practice,
    ]),
    StudentPositionDto_GemSecretary() => const GemSecretaryPosition(),
    StudentPositionDto_Invalid(:final raw) => UnrecognizedPosition(raw),
  };

  static ReportedInstrument? _instrument(
    InstrumentDto? instrument,
  ) => switch (instrument) {
    null => null,
    InstrumentDto_Violin() => const KnownInstrument(Instrument.violin),
    InstrumentDto_Viola() => const KnownInstrument(Instrument.viola),
    InstrumentDto_Cello() => const KnownInstrument(Instrument.cello),
    InstrumentDto_Flute() => const KnownInstrument(Instrument.flute),
    InstrumentDto_Oboe() => const KnownInstrument(Instrument.oboe),
    InstrumentDto_Bassoon() => const KnownInstrument(Instrument.bassoon),
    InstrumentDto_Clarinet() => const KnownInstrument(Instrument.clarinet),
    InstrumentDto_AltoClarinet() => const KnownInstrument(
      Instrument.altoClarinet,
    ),
    InstrumentDto_BassClarinet() => const KnownInstrument(
      Instrument.bassClarinet,
    ),
    InstrumentDto_AltoSaxophone() => const KnownInstrument(
      Instrument.altoSaxophone,
    ),
    InstrumentDto_CurvedSopranoSaxophone() => const KnownInstrument(
      Instrument.curvedSopranoSaxophone,
    ),
    InstrumentDto_StraightSopranoSaxophone() => const KnownInstrument(
      Instrument.straightSopranoSaxophone,
    ),
    InstrumentDto_TenorSaxophone() => const KnownInstrument(
      Instrument.tenorSaxophone,
    ),
    InstrumentDto_Trumpet() => const KnownInstrument(Instrument.trumpet),
    InstrumentDto_Cornet() => const KnownInstrument(Instrument.cornet),
    InstrumentDto_Flugelhorn() => const KnownInstrument(Instrument.flugelhorn),
    InstrumentDto_FrenchHorn() => const KnownInstrument(Instrument.frenchHorn),
    InstrumentDto_Trombone() => const KnownInstrument(Instrument.trombone),
    InstrumentDto_Euphonium() => const KnownInstrument(Instrument.euphonium),
    InstrumentDto_Tuba() => const KnownInstrument(Instrument.tuba),
    InstrumentDto_EnglishHorn() => const KnownInstrument(
      Instrument.englishHorn,
    ),
    InstrumentDto_ContraltoViolin() => const KnownInstrument(
      Instrument.contraltoViolin,
    ),
    InstrumentDto_Unknown(:final raw) => UnrecognizedInstrument(raw),
  };
}
