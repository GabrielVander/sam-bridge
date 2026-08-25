import 'package:flutter/material.dart';

class UnknownLevelBanner extends StatelessWidget {
  final String raw;

  const UnknownLevelBanner({super.key, required this.raw});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.help_outline, size: 48, color: Colors.orange),
            const SizedBox(height: 12),
            const Text(
              'nível não reconhecido',
              style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              textAlign: TextAlign.center,
            ),
            if (raw.isNotEmpty) ...[
              const SizedBox(height: 8),
              Text(
                'Valor: $raw',
                style: Theme.of(context).textTheme.bodySmall,
                textAlign: TextAlign.center,
              ),
            ],
            const SizedBox(height: 8),
            Text(
              'O progresso não pode ser calculado para este nível.',
              style: Theme.of(context).textTheme.bodySmall,
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }
}
