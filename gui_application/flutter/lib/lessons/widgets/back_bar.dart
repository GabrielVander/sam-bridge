import 'package:flutter/material.dart';
import 'package:flutter_application/l10n/l10n.dart';
import 'package:flutter_application/router.dart';
import 'package:go_router/go_router.dart';

final class BackBar extends StatelessWidget {
  final String? studentName;

  const BackBar({super.key, this.studentName});

  @override
  Widget build(BuildContext context) {
    final name = studentName;
    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 8, 8, 4),
      child: Row(
        children: [
          IconButton(
            tooltip: context.l10n.backToStudents,
            icon: const Icon(Icons.arrow_back),
            onPressed: () => context.go(Routes.students),
          ),
          const SizedBox(width: 4),
          Text(
            context.l10n.studentList,
            style: Theme.of(context).textTheme.titleMedium,
          ),
          if (name != null && name.isNotEmpty) ...[
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 6),
              child: Icon(
                Icons.chevron_right,
                size: 18,
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
            ),
            Expanded(
              child: Text(
                name,
                style: Theme.of(
                  context,
                ).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ),
          ],
        ],
      ),
    );
  }
}
