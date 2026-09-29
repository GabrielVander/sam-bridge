import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/errors.dart';
import '../support/roster.dart';

void main() {
  group('StudentsScreen row', () {
    final positionNames = {
      Positions.candidate: 'Candidato(a)',
      Positions.practice: 'Ensaio',
      Positions.youthService: 'Reunião de Jovens e Menores',
      Positions.officialService: 'Culto Oficial',
      Positions.officialized: 'Oficialização',
      Positions.youthServiceHalfHour: 'Reunião de Jovens e Menores / Meia Hora',
      Positions.gemSecretary: 'Secretário(a) do GEM',
      Positions.invalid('ALGO DESCONHECIDO'): 'ALGO DESCONHECIDO',
    };

    for (final MapEntry(key: position, value: name) in positionNames.entries) {
      testWidgets('names the $position position "$name"', (tester) async {
        await pumpRoster(tester, [studentSummary(position: position)]);

        expect(find.text(name), findsOneWidget);
      });
    }

    final portugueseInstrumentNames = {
      Instruments.violin: 'Violino',
      Instruments.viola: 'Viola',
      Instruments.cello: 'Violoncelo',
      Instruments.flute: 'Flauta',
      Instruments.oboe: 'Oboé',
      Instruments.bassoon: 'Fagote',
      Instruments.clarinet: 'Clarinete',
      Instruments.altoClarinet: 'Clarinete alto',
      Instruments.bassClarinet: 'Clarinete baixo',
      Instruments.altoSaxophone: 'Saxofone alto',
      Instruments.curvedSopranoSaxophone: 'Saxofone soprano curvo',
      Instruments.straightSopranoSaxophone: 'Saxofone soprano reto',
      Instruments.tenorSaxophone: 'Saxofone tenor',
      Instruments.trumpet: 'Trompete',
      Instruments.cornet: 'Cornet',
      Instruments.flugelhorn: 'Flugelhorn',
      Instruments.frenchHorn: 'Trompa',
      Instruments.trombone: 'Trombone',
      Instruments.euphonium: 'Eufônio',
      Instruments.tuba: 'Tuba',
      Instruments.englishHorn: 'Corne inglês',
      Instruments.contraltoViolin: 'Violino contralto',
      Instruments.unknown('BANDOLIM'): 'BANDOLIM',
    };

    for (final MapEntry(key: instrument, value: name)
        in portugueseInstrumentNames.entries) {
      testWidgets('names the $instrument instrument "$name"', (tester) async {
        await pumpRoster(tester, [studentSummary(instrument: instrument)]);

        expect(find.text(name), findsOneWidget);
      });
    }

    final englishInstrumentNames = {
      Instruments.violin: 'Violin',
      Instruments.viola: 'Viola',
      Instruments.cello: 'Cello',
      Instruments.flute: 'Flute',
      Instruments.oboe: 'Oboe',
      Instruments.bassoon: 'Bassoon',
      Instruments.clarinet: 'Clarinet',
      Instruments.altoClarinet: 'Alto clarinet',
      Instruments.bassClarinet: 'Bass clarinet',
      Instruments.altoSaxophone: 'Alto saxophone',
      Instruments.curvedSopranoSaxophone: 'Curved soprano saxophone',
      Instruments.straightSopranoSaxophone: 'Straight soprano saxophone',
      Instruments.tenorSaxophone: 'Tenor saxophone',
      Instruments.trumpet: 'Trumpet',
      Instruments.cornet: 'Cornet',
      Instruments.flugelhorn: 'Flugelhorn',
      Instruments.frenchHorn: 'French horn',
      Instruments.trombone: 'Trombone',
      Instruments.euphonium: 'Euphonium',
      Instruments.tuba: 'Tuba',
      Instruments.englishHorn: 'English horn',
      Instruments.contraltoViolin: 'Contralto violin',
    };

    for (final MapEntry(key: instrument, value: name)
        in englishInstrumentNames.entries) {
      testWidgets('names the $instrument instrument "$name" in English', (
        tester,
      ) async {
        await pumpRoster(tester, [
          studentSummary(instrument: instrument),
        ], locale: const Locale('en'));

        expect(find.text(name), findsOneWidget);
      });
    }

    testWidgets('shows the instrument with a music note icon', (tester) async {
      await pumpRoster(tester, [
        studentSummary(instrument: Instruments.tenorSaxophone),
      ]);

      expect(find.text('Saxofone tenor'), findsOneWidget);
      expect(find.byIcon(Icons.music_note_outlined), findsOneWidget);
    });

    testWidgets('puts the instrument between position and location', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(instrument: Instruments.oboe)]);

      final position = tester.getTopLeft(find.text('Ensaio')).dy;
      final instrument = tester.getTopLeft(find.text('Oboé')).dy;
      final location = tester.getTopLeft(find.text('Some Location')).dy;

      expect(position, lessThan(instrument));
      expect(instrument, lessThan(location));
    });

    testWidgets('shows no instrument line when there is no instrument', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary()]);

      expect(find.byIcon(Icons.music_note_outlined), findsNothing);
      expect(find.text('Ensaio'), findsOneWidget);
      expect(find.text('Some Location'), findsOneWidget);
    });

    testWidgets('keeps a very long instrument name on one line', (
      tester,
    ) async {
      tester.view.physicalSize = const Size(360, 800);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.reset);
      final longName = List.filled(12, 'INSTRUMENTO').join(' ');

      await pumpRoster(tester, [
        studentSummary(instrument: Instruments.unknown(longName)),
      ]);

      final text = tester.widget<Text>(find.textContaining('INSTRUMENTO'));
      expect(text.maxLines, 1);
      expect(text.overflow, TextOverflow.ellipsis);
      expect(tester.takeException(), isNull);
    });

    testWidgets('hides the decorative icon from screen readers', (
      tester,
    ) async {
      await pumpRoster(tester, [
        studentSummary(instrument: Instruments.violin),
      ]);

      expect(
        find.ancestor(
          of: find.byIcon(Icons.music_note_outlined),
          matching: find.byType(ExcludeSemantics),
        ),
        findsOneWidget,
      );
    });
  });

  group('StudentsScreen failure', () {
    testWidgets('loads the students again when retrying after a failure', (
      tester,
    ) async {
      await pumpStudents(
        tester,
        presenterAnswering([
          rosterFailed(networkFailure('connection refused')),
          rosterLoaded([studentSummary(name: 'Jane Doe')]),
        ]),
      );
      expect(find.text('Tentar novamente'), findsOneWidget);

      await tester.tap(find.text('Tentar novamente'));
      await tester.pumpAndSettle();

      expect(find.text('Tentar novamente'), findsNothing);
      expect(find.text('Jane Doe'), findsOneWidget);
    });
  });

  group('StudentsScreen navigation', () {
    testWidgets('tapping a student opens their page, passing the name along', (
      tester,
    ) async {
      await pumpRoster(tester, [
        studentSummary(id: '500132', name: 'Jane Doe'),
      ]);

      await tester.tap(find.text('Jane Doe'));
      await tester.pumpAndSettle();

      expect(find.text('página do aluno 500132 (Jane Doe)'), findsOneWidget);
    });

    testWidgets('a student without an id cannot be opened', (tester) async {
      await pumpRoster(tester, [studentSummary(id: '', name: 'Jane Doe')]);

      await tester.tap(find.text('Jane Doe'));
      await tester.pumpAndSettle();

      expect(find.textContaining('página do aluno'), findsNothing);
      expect(find.byIcon(Icons.chevron_right), findsNothing);
    });

    testWidgets('a student with an id shows that the row can be opened', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(id: '1')]);

      expect(find.byIcon(Icons.chevron_right), findsOneWidget);
    });
  });

  group('StudentsScreen avatar', () {
    testWidgets('shows the capitalised first letter of the name', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(name: 'jane doe')]);

      expect(find.text('J'), findsOneWidget);
    });

    testWidgets('keeps a first character made of two code units whole', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(name: '𝄞 Clave')]);

      expect(find.text('𝄞'), findsOneWidget);
    });

    testWidgets('shows a question mark when the name is empty', (tester) async {
      await pumpRoster(tester, [studentSummary(name: '')]);

      expect(find.text('?'), findsOneWidget);
    });
  });
}
