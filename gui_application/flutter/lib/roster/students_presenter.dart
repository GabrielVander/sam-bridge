import 'package:bloc_signals/bloc_signals.dart';
import 'package:diacritic/diacritic.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/presentation_models.dart';
import 'package:flutter_application/roster/application/use_cases/retrieve_students_use_case.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/rust/bootstrap/infra/roster_view.dart';
import 'package:fuzzy/fuzzy.dart';

sealed class StudentsState {
  const StudentsState();
}

final class StudentsIdle extends StudentsState {
  const StudentsIdle();
}

final class StudentsLoading extends StudentsState {
  const StudentsLoading();
}

final class StudentsLoaded extends StudentsState {
  final List<StudentListItem> students;
  final List<StudentListItem> allStudents;
  final String nameQuery;
  final Set<String> selectedLocations;
  final List<String> availableLocations;
  const StudentsLoaded(
    this.students, {
    this.allStudents = const [],
    this.nameQuery = '',
    this.selectedLocations = const {},
    this.availableLocations = const [],
  });
}

final class StudentsFailure extends StudentsState {
  final ErrorReport report;
  const StudentsFailure(this.report);
}

class StudentsPresenter extends CubitSignal<StudentsState> {
  final RetrieveStudentsUseCase _retrieveStudents;
  List<StudentListItem> _all = [];
  String _nameQuery = '';
  Set<String> _selectedLocations = {};

  StudentsPresenter({required this._retrieveStudents})
    : super(initialState: const StudentsIdle());

  Future<void> load() async {
    emit(const StudentsLoading());
    try {
      final outcome = await _retrieveStudents();
      switch (outcome) {
        case RetrieveAllAvailableStudentsOutcome_Success(:final field0):
          _all = RosterMapper.toViewModels(field0);
          emit(_filteredState());
        case RetrieveAllAvailableStudentsOutcome_Failure(:final field0):
          emit(StudentsFailure(ErrorReportMapper.toViewModel(field0)));
      }
    } catch (e) {
      emit(StudentsFailure(ErrorReportMapper.fromThrown(e)));
    }
  }

  void filter({String? nameQuery, Set<String>? selectedLocations}) {
    if (stateValue is! StudentsLoaded && stateValue is! StudentsIdle) return;
    if (nameQuery != null) _nameQuery = nameQuery;
    if (selectedLocations != null) {
      _selectedLocations = Set.from(selectedLocations);
    }
    if (stateValue is StudentsLoaded) {
      emit(_filteredState());
    }
  }

  void clearFilters() {
    _nameQuery = '';
    _selectedLocations = {};
    if (stateValue is StudentsLoaded) {
      emit(_filteredState());
    }
  }

  String _normalize(String s) => removeDiacritics(s).toLowerCase().trim();

  StudentsLoaded _filteredState() {
    final availableLocations = {
      for (final s in _all)
        if (s.location.isNotEmpty) s.location,
    }.toList()..sort();

    List<StudentListItem> filtered = _all;

    final query = _normalize(_nameQuery);
    if (query.isNotEmpty) {
      final fuse = Fuzzy<StudentListItem>(
        _all,
        options: FuzzyOptions(
          isCaseSensitive: false,
          threshold: 0.4,
          distance: 100,
          keys: [
            WeightedKey(
              name: 'name',
              getter: (s) => _normalize(s.name),
              weight: 1,
            ),
          ],
        ),
      );
      final results = fuse.search(query);
      if (results.isNotEmpty) {
        filtered = results.map((r) => r.item).toList();
      } else {
        filtered = _all.where((s) {
          final name = _normalize(s.name);
          return name.contains(query);
        }).toList();
      }
    }

    if (_selectedLocations.isNotEmpty) {
      filtered = filtered
          .where((s) => _selectedLocations.contains(s.location))
          .toList();
    }

    return StudentsLoaded(
      filtered,
      allStudents: List.unmodifiable(_all),
      nameQuery: _nameQuery,
      selectedLocations: Set.unmodifiable(_selectedLocations),
      availableLocations: availableLocations,
    );
  }
}
