import 'dart:async';

import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/roster/instrument_name.dart';
import 'package:flutter_application/roster/position_name.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_application/widgets/loading_indicator.dart';
import 'package:go_router/go_router.dart';

class StudentsScreen extends StatefulWidget {
  const StudentsScreen({super.key});

  @override
  State<StudentsScreen> createState() => _StudentsScreenState();
}

final class _StudentsScreenState extends State<StudentsScreen> {
  final _searchController = TextEditingController();
  Timer? _debounce;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final state = _presenter.stateValue;
      if (state is StudentsIdle) {
        _presenter.load();
      } else if (state is StudentsLoaded) {
        _searchController.text = state.nameQuery;
      }
    });
  }

  @override
  void dispose() {
    _searchController.dispose();
    _debounce?.cancel();
    super.dispose();
  }

  StudentsPresenter get _presenter => context.read<StudentsPresenter>();

  @override
  Widget build(BuildContext context) {
    return BlocSignalBuilder<StudentsPresenter, StudentsState>(
      builder: (context, state) => switch (state) {
        StudentsLoading() => const LoadingIndicator(),
        StudentsLoaded() => _loaded(state),
        StudentsFailure(:final report) => ErrorPanel(
          report: report,
          onRetry: () => _presenter.load(),
        ),
        _ => const SizedBox.shrink(),
      },
    );
  }

  Widget _loaded(StudentsLoaded state) => Column(
    children: [
      _SearchField(
        controller: _searchController,
        hasQuery: state.nameQuery.isNotEmpty,
        onChanged: _onSearchChanged,
        onCleared: _clearSearch,
      ),
      _LocationFilterBar(
        selectedLocations: state.selectedLocations,
        isFiltering:
            state.nameQuery.isNotEmpty || state.selectedLocations.isNotEmpty,
        onPickLocations: () =>
            _pickLocations(state.availableLocations, state.selectedLocations),
        onClearFilters: _clearFilters,
      ),
      if (state.selectedLocations.isNotEmpty)
        _SelectedLocationChips(
          selectedLocations: state.selectedLocations,
          onChanged: (locations) =>
              _presenter.filter(selectedLocations: locations),
        ),
      const SizedBox(height: 12),
      const Divider(height: 1),
      Expanded(
        child: _StudentsListContent(
          students: state.students,
          allStudents: state.allStudents,
          nameQuery: state.nameQuery,
          selectedLocations: state.selectedLocations,
          onClearFilters: _clearFilters,
        ),
      ),
    ],
  );

  void _onSearchChanged(String value) {
    _debounce?.cancel();
    _debounce = Timer(const Duration(milliseconds: 250), () {
      if (!mounted) return;
      _presenter.filter(nameQuery: value);
    });
  }

  void _clearSearch() {
    _searchController.clear();
    _presenter.filter(nameQuery: '');
  }

  void _clearFilters() {
    _searchController.clear();
    _presenter.clearFilters();
  }

  Future<void> _pickLocations(
    List<String> available,
    Set<String> selected,
  ) async {
    final result = await showDialog<Set<String>>(
      context: context,
      builder: (_) =>
          _LocationPickerDialog(available: available, selected: selected),
    );
    if (result != null && mounted) {
      _presenter.filter(selectedLocations: result);
    }
  }
}

final class _SearchField extends StatelessWidget {
  final TextEditingController controller;
  final bool hasQuery;
  final ValueChanged<String> onChanged;
  final VoidCallback onCleared;

  const _SearchField({
    required this.controller,
    required this.hasQuery,
    required this.onChanged,
    required this.onCleared,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 12),
      child: TextField(
        controller: controller,
        decoration: InputDecoration(
          prefixIcon: const Icon(Icons.search),
          hintText: context.l10n.rosterSearchHint,
          suffixIcon: hasQuery
              ? IconButton(icon: const Icon(Icons.clear), onPressed: onCleared)
              : null,
        ),
        textInputAction: TextInputAction.search,
        onChanged: onChanged,
      ),
    );
  }
}

final class _LocationFilterBar extends StatelessWidget {
  final Set<String> selectedLocations;
  final bool isFiltering;
  final VoidCallback onPickLocations;
  final VoidCallback onClearFilters;

  const _LocationFilterBar({
    required this.selectedLocations,
    required this.isFiltering,
    required this.onPickLocations,
    required this.onClearFilters,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: [
          Expanded(
            child: OutlinedButton.icon(
              icon: const Icon(Icons.filter_list, size: 18),
              label: Text(
                selectedLocations.isEmpty
                    ? context.l10n.rosterFilterByLocation
                    : context.l10n.rosterSelectedLocations(
                        selectedLocations.length,
                      ),
              ),
              onPressed: onPickLocations,
            ),
          ),
          if (isFiltering) ...[
            const SizedBox(width: 8),
            TextButton(
              onPressed: onClearFilters,
              child: Text(context.l10n.clear),
            ),
          ],
        ],
      ),
    );
  }
}

final class _SelectedLocationChips extends StatelessWidget {
  final Set<String> selectedLocations;
  final ValueChanged<Set<String>> onChanged;

