import 'dart:async';

import 'package:bloc_signals/bloc_signals.dart';

const Duration _slowConnectionThreshold = Duration(seconds: 10);

sealed class LoadingState {
  const LoadingState();
}

final class Waiting extends LoadingState {
  const Waiting();
}

final class WaitingLong extends LoadingState {
  const WaitingLong();
}

class LoadingPresenter extends CubitSignal<LoadingState> {
  late final Timer _slowConnectionTimer;

  LoadingPresenter() : super(initialState: const Waiting()) {
    _slowConnectionTimer = Timer(
      _slowConnectionThreshold,
      () => emit(const WaitingLong()),
    );
  }

  @override
  Future<void> close() {
    _slowConnectionTimer.cancel();
    return super.close();
  }
}
