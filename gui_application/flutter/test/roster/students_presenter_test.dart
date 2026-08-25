import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

void main() {
  test('starts idle', () {
    final presenter = StudentsCubitSignal(FakeSamPortal());
    expect(presenter.stateValue, isA<StudentsIdle>());
  });

  test('given students should load them', () async {
    final portal = FakeSamPortal()..students = [studentItem(id: '7')];
    final presenter = StudentsCubitSignal(portal);

    await presenter.load();

    final state = presenter.stateValue;
    expect(state, isA<StudentsLoaded>());
    expect((state as StudentsLoaded).students.single.id, '7');
  });

  test('given portal failure should surface failure state', () async {
    final portal = FakeSamPortal()..studentsError = Exception('Session expired');
    final presenter = StudentsCubitSignal(portal);

    await presenter.load();

    final state = presenter.stateValue;
    expect(state, isA<StudentsFailure>());
    expect((state as StudentsFailure).message, contains('Session expired'));
  });
}
