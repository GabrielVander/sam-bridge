final class WindowControls {
  final Future<void> Function() minimize;
  final Future<void> Function() toggleMaximize;
  final Future<void> Function() close;
  final Future<void> Function() startDragging;

  const WindowControls({
    required this.minimize,
    required this.toggleMaximize,
    required this.close,
    required this.startDragging,
  });
}
