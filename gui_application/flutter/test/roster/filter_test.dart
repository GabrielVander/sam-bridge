import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/roster/mapper.dart';
import 'package:flutter_application/roster/students_presenter.dart';
import 'package:flutter_application/roster/students_screen.dart';
import 'package:flutter_test/flutter_test.dart';

import '../support/fake_sam_portal.dart';

void main() {
  test('RosterMapper identity', () {
    final dto = studentItem(id: "1", name: "A", location: "L");
    expect(RosterMapper.toViewModel(dto).id, "1");
    expect(RosterMapper.toViewModels([dto]).length, 1);
  });

  testWidgets('StudentsScreen search filters', (tester) async {
    final portal = FakeSamPortal()
      ..students = [
        studentItem(id: "1", name: "JOAO SILVA", location: "CENTRO"),
        studentItem(id: "2", name: "MARIA OLIVEIRA", location: "JARDIM PAULISTANO"),
        studentItem(id: "3", name: "JOÃO ÁLVARES", location: "JARDIM PAULISTANO"),
      ];
    final presenter = StudentsCubitSignal(portal);
    await tester.pumpWidget(
      MaterialApp(
        home: BlocSignalProvider<StudentsCubitSignal>.value(
          value: presenter,
          child: const Scaffold(body: StudentsScreen()),
        ),
      ),
    );
    await tester.pumpAndSettle();
    // Initially all shown
    expect(find.text("JOAO SILVA"), findsOneWidget);
    expect(find.text("MARIA OLIVEIRA"), findsOneWidget);
    // Search for joao (diacritic insensitive)
    await tester.enterText(find.byType(TextField), "joao");
    await tester.pump(const Duration(milliseconds: 300));
    await tester.pumpAndSettle();
    // Should filter to JOAO and JOÃO (fuzzy)
    expect(find.text("JOAO SILVA"), findsOneWidget);
    // Clear
    await tester.tap(find.byIcon(Icons.clear));
    await tester.pumpAndSettle();
    expect(find.text("MARIA OLIVEIRA"), findsOneWidget);
  });

  testWidgets('StudentsScreen location filter', (tester) async {
    final portal = FakeSamPortal()
      ..students = [
        studentItem(id: "1", name: "A", location: "CENTRO"),
        studentItem(id: "2", name: "B", location: "JARDIM PAULISTANO"),
      ];
    final presenter = StudentsCubitSignal(portal);
    await tester.pumpWidget(
      MaterialApp(
        home: BlocSignalProvider<StudentsCubitSignal>.value(
          value: presenter,
          child: const Scaffold(body: StudentsScreen()),
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text("Filtrar por local"));
    await tester.pumpAndSettle();
    await tester.tap(find.byType(Checkbox).first);
    await tester.pumpAndSettle();
    await tester.tap(find.text("Aplicar"));
    await tester.pumpAndSettle();
    // After filtering by CENTRO, only A should show
    expect(find.text("A"), findsOneWidget);
    expect(find.text("B"), findsNothing);
    // Clear via Limpar button
    await tester.tap(find.text("Limpar").last);
    await tester.pumpAndSettle();
    expect(find.text("B"), findsOneWidget);
  });

  testWidgets('StudentsScreen empty filtered', (tester) async {
    final portal = FakeSamPortal()..students = [studentItem(name: "A", location: "CENTRO")];
    final presenter = StudentsCubitSignal(portal);
    await tester.pumpWidget(
      MaterialApp(
        home: BlocSignalProvider<StudentsCubitSignal>.value(
          value: presenter,
          child: const Scaffold(body: StudentsScreen()),
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), "ZZZNOTFOUND");
    await tester.pump(const Duration(milliseconds: 300));
    await tester.pumpAndSettle();
    expect(find.textContaining('Nenhum resultado'), findsOneWidget);
    expect(find.text("Limpar filtros"), findsOneWidget);
    await tester.tap(find.text("Limpar filtros"));
    await tester.pumpAndSettle();
    expect(find.text("A"), findsOneWidget);
  });
}
