import 'package:flutter_application/roster/location_picker_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

List<(String, bool)> offered(LocationPickerPresenter presenter) =>
    switch (presenter.stateValue) {
      LocationChoices(:final choices) =>
        choices.map((choice) => (choice.location, choice.chosen)).toList(),
      NoLocationsToChoose() => [],
    };

void main() {
  group('LocationPickerPresenter', () {
    test('offers every location, marking the chosen ones', () {
      final presenter = LocationPickerPresenter(
        available: ['Alfa', 'Beta'],
        chosen: {'Beta'},
      );

      expect(offered(presenter), [('Alfa', false), ('Beta', true)]);
    });

    test('toggling a location flips whether it is chosen', () {
      final presenter = LocationPickerPresenter(
        available: ['Alfa', 'Beta'],
        chosen: {'Beta'},
      );

      presenter.toggle('Alfa');
      presenter.toggle('Beta');

      expect(offered(presenter), [('Alfa', true), ('Beta', false)]);
      expect(presenter.chosen, {'Alfa'});
    });

    test('clearing leaves no location chosen', () {
      final presenter = LocationPickerPresenter(
        available: ['Alfa', 'Beta'],
        chosen: {'Alfa', 'Beta'},
      );

      presenter.clear();

      expect(offered(presenter), [('Alfa', false), ('Beta', false)]);
      expect(presenter.chosen, isEmpty);
    });

    test('without locations there is nothing to choose', () {
      final presenter = LocationPickerPresenter(available: [], chosen: {});

      expect(presenter.stateValue, isA<NoLocationsToChoose>());
    });
  });
}
