import 'package:flutter_application/roster/instrument.dart';
import 'package:flutter_application/roster/student_position.dart';

class StudentListItem {
  final String id;
  final String name;
  final String initial;
  final String location;
  final StudentPosition position;
  final ReportedInstrument? instrument;
  final bool canOpen;

  const StudentListItem({
    required this.id,
    required this.name,
    required this.initial,
    required this.location,
    required this.position,
    this.instrument,
    required this.canOpen,
  });
}
