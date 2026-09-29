import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/roster/widgets/location_picker_dialog.dart';
import 'package:flutter_application/roster/widgets/student_card.dart';
import 'package:flutter_application/widgets/error_panel.dart';
import 'package:flutter_application/widgets/loading_indicator.dart';

class StudentsScreen extends StatefulWidget {
  const StudentsScreen({super.key});

  @override
  State<StudentsScreen> createState() => _StudentsScreenState();
}

final class _StudentsScreenState extends State<StudentsScreen> {
  late final TextEditingController _searchController;

  @override
  void initState() {
    super.initState();
    _searchController = TextEditingController(text: _presenter.nameQuery);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _presenter.open();
    });
  }

  @override
  void dispose() {
    _searchController.dispose();
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
        onChanged: _presenter.search,
        onCleared: _clearSearch,
      ),
      _LocationFilterBar(
        selectedLocations: state.selectedLocations,
        isFiltering: state.isFiltering,
        onPickLocations: () =>
            _pickLocations(state.availableLocations, state.selectedLocations),
        onClearFilters: _clearFilters,
      ),
      if (state.selectedLocations.isNotEmpty)
        _SelectedLocationChips(
          selectedLocations: state.selectedLocations,
          onRemoved: _presenter.removeLocation,
        ),
      const SizedBox(height: 12),
      const Divider(height: 1),
      Expanded(
        child: switch (state.listing) {
          NoStudents() => Center(child: Text(context.l10n.rosterNoStudents)),
          NoMatches() => _NoResults(
            nameQuery: state.nameQuery,
            selectedLocations: state.selectedLocations,
            onClearFilters: _clearFilters,
          ),
          Matches(:final students) => _StudentsList(students),
        },
      ),
    ],
  );

  void _clearSearch() {
    _searchController.clear();
    _presenter.clearSearch();
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
          LocationPickerDialog(available: available, selected: selected),
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
  final ValueChanged<String> onRemoved;

  const _SelectedLocationChips({
    required this.selectedLocations,
    required this.onRemoved,
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
              onDeleted: () => onRemoved(location),
            ),
        ],
      ),
    );
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
      itemBuilder: (_, index) => StudentCard(students[index]),
    );
  }
}
