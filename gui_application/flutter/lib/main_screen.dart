import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/router.dart';
import 'package:flutter_application/window/window_controls.dart';
import 'package:flutter_application/window/window_title_bar.dart';
import 'package:go_router/go_router.dart';

class MainScreen extends StatelessWidget {
  final WindowControls? windowControls;
  final bool offersSettings;
  final Widget child;

  const MainScreen({
    super.key,
    required this.windowControls,
    required this.offersSettings,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: WindowTitleBar(
        title: context.l10n.appTitle,
        controls: windowControls,
        actions: [
          if (offersSettings)
            IconButton(
              icon: const Icon(Icons.settings),
              tooltip: context.l10n.settings,
              onPressed: () => context.push(Routes.settings),
            ),
        ],
      ),
      body: child,
    );
  }
}
