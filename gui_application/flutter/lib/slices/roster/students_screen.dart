import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/slices/roster/students_presenter.dart';
import 'package:flutter_application/view_models.dart';
import 'package:go_router/go_router.dart';

class StudentsScreen extends StatefulWidget {
  const StudentsScreen({super.key});

  @override
  State<StudentsScreen> createState() => _StudentsScreenState();
}

final class _StudentsScreenState extends State<StudentsScreen> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final state = context.read<StudentsCubitSignal>().stateValue;
      if (state is StudentsIdle) {
        context.read<StudentsCubitSignal>().load();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return BlocSignalBuilder<StudentsCubitSignal, StudentsState>(
      builder: (context, state) => switch (state) {
        StudentsLoading() => const Center(child: CircularProgressIndicator()),
        StudentsLoaded(:final students) => _StudentsList(students),
        StudentsFailure(:final message) => Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.error_outline,
                size: 48,
                color: Theme.of(context).colorScheme.error,
              ),
              const SizedBox(height: 16),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 24),
                child: Text(message, textAlign: TextAlign.center),
              ),
              const SizedBox(height: 16),
              FilledButton.tonal(
                onPressed: () => context.read<StudentsCubitSignal>().load(),
                child: const Text('Tentar novamente'),
              ),
            ],
          ),
        ),
        _ => const SizedBox.shrink(),
      },
    );
  }
}

final class _StudentsList extends StatelessWidget {
  final List<StudentListItem> students;

  const _StudentsList(this.students);

  @override
  Widget build(BuildContext context) {
    if (students.isEmpty) {
      return const Center(child: Text('Nenhum aluno disponível.'));
    }

    return ListView.separated(
      itemCount: students.length,
      separatorBuilder: (_, _) => const Divider(height: 1),
      itemBuilder: (context, index) {
        final student = students[index];
        final hasId = student.id.isNotEmpty;
        return ListTile(
          title: Text(student.name),
          subtitle: Text(
            '${student.position}\n${student.location}',
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
          isThreeLine: true,
          trailing: hasId ? const Icon(Icons.chevron_right) : null,
          onTap: hasId ? () => context.go('/students/${student.id}') : null,
        );
      },
    );
  }
}
