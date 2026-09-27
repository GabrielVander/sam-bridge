import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';

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
  late final Set<String> _chosen = Set<String>.from(widget.selected);

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.l10n.rosterFilterByLocation),
      content: SizedBox(
        width: double.maxFinite,
        child: widget.available.isEmpty
            ? Text(context.l10n.rosterNoLocations)
            : ListView(
                shrinkWrap: true,
                children: [
                  for (final location in widget.available)
                    CheckboxListTile(
                      title: Text(location),
                      value: _chosen.contains(location),
                      onChanged: (checked) => _toggle(location, checked),
                    ),
                ],
              ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: Text(context.l10n.cancel),
        ),
        TextButton(
          onPressed: () => setState(_chosen.clear),
          child: Text(context.l10n.clear),
        ),
        FilledButton(
          onPressed: () => Navigator.pop(context, _chosen),
          child: Text(context.l10n.apply),
        ),
      ],
    );
  }

  void _toggle(String location, bool? checked) => setState(() {
    if (checked == true) {
      _chosen.add(location);
    } else {
      _chosen.remove(location);
    }
  });
}
