import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/roster/location_picker_presenter.dart';

final class LocationPickerDialog extends StatefulWidget {
  final List<String> available;
  final Set<String> selected;

  const LocationPickerDialog({
    super.key,
    required this.available,
    required this.selected,
  });

  @override
  State<LocationPickerDialog> createState() => _State();
}

final class _State extends State<LocationPickerDialog> {
  late final LocationPickerPresenter _presenter = LocationPickerPresenter(
    available: widget.available,
    chosen: widget.selected,
  );

  @override
  void dispose() {
    _presenter.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.l10n.rosterFilterByLocation),
      content: SizedBox(
        width: double.maxFinite,
        child: BlocSignalBuilder<LocationPickerPresenter, LocationPickerState>(
          bloc: _presenter,
          builder: (context, state) => switch (state) {
            NoLocationsToChoose() => Text(context.l10n.rosterNoLocations),
            LocationChoices(:final choices) => ListView(
              shrinkWrap: true,
              children: [
                for (final choice in choices)
                  CheckboxListTile(
                    title: Text(choice.location),
                    value: choice.chosen,
                    onChanged: (_) => _presenter.toggle(choice.location),
                  ),
              ],
            ),
          },
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: Text(context.l10n.cancel),
        ),
        TextButton(
          onPressed: _presenter.clear,
          child: Text(context.l10n.clear),
        ),
        FilledButton(
          onPressed: () => Navigator.pop(context, _presenter.chosen),
          child: Text(context.l10n.apply),
        ),
      ],
    );
  }
}
