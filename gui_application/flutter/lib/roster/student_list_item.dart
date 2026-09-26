import 'package:equatable/equatable.dart';
import 'package:flutter_application/roster/student_position.dart';

class StudentListItem extends Equatable {
  final String id;
  final String name;
  final String location;
  final StudentPosition position;
  final String? instrument;

  const StudentListItem({
    required this.id,
    required this.name,
    required this.location,
    required this.position,
    this.instrument,
  });

  @override
  List<Object?> get props => [id, name, location, position, instrument];
}
