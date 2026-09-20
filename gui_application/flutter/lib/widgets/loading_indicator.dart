import 'dart:async';

import 'package:flutter/material.dart';

const Duration _slowConnectionThreshold = Duration(seconds: 10);

class LoadingIndicator extends StatefulWidget {
  const LoadingIndicator({super.key});

  @override
  State<LoadingIndicator> createState() => _LoadingIndicatorState();
}

class _LoadingIndicatorState extends State<LoadingIndicator> {
  late final Timer _slowConnectionTimer;
  bool _slow = false;

  @override
  void initState() {
    super.initState();
    _slowConnectionTimer = Timer(
      _slowConnectionThreshold,
      () => setState(() => _slow = true),
    );
  }

  @override
  void dispose() {
    _slowConnectionTimer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const CircularProgressIndicator(),
            if (_slow) ...[
              const SizedBox(height: 16),
              const Text(
                'O SAM está demorando para responder. '
                'Aguarde mais um pouco…',
                textAlign: TextAlign.center,
              ),
            ],
          ],
        ),
      ),
    );
  }
}
