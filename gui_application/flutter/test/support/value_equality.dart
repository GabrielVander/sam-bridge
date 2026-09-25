import 'package:flutter_test/flutter_test.dart';

/// Every model is compared by value: presenters emit states and tests (and the
/// UI's rebuild logic) rely on two equal models being interchangeable.
void expectValueEquality<T>({
  required T Function() build,
  required Map<String, T Function()> variants,
}) {
  test('is equal, with the same hash, when every field is equal', () {
    expect(build(), build());
    expect(build().hashCode, build().hashCode);
  });

  variants.forEach((field, variant) {
    test('differs when only $field differs', () {
      expect(build(), isNot(variant()));
    });
  });
}
