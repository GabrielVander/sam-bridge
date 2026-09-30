import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/widgets/loading_presenter.dart';

class LoadingIndicator extends StatefulWidget {
  const LoadingIndicator({super.key});

  @override
  State<LoadingIndicator> createState() => _LoadingIndicatorState();
}

class _LoadingIndicatorState extends State<LoadingIndicator> {
  final LoadingPresenter _presenter = LoadingPresenter();

  @override
  void dispose() {
    _presenter.close();
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
            BlocSignalBuilder<LoadingPresenter, LoadingState>(
              bloc: _presenter,
              builder: (context, state) => switch (state) {
                Waiting() => const SizedBox.shrink(),
                WaitingLong() => Padding(
                  padding: const EdgeInsets.only(top: 16),
                  child: Text(
                    context.l10n.slowConnection,
                    textAlign: TextAlign.center,
                  ),
                ),
              },
            ),
          ],
        ),
      ),
    );
  }
}
