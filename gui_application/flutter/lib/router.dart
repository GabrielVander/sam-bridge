import 'dart:async';

import 'package:bloc_signals/bloc_signals.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/authentication/login_screen.dart';
import 'package:flutter_application/lessons/student_screen.dart';
import 'package:flutter_application/roster/students_screen.dart';
import 'package:go_router/go_router.dart';

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
  required AuthPresenter authPresenter,
}) {
  return GoRouter(
    navigatorKey: GlobalKey<NavigatorState>(),
    initialLocation: authPresenter.isAuthenticated ? '/students' : '/login',
    refreshListenable: PresenterRefreshListenable(authPresenter.stream),
    redirect: (BuildContext context, GoRouterState state) {
      final bool loggedIn = authPresenter.isAuthenticated;
      final bool loggingIn = state.matchedLocation == '/login';

      if (!loggedIn && !loggingIn) return '/login';
      if (loggedIn && loggingIn) return '/students';
      return null;
    },
    routes: [
      ShellRoute(
        builder: (BuildContext context, GoRouterState state, Widget child) =>
            MainScreen(versionDisplay: appVersion, child: child),
        routes: [
          GoRoute(
            path: '/login',
            builder: (BuildContext _, GoRouterState _) => const LoginScreen(),
          ),
          GoRoute(
            path: '/students',
            builder: (BuildContext _, GoRouterState _) =>
                const StudentsScreen(),
            routes: [
              GoRoute(
                path: ':studentId',
                builder: (BuildContext _, GoRouterState state) => StudentScreen(
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
