import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/roster/students_screen.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

Widget harness(FakeSamPortal portal) {
  final presenter = StudentsCubitSignal(portal);
  return BlocSignalProvider<StudentsCubitSignal>.value(
    value: presenter,
    child: MaterialApp(home: Scaffold(body: const StudentsScreen())),
  );
}

void main() {
  testWidgets('loads and renders the student roster', (tester) async {
    final portal = FakeSamPortal()
      ..students = [
        studentItem(id: '1', name: 'ALUNA UM'),
        studentItem(id: '2', name: 'ALUNO DOIS', levelName: 'HalfHour', kind: DtoPositionKind.organist),
      ];
    await tester.pumpWidget(harness(portal));
    await tester.pumpAndSettle();

    expect(find.text('ALUNA UM'), findsOneWidget);
    expect(find.text('ALUNO DOIS'), findsOneWidget);
    expect(find.textContaining('Organista · Meia hora'), findsOneWidget);
  });

  testWidgets('shows the failure message when loading fails', (tester) async {
    await tester.pumpWidget(harness(FakeSamPortal()..studentsError = Exception('boom')));
    await tester.pumpAndSettle();

    expect(find.textContaining('boom'), findsOneWidget);
    expect(find.text('Tentar novamente'), findsOneWidget);
  });

  testWidgets('rows without an id are rendered but not clickable', (tester) async {
    final portal = FakeSamPortal()
      ..students = [studentItem(id: '', name: 'SEM IDENTIFICAÇÃO')];
    await tester.pumpWidget(harness(portal));
    await tester.pumpAndSettle();

    expect(find.text('SEM IDENTIFICAÇÃO'), findsOneWidget);
    expect(find.byIcon(Icons.chevron_right), findsNothing,
        reason: 'No chevron without a navigable id');
  });

  testWidgets('shows the empty message for an empty roster', (tester) async {
    await tester.pumpWidget(harness(FakeSamPortal()));
    await tester.pumpAndSettle();

    expect(find.text('Nenhum aluno disponível.'), findsOneWidget);
  });
}