  const _SelectedLocationChips({
    required this.selectedLocations,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
      child: Wrap(
        spacing: 8,
        runSpacing: 8,
        children: [
          for (final location in selectedLocations)
            InputChip(
              label: Text(location),
              onDeleted: () => onChanged(
                Set<String>.from(selectedLocations)..remove(location),
              ),
            ),
        ],
      ),
    );
  }
}

final class _LocationPickerDialog extends StatefulWidget {
  final List<String> available;
  final Set<String> selected;

  const _LocationPickerDialog({
    required this.available,
    required this.selected,
  });

  @override
  State<_LocationPickerDialog> createState() => _LocationPickerDialogState();
}

final class _LocationPickerDialogState extends State<_LocationPickerDialog> {
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

final class _StudentsListContent extends StatelessWidget {
  final List<StudentListItem> students;
  final List<StudentListItem> allStudents;
  final String nameQuery;
  final Set<String> selectedLocations;
  final VoidCallback onClearFilters;

  const _StudentsListContent({
    required this.students,
    required this.allStudents,
    required this.nameQuery,
    required this.selectedLocations,
    required this.onClearFilters,
  });

  @override
  Widget build(BuildContext context) {
    if (allStudents.isEmpty) {
      return Center(child: Text(context.l10n.rosterNoStudents));
    }
    if (students.isEmpty) {
      return _NoResults(
        nameQuery: nameQuery,
        selectedLocations: selectedLocations,
        onClearFilters: onClearFilters,
      );
    }
    return _StudentsList(students);
  }
}

final class _NoResults extends StatelessWidget {
  final String nameQuery;
  final Set<String> selectedLocations;
  final VoidCallback onClearFilters;

  const _NoResults({
    required this.nameQuery,
    required this.selectedLocations,
    required this.onClearFilters,
  });

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.search_off, size: 48),
            const SizedBox(height: 12),
            Text(
              nameQuery.isNotEmpty
                  ? context.l10n.rosterNoResultsFor(nameQuery)
                  : context.l10n.rosterNoResults,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.titleMedium,
            ),
            if (selectedLocations.isNotEmpty) ...[
              const SizedBox(height: 4),
              Text(
                context.l10n.rosterInLocations(selectedLocations.join(', ')),
                textAlign: TextAlign.center,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ],
            const SizedBox(height: 16),
            FilledButton.tonal(
              onPressed: onClearFilters,
              child: Text(context.l10n.rosterClearFilters),
            ),
          ],
        ),
      ),
    );
  }
}

final class _StudentsList extends StatelessWidget {
  final List<StudentListItem> students;

  const _StudentsList(this.students);

  @override
  Widget build(BuildContext context) {
    return ListView.separated(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 16),
      itemCount: students.length,
      separatorBuilder: (_, _) => const SizedBox(height: 10),
      itemBuilder: (_, index) => _StudentCard(students[index]),
    );
  }
}

final class _StudentCard extends StatelessWidget {
  final StudentListItem student;

  const _StudentCard(this.student);

  bool get _canOpen => student.id.isNotEmpty;

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: InkWell(
        onTap: _canOpen
            ? () => context.go('/students/${student.id}', extra: student.name)
            : null,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          child: Row(
            children: [
              _Avatar(name: student.name),
              const SizedBox(width: 14),
              Expanded(child: _details(context)),
              if (_canOpen) ...[
                const SizedBox(width: 8),
                const Icon(Icons.chevron_right),
              ],
            ],
          ),
        ),
      ),
    );
  }

  Widget _details(BuildContext context) => Column(
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      Text(
        student.name,
        style: Theme.of(context).textTheme.titleMedium,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
      ),
      const SizedBox(height: 4),
      Text(
        context.l10n.positionName(student.position),
        style: Theme.of(context).textTheme.bodySmall,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
      ),
      if (student.instrument case final instrument?) ...[
        const SizedBox(height: 2),
        _IconLine(
          icon: Icons.music_note_outlined,
          text: context.l10n.reportedInstrumentName(instrument),
        ),
      ],
      const SizedBox(height: 2),
      _IconLine(icon: Icons.place_outlined, text: student.location),
    ],
  );
}

final class _Avatar extends StatelessWidget {
  final String name;

  const _Avatar({required this.name});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return CircleAvatar(
      radius: 22,
      backgroundColor: colors.primaryContainer,
      child: Text(
        name.isEmpty ? '?' : name.substring(0, 1).toUpperCase(),
        style: TextStyle(
          color: colors.onPrimaryContainer,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }
}

final class _IconLine extends StatelessWidget {
  final IconData icon;
  final String text;

  const _IconLine({required this.icon, required this.text});

  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.onSurfaceVariant;

    return Row(
      children: [
        ExcludeSemantics(child: Icon(icon, size: 14, color: color)),
        const SizedBox(width: 4),
        Expanded(
          child: Text(
            text,
            style: Theme.of(
              context,
            ).textTheme.bodySmall?.copyWith(color: color),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ),
      ],
    );
  }
}
