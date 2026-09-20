import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/roster.dart';

final _ana = studentSummary(id: '1', name: 'Ana Lima', location: 'Alfa');
final _bruno = studentSummary(id: '2', name: 'Bruno Costa', location: 'Beta');
final _carla = studentSummary(id: '3', name: 'Carla Dias', location: 'Alfa');
final _everyone = [_ana, _bruno, _carla];

Finder get _searchField => find.byType(TextField);
Finder get _dialog => find.byType(AlertDialog);
Finder inDialog(Finder finder) =>
    find.descendant(of: _dialog, matching: finder);

/// Types [text] and lets the search settle: the screen filters only after a
/// pause in typing, and a pending timer does not by itself schedule a frame.
Future<void> search(WidgetTester tester, String text) async {
  await tester.enterText(_searchField, text);
  await tester.pump(const Duration(seconds: 1));
  await tester.pumpAndSettle();
}

Future<void> openLocationPicker(WidgetTester tester) async {
  await tester.tap(find.byIcon(Icons.filter_list));
  await tester.pumpAndSettle();
}

Future<void> pickLocations(WidgetTester tester, List<String> locations) async {
  await openLocationPicker(tester);
  for (final location in locations) {
    await tester.tap(find.widgetWithText(CheckboxListTile, location));
    await tester.pumpAndSettle();
  }
  await tester.tap(find.text('Aplicar'));
  await tester.pumpAndSettle();
}

