import 'package:equatable/equatable.dart';

class ErrorReport extends Equatable {
  final String userMessage;
  final String details;

  const ErrorReport({required this.userMessage, required this.details});

  @override
  List<Object?> get props => [userMessage, details];
}
