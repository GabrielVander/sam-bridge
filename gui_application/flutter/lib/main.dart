import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/router.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:flutter_application/rust/api.dart';
import 'package:flutter_application/rust/bootstrap/infra/application.dart';
import 'package:flutter_application/rust/frb_generated.dart';
import 'package:package_info_plus/package_info_plus.dart';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  final PackageInfo packageInfo = await PackageInfo.fromPlatform();
  final String versionDisplay =
      'v${packageInfo.version}+${packageInfo.buildNumber}';

  final Application application = await buildMainApplication();

  final AuthPresenter authPresenter = AuthPresenter(
    loginUseCase: application.login,
  );

  runApp(
    SamSiteApp(versionDisplay: versionDisplay, authPresenter: authPresenter),
  );
}

class SamSiteApp extends StatelessWidget {
  final String versionDisplay;
  final AuthPresenter authPresenter;

  const SamSiteApp({
    super.key,
    required this.versionDisplay,
    required this.authPresenter,
  });

  @override
  Widget build(BuildContext context) {
    return MultiBlocSignalProvider(
      providers: [
        BlocSignalProvider<AuthPresenter>.value(value: authPresenter),
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
    );
  }
}
