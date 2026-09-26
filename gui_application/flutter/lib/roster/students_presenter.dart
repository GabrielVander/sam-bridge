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
    _filter.apply(_all),
    allStudents: _all,
    nameQuery: _filter.nameQuery,
    selectedLocations: _filter.locations,
    availableLocations: StudentFilter.locationsOf(_all),
  );
}
