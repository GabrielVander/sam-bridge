import 'dart:async';

import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/view_models.dart';
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
      final state = context.read<StudentsCubitSignal>().stateValue;
      if (state is StudentsIdle) {
        context.read<StudentsCubitSignal>().load();
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

  void _onSearchChanged(String value) {
    _debounce?.cancel();
    _debounce = Timer(const Duration(milliseconds: 250), () {
      if (!mounted) return;
      context.read<StudentsCubitSignal>().filter(nameQuery: value);
    });
  }

  Future<void> _showLocationPicker(List<String> available, Set<String> selected) async {
    final temp = Set<String>.from(selected);
    final result = await showDialog<Set<String>>(
      context: context,
      builder: (ctx) => StatefulBuilder(
        builder: (ctx, setState) => AlertDialog(
          title: const Text('Filtrar por local'),
          content: SizedBox(
            width: double.maxFinite,
            child: available.isEmpty
                ? const Text('Nenhum local disponível.')
                : ListView(
                    shrinkWrap: true,
                    children: [
                      for (final loc in available)
                        CheckboxListTile(
                          title: Text(loc),
                          value: temp.contains(loc),
                          onChanged: (v) => setState(() {
                            if (v == true) {
                              temp.add(loc);
                            } else {
                              temp.remove(loc);
                            }
                          }),
                        ),
                    ],
                  ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(ctx),
              child: const Text('Cancelar'),
            ),
            TextButton(
              onPressed: () {
                temp.clear();
                setState(() {});
              },
              child: const Text('Limpar'),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(ctx, temp),
              child: const Text('Aplicar'),
            ),
          ],
        ),
      ),
    );
    if (result != null && mounted) {
      context.read<StudentsCubitSignal>().filter(selectedLocations: result);
    }
  }

  @override
  Widget build(BuildContext context) {
    return BlocSignalBuilder<StudentsCubitSignal, StudentsState>(
      builder: (context, state) => switch (state) {
        StudentsLoading() => const Center(child: CircularProgressIndicator()),
        StudentsLoaded(
          :final students,
          :final allStudents,
          :final nameQuery,
          :final selectedLocations,
          :final availableLocations,
        ) =>
          Column(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(12, 12, 12, 8),
                child: TextField(
                  controller: _searchController,
                  decoration: InputDecoration(
                    prefixIcon: const Icon(Icons.search),
                    hintText: 'Buscar por nome…',
                    border: const OutlineInputBorder(),
                    isDense: true,
                    suffixIcon: nameQuery.isEmpty
                        ? null
                        : IconButton(
                            icon: const Icon(Icons.clear),
                            onPressed: () {
                              _searchController.clear();
                              context.read<StudentsCubitSignal>().filter(nameQuery: '');
                            },
                          ),
                  ),
                  textInputAction: TextInputAction.search,
                  onChanged: _onSearchChanged,
                ),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12),
                child: Row(
                  children: [
                    Expanded(
                      child: OutlinedButton.icon(
                        icon: const Icon(Icons.filter_list, size: 18),
                        label: Text(
                          selectedLocations.isEmpty
                              ? 'Filtrar por local'
                              : '${selectedLocations.length} ${selectedLocations.length == 1 ? 'local' : 'locais'}',
                        ),
                        onPressed: () => _showLocationPicker(availableLocations, selectedLocations),
                      ),
                    ),
                    if (nameQuery.isNotEmpty || selectedLocations.isNotEmpty) ...[
                      const SizedBox(width: 8),
                      TextButton(
                        onPressed: () {
                          _searchController.clear();
                          context.read<StudentsCubitSignal>().clearFilters();
                        },
                        child: const Text('Limpar'),
                      ),
                    ],
                  ],
                ),
              ),
              if (selectedLocations.isNotEmpty)
                Padding(
                  padding: const EdgeInsets.fromLTRB(12, 8, 12, 0),
                  child: Wrap(
                    spacing: 6,
                    runSpacing: 6,
                    children: [
                      for (final loc in selectedLocations)
                        InputChip(
                          label: Text(loc),
                          onDeleted: () {
                            final next = Set<String>.from(selectedLocations)..remove(loc);
                            context.read<StudentsCubitSignal>().filter(selectedLocations: next);
                          },
                        ),
                    ],
                  ),
                ),
              const Divider(height: 1),
              Expanded(
                child: _StudentsListContent(
                  students: students,
                  allStudents: allStudents,
                  nameQuery: nameQuery,
                  selectedLocations: selectedLocations,
                ),
              ),
            ],
          ),
        StudentsFailure(:final message) => Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.error_outline,
                size: 48,
                color: Theme.of(context).colorScheme.error,
              ),
              const SizedBox(height: 16),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 24),
                child: Text(message, textAlign: TextAlign.center),
              ),
              const SizedBox(height: 16),
              FilledButton.tonal(
                onPressed: () => context.read<StudentsCubitSignal>().load(),
                child: const Text('Tentar novamente'),
              ),
            ],
          ),
        ),
        _ => const SizedBox.shrink(),
      },
    );
  }
}

final class _StudentsListContent extends StatelessWidget {
  final List<StudentListItem> students;
  final List<StudentListItem> allStudents;
  final String nameQuery;
  final Set<String> selectedLocations;

  const _StudentsListContent({
    required this.students,
    required this.allStudents,
    required this.nameQuery,
    required this.selectedLocations,
  });

  @override
  Widget build(BuildContext context) {
    if (allStudents.isEmpty) {
      return const Center(child: Text('Nenhum aluno disponível.'));
    }
    if (students.isEmpty) {
      final hasFilter = nameQuery.isNotEmpty || selectedLocations.isNotEmpty;
      if (hasFilter) {
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
                      ? 'Nenhum resultado para "$nameQuery"'
                      : 'Nenhum resultado',
                  textAlign: TextAlign.center,
                  style: Theme.of(context).textTheme.titleMedium,
                ),
                if (selectedLocations.isNotEmpty) ...[
                  const SizedBox(height: 4),
                  Text(
                    'em ${selectedLocations.join(', ')}',
                    textAlign: TextAlign.center,
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ],
                const SizedBox(height: 16),
                FilledButton.tonal(
                  onPressed: () => context.read<StudentsCubitSignal>().clearFilters(),
                  child: const Text('Limpar filtros'),
                ),
              ],
            ),
          ),
        );
      }
      return const Center(child: Text('Nenhum aluno disponível.'));
    }
    return _StudentsList(students);
  }
}

final class _StudentsList extends StatelessWidget {
  final List<StudentListItem> students;

  const _StudentsList(this.students);

  @override
  Widget build(BuildContext context) {
    if (students.isEmpty) {
      return const Center(child: Text('Nenhum aluno disponível.'));
    }

    return ListView.separated(
      itemCount: students.length,
      separatorBuilder: (_, _) => const Divider(height: 1),
      itemBuilder: (context, index) {
        final student = students[index];
        final hasId = student.id.isNotEmpty;
        return ListTile(
          title: Text(student.name),
          subtitle: Text(
            '${student.position}\n${student.location}',
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
          isThreeLine: true,
          trailing: hasId ? const Icon(Icons.chevron_right) : null,
          onTap: hasId ? () => context.go('/students/${student.id}') : null,
        );
      },
    );
  }
}
