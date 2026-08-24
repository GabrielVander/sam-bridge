import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/api.dart' as api;
import 'package:flutter_application/frb_generated.dart';
import 'package:flutter_application/portal/sam_portal.dart';
import 'package:flutter_application/router.dart';
import 'package:flutter_application/slices/authentication/auth_presenter.dart';
import 'package:flutter_application/slices/lessons/lessons_presenter.dart';
import 'package:flutter_application/slices/roster/students_presenter.dart';
import 'package:package_info_plus/package_info_plus.dart';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  final restored = await api.tryRestoreSession();

  final packageInfo = await PackageInfo.fromPlatform();
  final versionDisplay = 'v${packageInfo.version}+${packageInfo.buildNumber}';

  final initialPortal = const RustSamPortal();
  final initialAuth = AuthCubitSignal(initialPortal);
  if (restored) initialAuth.markRestored();

  runApp(
    SamSiteApp(
      versionDisplay: versionDisplay,
      portal: const RustSamPortal(),
      authPresenter: initialAuth,
    ),
  );
}

class SamSiteApp extends StatelessWidget {
  final String versionDisplay;
  final SamPortal portal;
  final AuthCubitSignal? authPresenter;

  const SamSiteApp({
    super.key,
    required this.versionDisplay,
    required this.portal,
    this.authPresenter,
  });

  @override
  Widget build(BuildContext context) {
    final authPresenter = this.authPresenter ?? AuthCubitSignal(portal);

    return BlocSignalProvider<AuthCubitSignal>.value(
      value: authPresenter,
      child: MultiBlocSignalProvider(
        providers: [
          BlocSignalProvider<StudentsCubitSignal>(
            create: (_) => StudentsCubitSignal(portal),
          ),
          BlocSignalProvider<LessonsCubitSignal>(
            create: (_) => LessonsCubitSignal(portal),
          ),
        ],
        child: Builder(
          builder: (context) {
            return MaterialApp.router(
              title: 'SamSite GUI',
              routerConfig: buildRouter(
                appVersion: versionDisplay,
                authPresenter: authPresenter,
              ),
              debugShowCheckedModeBanner: true,
              theme: ThemeData(
                colorScheme: ColorScheme.fromSeed(
                  seedColor: Colors.cyan,
                  brightness: Brightness.dark,
                ),
                useMaterial3: true,
              ),
            );
          },
        ),
      ),
    );
  }
}
