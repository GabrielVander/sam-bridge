import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/window/window_controls.dart';

final class WindowTitleBar extends StatelessWidget
    implements PreferredSizeWidget {
  final String title;
  final WindowControls? controls;
  final List<Widget> actions;

  const WindowTitleBar({
    super.key,
    required this.title,
    required this.controls,
    this.actions = const [],
  });

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);

  @override
  Widget build(BuildContext context) {
    final WindowControls? controls = this.controls;

    return AppBar(
      title: IgnorePointer(child: Text(title)),
      centerTitle: true,
      flexibleSpace: controls == null ? null : _dragArea(controls),
      backgroundColor: Theme.of(context).colorScheme.surfaceContainerHighest,
      actions: [
        ...actions,
        if (controls != null) ..._buttons(context, controls),
      ],
    );
  }

  Widget _dragArea(WindowControls controls) => GestureDetector(
    behavior: HitTestBehavior.opaque,
    onPanStart: (_) => controls.startDragging(),
    onDoubleTap: controls.toggleMaximize,
  );

  List<Widget> _buttons(BuildContext context, WindowControls controls) => [
    IconButton(
      icon: const Icon(Icons.minimize),
      tooltip: context.l10n.windowMinimize,
      onPressed: controls.minimize,
    ),
    IconButton(
      icon: const Icon(Icons.crop_square),
      tooltip: context.l10n.windowMaximize,
      onPressed: controls.toggleMaximize,
    ),
    IconButton(
      icon: const Icon(Icons.close),
      tooltip: context.l10n.windowClose,
      onPressed: controls.close,
    ),
  ];
}
