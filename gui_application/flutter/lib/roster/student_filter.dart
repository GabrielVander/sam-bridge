import 'package:diacritic/diacritic.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:fuzzy/fuzzy.dart';

/// What the user is narrowing the roster by: part of a name and a set of
/// locations. An empty criterion lets everyone through.
class StudentFilter extends Equatable {
  final String nameQuery;
  final Set<String> locations;

  const StudentFilter({this.nameQuery = '', this.locations = const {}});

  bool get isEmpty => nameQuery.isEmpty && locations.isEmpty;

  StudentFilter copyWith({String? nameQuery, Set<String>? locations}) =>
      StudentFilter(
        nameQuery: nameQuery ?? this.nameQuery,
        locations: locations == null
            ? this.locations
            : Set.unmodifiable(locations),
      );

  List<StudentListItem> apply(List<StudentListItem> students) {
    final byName = _matchingName(students);
    if (locations.isEmpty) return byName;
    return byName.where((s) => locations.contains(s.location)).toList();
  }

  /// Every location worth offering as a choice: once each, alphabetically.
  static List<String> locationsOf(List<StudentListItem> students) => {
    for (final s in students)
      if (s.location.isNotEmpty) s.location,
  }.toList()..sort();

  List<StudentListItem> _matchingName(List<StudentListItem> students) {
    final query = _normalize(nameQuery);
    if (query.isEmpty) return students;

    final fuzzy = Fuzzy<StudentListItem>(
      students,
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
    final results = fuzzy.search(query);
    if (results.isNotEmpty) return results.map((r) => r.item).toList();

    // Fuzzy scoring penalises matches far from the start of the name, so a
    // surname deep into a long name needs a plain substring match instead.
    return students.where((s) => _normalize(s.name).contains(query)).toList();
  }

  static String _normalize(String s) =>
      removeDiacritics(s).toLowerCase().trim();

  @override
  List<Object?> get props => [nameQuery, locations];
}
