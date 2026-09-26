import 'package:equatable/equatable.dart';
import 'package:flutter_application/errors/error_reason.dart';

class ErrorReport extends Equatable {
  final ErrorReason reason;
  final String details;

  const ErrorReport({required this.reason, required this.details});

  @override
  List<Object?> get props => [reason, details];
}
