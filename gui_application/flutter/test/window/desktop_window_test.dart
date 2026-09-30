import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter_application/window/desktop_window.dart' as desktop;
import 'package:flutter_application/window/desktop_window_web.dart' as web;
import 'package:flutter_application/window/window_controls.dart';
import 'package:flutter_test/flutter_test.dart';

final class FakePlatformWindow {
  final List<MethodCall> calls = [];
  bool maximized = false;

  void install() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(const MethodChannel('window_manager'), (
          call,
        ) async {
          calls.add(call);
          return call.method == 'isMaximized' ? maximized : null;
        });
    addTearDown(
      () => TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('window_manager'),
            null,
          ),
    );
  }

  List<String> get methods => calls.map((call) => call.method).toList();
}

Future<WindowControls?> frameOn(TargetPlatform platform) async {
  debugDefaultTargetPlatformOverride = platform;
  try {
    return await desktop.frameDesktopWindow();
  } finally {
    debugDefaultTargetPlatformOverride = null;
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  for (final platform in [
    TargetPlatform.linux,
    TargetPlatform.macOS,
    TargetPlatform.windows,
  ]) {
    test('hides the ${platform.name} title bar and its buttons', () async {
      final window = FakePlatformWindow()..install();

      final controls = await frameOn(platform);

      expect(controls, isNotNull);
      expect(window.calls.last.arguments, {
        'titleBarStyle': 'hidden',
        'windowButtonVisibility': false,
      });
      expect(window.methods, ['ensureInitialized', 'setTitleBarStyle']);
    });
  }

  for (final platform in [TargetPlatform.android, TargetPlatform.iOS]) {
    test('leaves the ${platform.name} window to the platform', () async {
      final window = FakePlatformWindow()..install();

      expect(await frameOn(platform), isNull);
      expect(window.calls, isEmpty);
    });
  }

  test('leaves the browser window to the browser', () async {
    expect(await web.frameDesktopWindow(), isNull);
  });

  test('minimises, closes and drags the platform window', () async {
    final window = FakePlatformWindow()..install();
    final controls = (await frameOn(TargetPlatform.linux))!;
    window.calls.clear();

    await controls.minimize();
    await controls.close();
    await controls.startDragging();

    expect(window.methods, ['minimize', 'close', 'startDragging']);
  });

  test('maximises a restored window and restores a maximised one', () async {
    final window = FakePlatformWindow()..install();
    final controls = (await frameOn(TargetPlatform.linux))!;
    window.calls.clear();

    await controls.toggleMaximize();
    window.maximized = true;
    await controls.toggleMaximize();

    expect(window.methods, [
      'isMaximized',
      'maximize',
      'isMaximized',
      'unmaximize',
    ]);
  });
}
