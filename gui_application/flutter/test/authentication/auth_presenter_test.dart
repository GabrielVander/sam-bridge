import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

void main() {
  test('given empty credentials should fail without touching the portal', () async {
    final portal = FakeSamPortal();
    final presenter = AuthCubitSignal(portal);

    await presenter.submitLogin('', '');

    expect(presenter.stateValue, isA<AuthFailure>());
    expect((presenter.stateValue as AuthFailure).message, contains('Informe'));
    expect(portal.loginCalls, isEmpty);
  });

  test('given valid credentials should transition loading then success', () async {
    final portal = FakeSamPortal();
    final presenter = AuthCubitSignal(portal);

    final submitting = presenter.submitLogin('user', 'pass');
    expect(presenter.stateValue, isA<AuthLoading>());

    await submitting;
    expect(presenter.stateValue, isA<AuthSuccess>());
    expect(presenter.isAuthenticated, isTrue);
    expect(portal.loggedIn, isTrue);
    expect(portal.loginCalls.single.$1, 'user');
  });

  test('given portal rejection should surface a failure state', () async {
    final portal = FakeSamPortal()..loginError = Exception('Invalid credentials');
    final presenter = AuthCubitSignal(portal);

    await presenter.submitLogin('user', 'wrong');

    final state = presenter.stateValue;
    expect(state, isA<AuthFailure>());
    expect((state as AuthFailure).message, contains('Invalid credentials'));
    expect(presenter.isAuthenticated, isFalse);
  });

  test('reset should return to idle', () async {
    final portal = FakeSamPortal();
    final presenter = AuthCubitSignal(portal);

    await presenter.submitLogin('u', 'p');
    presenter.reset();

    expect(presenter.stateValue, isA<AuthIdle>());
  });
}
