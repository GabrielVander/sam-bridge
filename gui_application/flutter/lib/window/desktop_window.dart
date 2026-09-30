import 'package:flutter/foundation.dart';
import 'package:flutter_application/window/window_controls.dart';
import 'package:window_manager/window_manager.dart';

Future<WindowControls?> frameDesktopWindow() async {
  if (!_isDesktop(defaultTargetPlatform)) return null;

  await windowManager.ensureInitialized();
  await windowManager.setTitleBarStyle(
    TitleBarStyle.hidden,
    windowButtonVisibility: false,
  );

  return WindowControls(
    minimize: windowManager.minimize,
    toggleMaximize: _toggleMaximize,
    close: windowManager.close,
    startDragging: windowManager.startDragging,
  );
}

bool _isDesktop(TargetPlatform platform) => switch (platform) {
  TargetPlatform.linux ||
  TargetPlatform.macOS ||
  TargetPlatform.windows => true,
  TargetPlatform.android ||
  TargetPlatform.iOS ||
  TargetPlatform.fuchsia => false,
};

Future<void> _toggleMaximize() async {
  if (await windowManager.isMaximized()) {
    await windowManager.unmaximize();
  } else {
    await windowManager.maximize();
  }
}
