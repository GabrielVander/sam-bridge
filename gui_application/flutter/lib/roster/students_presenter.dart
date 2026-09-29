import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter_application/errors/error_report_mapper.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/roster/ports/retrieve_students_use_case.dart';
import 'package:flutter_application/roster/roster_mapper.dart';
import 'package:flutter_application/roster/student_filter.dart';
import 'package:flutter_application/rust/api/roster.dart';

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
  const StudentsLoaded({
    required this.listing,
    required this.nameQuery,
    required this.selectedLocations,
    required this.availableLocations,
    required this.isFiltering,
  });
}

sealed class RosterListing {
  const RosterListing();
}

final class NoStudents extends RosterListing {
  const NoStudents();
}

final class NoMatches extends RosterListing {
  const NoMatches();
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
  List<StudentListItem> _all = [];
  StudentFilter _filter = const StudentFilter();

  StudentsPresenter({required this._retrieveStudents})
    : super(initialState: const StudentsIdle());

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
    if (stateValue is StudentsLoaded) {
      emit(_filteredState());
    }
  }

  void clearFilters() {
    _filter = const StudentFilter();
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
  );

  RosterListing _listing() {
    if (_all.isEmpty) return const NoStudents();
    final matching = _filter.apply(_all);
    if (matching.isEmpty) return const NoMatches();
    return Matches(matching);
  }
}
