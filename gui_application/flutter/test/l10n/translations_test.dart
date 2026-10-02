import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

void main() {
  test('English translates every Portuguese message', () {
    expect(messagesIn('en'), messagesIn('pt'));
  });
}

Set<String> messagesIn(String language) {
  final arb = File('lib/l10n/app_$language.arb').readAsStringSync();
  final entries = jsonDecode(arb) as Map<String, dynamic>;

  return entries.keys.where((key) => !key.startsWith('@')).toSet();
}
