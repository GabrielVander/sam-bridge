import 'package:flutter_application/errors/error_reason.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/value_equality.dart';

void main() {
  group('ErrorReport', () {
    expectValueEquality<ErrorReport>(
      build: () =>
          const ErrorReport(reason: ErrorReason.network, details: 'detalhe'),
      variants: {
        'reason': () =>
            const ErrorReport(reason: ErrorReason.generic, details: 'detalhe'),
        'details': () =>
            const ErrorReport(reason: ErrorReason.network, details: 'outro'),
      },
    );
  });
}