void main() {
  group('searching by name', () {
    testWidgets('narrows the list to the matching students', (tester) async {
      await pumpRoster(tester, _everyone);

      await search(tester, 'bruno');

      expect(find.text('Bruno Costa'), findsOneWidget);
      expect(find.text('Ana Lima'), findsNothing);
      expect(find.text('Carla Dias'), findsNothing);
    });

    testWidgets('waits for a pause in typing before filtering', (tester) async {
      await pumpRoster(tester, _everyone);

      await tester.enterText(_searchField, 'bruno');
      await tester.pump(const Duration(milliseconds: 100));
      expect(find.text('Ana Lima'), findsOneWidget);

      await tester.pump(const Duration(milliseconds: 300));
      expect(find.text('Ana Lima'), findsNothing);
    });

    testWidgets('offers a clear button only while there is a search', (
      tester,
    ) async {
      await pumpRoster(tester, _everyone);
      expect(find.byIcon(Icons.clear), findsNothing);

      await search(tester, 'bruno');
      expect(find.byIcon(Icons.clear), findsOneWidget);

      await tester.tap(find.byIcon(Icons.clear));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.clear), findsNothing);
      expect(find.text('Ana Lima'), findsOneWidget);
      expect(tester.widget<TextField>(_searchField).controller?.text, isEmpty);
    });

    testWidgets('says when nothing matches and offers to clear the filters', (
      tester,
    ) async {
      await pumpRoster(tester, _everyone);

      await search(tester, 'zzzz');

      expect(find.text('Nenhum resultado para "zzzz"'), findsOneWidget);
      expect(find.text('Ana Lima'), findsNothing);

      await tester.tap(find.text('Limpar filtros'));
      await tester.pumpAndSettle();

      expect(find.text('Ana Lima'), findsOneWidget);
      expect(tester.widget<TextField>(_searchField).controller?.text, isEmpty);
    });

    testWidgets('remembers the search when the screen is shown again', (
      tester,
    ) async {
      final presenter = presenterAnswering([loaded(_everyone)]);
      await presenter.load();
      presenter.filter(nameQuery: 'bruno');

      await pumpStudents(tester, presenter);

      expect(tester.widget<TextField>(_searchField).controller?.text, 'bruno');
      expect(find.text('Ana Lima'), findsNothing);
    });
  });

  group('filtering by location', () {
    testWidgets('offers every location once, in alphabetical order', (
      tester,
    ) async {
      await pumpRoster(tester, [_bruno, _carla, _ana]);

      await openLocationPicker(tester);

      final tiles = tester
          .widgetList<CheckboxListTile>(find.byType(CheckboxListTile))
          .map((tile) => (tile.title as Text).data)
          .toList();
      expect(tiles, ['Alfa', 'Beta']);
    });

    testWidgets('applying a selection shows only those locations', (
      tester,
    ) async {
      await pumpRoster(tester, _everyone);

      await pickLocations(tester, ['Alfa']);

      expect(find.text('Ana Lima'), findsOneWidget);
      expect(find.text('Carla Dias'), findsOneWidget);
      expect(find.text('Bruno Costa'), findsNothing);
      expect(find.widgetWithText(InputChip, 'Alfa'), findsOneWidget);
    });

    testWidgets('the button counts the selected locations', (tester) async {
      await pumpRoster(tester, _everyone);
      expect(find.text('Filtrar por local'), findsOneWidget);

      await pickLocations(tester, ['Alfa']);
      expect(find.text('1 local'), findsOneWidget);

      await pickLocations(tester, ['Beta']);
      expect(find.text('2 locais'), findsOneWidget);
    });

    testWidgets('cancelling leaves the filter as it was', (tester) async {
      await pumpRoster(tester, _everyone);

      await openLocationPicker(tester);
      await tester.tap(find.widgetWithText(CheckboxListTile, 'Alfa'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Cancelar'));
      await tester.pumpAndSettle();

      expect(_dialog, findsNothing);
      expect(find.text('Bruno Costa'), findsOneWidget);
      expect(find.text('Filtrar por local'), findsOneWidget);
    });

    testWidgets('unticking a location removes it from the selection', (
      tester,
    ) async {
      await pumpRoster(tester, _everyone);

      await openLocationPicker(tester);
      await tester.tap(find.widgetWithText(CheckboxListTile, 'Alfa'));
      await tester.pumpAndSettle();
      await tester.tap(find.widgetWithText(CheckboxListTile, 'Alfa'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Aplicar'));
      await tester.pumpAndSettle();

      expect(find.text('Bruno Costa'), findsOneWidget);
      expect(find.byType(InputChip), findsNothing);
    });

    testWidgets('clearing inside the picker unticks everything', (
      tester,
    ) async {
      await pumpRoster(tester, _everyone);
      await pickLocations(tester, ['Alfa']);

      await openLocationPicker(tester);
      await tester.tap(inDialog(find.text('Limpar')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Aplicar'));
      await tester.pumpAndSettle();

      expect(find.byType(InputChip), findsNothing);
      expect(find.text('Bruno Costa'), findsOneWidget);
    });

    testWidgets('removing a location chip drops that location', (tester) async {
      await pumpRoster(tester, _everyone);
      await pickLocations(tester, ['Alfa', 'Beta']);

      await tester.tap(
        find.descendant(
          of: find.widgetWithText(InputChip, 'Alfa'),
          matching: find.byType(Icon),
        ),
      );
      await tester.pumpAndSettle();

      expect(find.widgetWithText(InputChip, 'Alfa'), findsNothing);
      expect(find.widgetWithText(InputChip, 'Beta'), findsOneWidget);
      expect(find.text('Bruno Costa'), findsOneWidget);
      expect(find.text('Ana Lima'), findsNothing);
    });

    testWidgets('says so when there are no locations to choose from', (
      tester,
    ) async {
      await pumpRoster(tester, [studentSummary(location: '')]);

      await openLocationPicker(tester);

      expect(find.text('Nenhum local disponível.'), findsOneWidget);
    });
  });

  group('clearing every filter at once', () {
    testWidgets('resets both the search and the locations', (tester) async {
      await pumpRoster(tester, _everyone);
      await pickLocations(tester, ['Alfa']);
      await search(tester, 'ana');
      expect(find.text('Carla Dias'), findsNothing);

      await tester.tap(find.text('Limpar'));
      await tester.pumpAndSettle();

      expect(find.byType(InputChip), findsNothing);
      expect(find.text('Filtrar por local'), findsOneWidget);
      expect(find.text('Bruno Costa'), findsOneWidget);
      expect(tester.widget<TextField>(_searchField).controller?.text, isEmpty);
    });

    testWidgets('is not offered while nothing is filtered', (tester) async {
      await pumpRoster(tester, _everyone);

      expect(find.text('Limpar'), findsNothing);
    });
  });

  group('when the filters leave nothing', () {
    testWidgets('names the locations that were searched', (tester) async {
      await pumpRoster(tester, _everyone);
      await pickLocations(tester, ['Alfa']);

      await search(tester, 'zzzz');

      expect(find.text('Nenhum resultado para "zzzz"'), findsOneWidget);
      expect(find.text('em Alfa'), findsOneWidget);
    });

    testWidgets('still explains itself when only a location is selected', (
      tester,
    ) async {
      final presenter = presenterAnswering([
        loaded(_everyone),
        loaded([_bruno]),
      ]);
      await pumpStudents(tester, presenter);
      await pickLocations(tester, ['Alfa']);

      await presenter.load();
      await tester.pumpAndSettle();

      expect(find.text('Nenhum resultado'), findsOneWidget);
      expect(find.text('em Alfa'), findsOneWidget);
    });
  });

  group('without any students', () {
    testWidgets('says there are none', (tester) async {
      await pumpRoster(tester, []);

      expect(find.text('Nenhum aluno disponível.'), findsOneWidget);
    });
  });
}
