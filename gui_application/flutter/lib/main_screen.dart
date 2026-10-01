import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
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
          BlocSignalBuilder<AuthPresenter, AuthState>(
            builder: (context, state) => state.isSignedIn
                ? IconButton(
                    icon: const Icon(Icons.logout),
                    tooltip: context.l10n.logOut,
                    onPressed: () => context.read<AuthPresenter>().signOut(),
                  )
                : const SizedBox.shrink(),
          ),
        ],
      ),
      body: child,
    );
  }
}
