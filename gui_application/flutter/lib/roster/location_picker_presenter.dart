import 'package:bloc_signals/bloc_signals.dart';

class LocationPickerPresenter extends CubitSignal<LocationPickerState> {
  final List<String> _available;
  final Set<String> _chosen;

  LocationPickerPresenter({
    required List<String> available,
    required Set<String> chosen,
  }) : _available = available,
       _chosen = {...chosen},
       super(initialState: _stateOf(available, chosen));

  Set<String> get chosen => Set.unmodifiable(_chosen);

  void toggle(String location) {
    if (!_chosen.remove(location)) _chosen.add(location);
    emit(_stateOf(_available, _chosen));
  }

  void clear() {
    _chosen.clear();
    emit(_stateOf(_available, _chosen));
  }

  static LocationPickerState _stateOf(
    List<String> available,
    Set<String> chosen,
  ) => available.isEmpty
      ? const NoLocationsToChoose()
      : LocationChoices([
          for (final location in available)
            LocationChoice(
              location: location,
              chosen: chosen.contains(location),
            ),
        ]);
}

sealed class LocationPickerState {
  const LocationPickerState();
}

final class NoLocationsToChoose extends LocationPickerState {
  const NoLocationsToChoose();
}

final class LocationChoices extends LocationPickerState {
  final List<LocationChoice> choices;
  const LocationChoices(this.choices);
}

final class LocationChoice {
  final String location;
  final bool chosen;
  const LocationChoice({required this.location, required this.chosen});
}
