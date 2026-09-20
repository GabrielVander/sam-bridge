import 'package:flutter/material.dart';
import 'package:flutter_application/lessons/widgets/back_bar.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';

Future<void> pumpBackBar(WidgetTester tester, {String? studentName}) {
  final router = GoRouter(
    initialLocation: '/student',
    routes: [
      GoRoute(
        path: '/students',
        builder: (_, _) => const Scaffold(body: Text('lista de alunos aberta')),
      ),
      GoRoute(
        path: '/student',
        builder: (_, _) => Scaffold(body: BackBar(studentName: studentName)),
      ),
    ],
  );
  return tester.pumpWidget(MaterialApp.router(routerConfig: router));
}

void main() {
  group('BackBar', () {
    testWidgets('shows the student name after the list link', (tester) async {
      await pumpBackBar(tester, studentName: 'Jane Doe');

      expect(find.text('Lista de alunos'), findsOneWidget);
      expect(find.text('Jane Doe'), findsOneWidget);
    });

    testWidgets('shows no name when there is none', (tester) async {
      await pumpBackBar(tester);

      expect(find.text('Lista de alunos'), findsOneWidget);
      expect(find.byIcon(Icons.chevron_right), findsNothing);
    });

    testWidgets('shows no name when it is empty', (tester) async {
      await pumpBackBar(tester, studentName: '');

      expect(find.byIcon(Icons.chevron_right), findsNothing);
    });

    testWidgets('going back returns to the list of students', (tester) async {
      await pumpBackBar(tester, studentName: 'Jane Doe');

      await tester.tap(find.byTooltip('Voltar'));
      await tester.pumpAndSettle();

      expect(find.text('lista de alunos aberta'), findsOneWidget);
    });
  });
}
