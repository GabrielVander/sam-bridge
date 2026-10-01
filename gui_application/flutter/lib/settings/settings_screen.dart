import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Text(context.l10n.language);
  }
}
