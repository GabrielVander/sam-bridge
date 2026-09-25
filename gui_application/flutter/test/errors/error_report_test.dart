import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/value_equality.dart';

void main() {
  group('ErrorReport', () {
    expectValueEquality<ErrorReport>(
      build: () => const ErrorReport(userMessage: 'Falhou', details: 'detalhe'),
      variants: {
        'userMessage': () =>
            const ErrorReport(userMessage: 'Outra', details: 'detalhe'),
        'details': () =>
            const ErrorReport(userMessage: 'Falhou', details: 'outro'),
      },
    );
  });
}
