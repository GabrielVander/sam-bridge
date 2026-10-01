import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:go_router/go_router.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.fromLTRB(8, 8, 8, 16),
      children: [_header(context), Text(context.l10n.language)],
    );
  }

  Widget _header(BuildContext context) => Row(
    children: [
      IconButton(
        tooltip: context.l10n.back,
        icon: const Icon(Icons.arrow_back),
        onPressed: () => context.pop(),
      ),
      const SizedBox(width: 4),
      Text(
        context.l10n.settings,
        style: Theme.of(context).textTheme.titleMedium,
      ),
    ],
  );
}
