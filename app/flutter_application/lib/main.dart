import 'package:flutter/material.dart';
import 'package:flutter_application/adapters/students_presenter.dart';
import 'package:flutter_application/adapters/views.dart';
import 'package:flutter_application/frb_generated.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const SamSiteApp());
}

class SamSiteApp extends StatelessWidget {
  const SamSiteApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SamSite GUI',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.cyan,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: BlocProvider<StudentsPresenter>(
        create: (BuildContext context) => StudentsPresenter(),
        child: const MainScreen(),
      ),
    );
  }
}

class MainScreen extends StatelessWidget {
  const MainScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SamSite Portal'),
        centerTitle: true,
        backgroundColor: Theme.of(context).colorScheme.surfaceContainerHighest,
      ),
      body: BlocBuilder<StudentsPresenter, StudentsViewState>(
        builder: (BuildContext context, StudentsViewState state) {
          return switch (state) {
            StudentsView() => LoginForm(
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
