import 'package:flutter/material.dart';
import 'package:flutter_application/adapters/students_presenter.dart';
import 'package:flutter_application/adapters/views.dart';
import 'package:flutter_application/frb_generated.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:package_info_plus/package_info_plus.dart';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();

  PackageInfo packageInfo = await PackageInfo.fromPlatform();
  String versionDisplay = "v${packageInfo.version}+${packageInfo.buildNumber}";

  runApp(SamSiteApp(versionDisplay: versionDisplay));
}

class SamSiteApp extends StatelessWidget {
  final String versionDisplay;

  const SamSiteApp({super.key, required this.versionDisplay});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SamSite GUI',
      debugShowCheckedModeBanner: true,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.cyan,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: BlocProvider<StudentsPresenter>(
        create: (BuildContext context) => StudentsPresenter(),
        child: MainScreen(versionDisplay: versionDisplay),
      ),
    );
  }
}

class MainScreen extends StatelessWidget {
  final String versionDisplay;

  const MainScreen({super.key, required this.versionDisplay});

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
          style: TextStyle(color: Colors.white70),
        ),
      ),
      body: BlocBuilder<StudentsPresenter, StudentsViewState>(
        builder: (BuildContext context, StudentsViewState state) {
          return switch (state) {
            StudentsLoginView() => LoginForm(
              onSubmitted: (user, password) =>
                  context.read<StudentsPresenter>().submitLogin(user, password),
            ),
            StudentsViewLoading() => const LoadingView(
              message: "Retrieving data...",
            ),
            StudentsViewLoaded() => TableView(students: state.students),
            StudentsViewLessonsLoaded() => LessonsListView(
              lessons: state.lessons,
            ),
            StudentsViewError() => ErrorView(
              errorMessage: state.message,
              onBackButtonPressed: () =>
                  context.read<StudentsPresenter>().returnToLogin(),
            ),
          };
        },
      ),
    );
  }
}
