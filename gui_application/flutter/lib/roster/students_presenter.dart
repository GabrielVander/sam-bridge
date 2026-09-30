import 'dart:async';

import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/roster/ports/remember_locations.dart';
import 'package:flutter_application/roster/ports/retrieve_students_use_case.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/roster/student_filter.dart';
import 'package:flutter_application/rust/api/roster.dart';

const Duration _searchDelay = Duration(milliseconds: 250);

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
  final RosterListing listing;
  final String nameQuery;
  final Set<String> selectedLocations;
  final List<String> availableLocations;
  final bool isFiltering;
  final bool canClearSearch;
  final LocationFilterSummary locationFilter;
  final bool showsSelectedLocations;
  const StudentsLoaded({
    required this.listing,
    required this.nameQuery,
    required this.selectedLocations,
    required this.availableLocations,
    required this.isFiltering,
    required this.canClearSearch,
    required this.locationFilter,
    required this.showsSelectedLocations,
  });
}

sealed class LocationFilterSummary {
  const LocationFilterSummary();
}

final class AnyLocation extends LocationFilterSummary {
  const AnyLocation();
}

final class ChosenLocations extends LocationFilterSummary {
  final int count;
  const ChosenLocations(this.count);
}

sealed class RosterListing {
  const RosterListing();
}

final class NoStudents extends RosterListing {
  const NoStudents();
}

final class NoMatches extends RosterListing {
  final String? searchedName;
  final List<String>? chosenLocations;
  const NoMatches({this.searchedName, this.chosenLocations});
}

final class Matches extends RosterListing {
  final List<StudentListItem> students;
  const Matches(this.students);
}

final class StudentsFailure extends StudentsState {
  final ErrorReport report;
  const StudentsFailure(this.report);
}

class StudentsPresenter extends CubitSignal<StudentsState> {
  final RetrieveStudentsUseCase _retrieveStudents;
  final RememberLocations _rememberLocations;
  List<StudentListItem> _all = [];
  StudentFilter _filter;
  Timer? _pendingSearch;

  StudentsPresenter({
    required this._retrieveStudents,
    Set<String> rememberedLocations = const {},
    this._rememberLocations = _rememberNothing,
  }) : _filter = StudentFilter(
         locations: Set.unmodifiable(rememberedLocations),
       ),
       super(initialState: const StudentsIdle());

  static Future<void> _rememberNothing(Set<String> _) async {}

  @override
  Future<void> close() {
    _pendingSearch?.cancel();
    return super.close();
  }

  String get nameQuery => _filter.nameQuery;

  Future<void> open() async {
    if (stateValue is StudentsIdle) await load();
  }

  Future<void> load() async {
    emit(const StudentsLoading());
    try {
      final outcome = await _retrieveStudents();
      switch (outcome) {
        case RetrieveAllAvailableStudentsOutcomeDto_Success(:final students):
          _all = List.unmodifiable(RosterMapper.toViewModels(students));
          emit(_filteredState());
        case RetrieveAllAvailableStudentsOutcomeDto_Failure(:final report):
          emit(StudentsFailure(ErrorReportMapper.toViewModel(report)));
      }
    } catch (e) {
      emit(StudentsFailure(ErrorReportMapper.fromThrown(e)));
    }
  }

  void filter({String? nameQuery, Set<String>? selectedLocations}) {
    if (stateValue is! StudentsLoaded && stateValue is! StudentsIdle) return;
    _filter = _filter.copyWith(
      nameQuery: nameQuery,
      locations: selectedLocations,
    );
    if (selectedLocations != null) _rememberLocations(_filter.locations);
    if (stateValue is StudentsLoaded) {
      emit(_filteredState());
    }
  }

  void search(String nameQuery) {
    _pendingSearch?.cancel();
    _pendingSearch = Timer(_searchDelay, () => filter(nameQuery: nameQuery));
  }

  void clearSearch() {
    _pendingSearch?.cancel();
    filter(nameQuery: '');
  }

  void removeLocation(String location) =>
      filter(selectedLocations: {..._filter.locations}..remove(location));

  void clearFilters() {
    _pendingSearch?.cancel();
    _filter = const StudentFilter();
    _rememberLocations(_filter.locations);
    if (stateValue is StudentsLoaded) {
      emit(_filteredState());
    }
  }

  StudentsLoaded _filteredState() => StudentsLoaded(
    listing: _listing(),
    nameQuery: _filter.nameQuery,
    selectedLocations: _filter.locations,
    availableLocations: StudentFilter.locationsOf(_all),
    isFiltering: !_filter.isEmpty,
    canClearSearch: _filter.nameQuery.isNotEmpty,
    locationFilter: _filter.locations.isEmpty
        ? const AnyLocation()
        : ChosenLocations(_filter.locations.length),
    showsSelectedLocations: _filter.locations.isNotEmpty,
  );

  RosterListing _listing() {
    if (_all.isEmpty) return const NoStudents();
    final matching = _filter.apply(_all);
    if (matching.isEmpty) {
      return NoMatches(
        searchedName: _filter.nameQuery.isEmpty ? null : _filter.nameQuery,
        chosenLocations: _filter.locations.isEmpty
            ? null
            : _filter.locations.toList(),
      );
    }
    return Matches(matching);
  }
}
