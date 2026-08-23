import 'dart:async';

import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/slices/authentication/auth_presenter.dart';
import 'package:flutter_application/slices/authentication/login_screen.dart';
import 'package:flutter_application/slices/lessons/student_screen.dart';
import 'package:flutter_application/slices/roster/students_screen.dart';
import 'package:go_router/go_router.dart';

/// Bridges a presenter's state stream into a [Listenable] so [GoRouter] can
/// re-evaluate redirects on authentication changes.
class PresenterRefreshListenable extends ChangeNotifier {
  StreamSubscription<dynamic>? _subscription;

  PresenterRefreshListenable(Stream<dynamic> stream) {
    _subscription = stream.listen((_) => notifyListeners());
  }

  @override
  void dispose() {
    unawaited(_subscription?.cancel());
    super.dispose();
  }
}

GoRouter buildRouter({
  required String appVersion,
  required AuthCubitSignal authPresenter,
}) {
  return GoRouter(
    navigatorKey: GlobalKey<NavigatorState>(),
    initialLocation: '/login',
    refreshListenable: PresenterRefreshListenable(authPresenter.stream),
    redirect: (context, state) {
      final loggedIn = authPresenter.isAuthenticated;
      final loggingIn = state.matchedLocation == '/login';

      if (!loggedIn && !loggingIn) return '/login';
      if (loggedIn && loggingIn) return '/students';
      return null;
    },
    routes: [
      ShellRoute(
        builder: (context, state, child) =>
            MainScreen(versionDisplay: appVersion, child: child),
        routes: [
          GoRoute(path: '/login', builder: (_, _) => const LoginScreen()),
          GoRoute(
            path: '/students',
            builder: (_, _) => const StudentsScreen(),
            routes: [
              GoRoute(
                path: ':studentId',
                builder: (_, state) => StudentScreen(
                  studentId: state.pathParameters['studentId']!,
                ),
              ),
            ],
          ),
        ],
      ),
    ],
  );
}

/// App shell with the version footer, wrapping every route.
class MainScreen extends StatelessWidget {
  final String versionDisplay;
  final Widget child;

  const MainScreen({
    super.key,
    required this.versionDisplay,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SamSite Portal'),
        centerTitle: true,
        backgroundColor: Theme.of(context).colorScheme.surfaceContainerHighest,
      ),
      bottomNavigationBar: Container(
        width: double.infinity,
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        child: Text(
          versionDisplay,
          textAlign: TextAlign.center,
          style: const TextStyle(color: Colors.white70),
        ),
      ),
      body: child,
    );
  }
}
