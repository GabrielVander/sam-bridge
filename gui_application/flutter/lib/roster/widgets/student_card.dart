import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/roster/instrument_name.dart';
import 'package:flutter_application/roster/position_name.dart';
import 'package:flutter_application/roster/student_list_item.dart';
import 'package:go_router/go_router.dart';

final class StudentCard extends StatelessWidget {
  final StudentListItem student;

  const StudentCard(this.student, {super.key});

  bool get _canOpen => student.id.isNotEmpty;

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: InkWell(
        onTap: _canOpen
            ? () => context.go('/students/${student.id}', extra: student.name)
            : null,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          child: Row(
            children: [
              _Avatar(name: student.name),
              const SizedBox(width: 14),
              Expanded(child: _details(context)),
              if (_canOpen) ...[
                const SizedBox(width: 8),
                const Icon(Icons.chevron_right),
              ],
            ],
          ),
        ),
      ),
    );
  }

  Widget _details(BuildContext context) => Column(
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      Text(
        student.name,
        style: Theme.of(context).textTheme.titleMedium,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
      ),
      const SizedBox(height: 4),
      Text(
        context.l10n.positionName(student.position),
        style: Theme.of(context).textTheme.bodySmall,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
      ),
      if (student.instrument case final instrument?) ...[
        const SizedBox(height: 2),
        _IconLine(
          icon: Icons.music_note_outlined,
          text: context.l10n.reportedInstrumentName(instrument),
        ),
      ],
      const SizedBox(height: 2),
      _IconLine(icon: Icons.place_outlined, text: student.location),
    ],
  );
}

final class _Avatar extends StatelessWidget {
  final String name;

  const _Avatar({required this.name});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return CircleAvatar(
      radius: 22,
      backgroundColor: colors.primaryContainer,
      child: Text(
        name.isEmpty ? '?' : name.substring(0, 1).toUpperCase(),
        style: TextStyle(
          color: colors.onPrimaryContainer,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }
}

final class _IconLine extends StatelessWidget {
  final IconData icon;
  final String text;

  const _IconLine({required this.icon, required this.text});

  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.onSurfaceVariant;

    return Row(
      children: [
        ExcludeSemantics(child: Icon(icon, size: 14, color: color)),
        const SizedBox(width: 4),
        Expanded(
          child: Text(
            text,
            style: Theme.of(
              context,
            ).textTheme.bodySmall?.copyWith(color: color),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ),
      ],
    );
  }
}
