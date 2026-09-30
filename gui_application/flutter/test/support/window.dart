import 'package:flutter_application/window/window_controls.dart';

final class FakeWindow {
  final List<String> requests = [];

  late final WindowControls controls = WindowControls(
    minimize: () async => requests.add('minimize'),
    toggleMaximize: () async => requests.add('toggle maximize'),
    close: () async => requests.add('close'),
    startDragging: () async => requests.add('start dragging'),
  );
}
