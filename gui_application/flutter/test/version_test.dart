import 'package:flutter_application/version.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('formatVersion joins the version and build number', () {
    expect(formatVersion(version: '1.2.3', buildNumber: '45'), 'v1.2.3+45');
  });
}
