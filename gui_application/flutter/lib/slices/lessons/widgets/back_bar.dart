import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

final class BackBar extends StatelessWidget {
  const BackBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(4, 4, 4, 0),
      child: Row(
        children: [
          IconButton(
            tooltip: 'Voltar',
            icon: const Icon(Icons.arrow_back),
            onPressed: () => context.go('/students'),
          ),
        ],
      ),
    );
  }
}
