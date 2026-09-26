import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_application/errors/error_message.dart';
import 'package:flutter_application/errors/error_report.dart';
import 'package:flutter_application/l10n/l10n.dart';

class ErrorPanel extends StatelessWidget {
  final ErrorReport report;
  final VoidCallback onRetry;

  const ErrorPanel({super.key, required this.report, required this.onRetry});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SingleChildScrollView(
        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 480),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                Icons.error_outline,
                size: 48,
                color: Theme.of(context).colorScheme.error,
              ),
              const SizedBox(height: 16),
              Text(
                context.l10n.errorMessage(report.reason),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 16),
              FilledButton.tonal(
                onPressed: onRetry,
                child: Text(context.l10n.retry),
              ),
              if (report.details.isNotEmpty) ...[
                const SizedBox(height: 8),
                TechnicalDetails(details: report.details),
              ],
            ],
          ),
        ),
      ),
    );
  }
}

class TechnicalDetails extends StatelessWidget {
  final String details;

  const TechnicalDetails({super.key, required this.details});

  @override
  Widget build(BuildContext context) {
    return ExpansionTile(
      title: Text(
        context.l10n.technicalDetails,
        style: Theme.of(context).textTheme.labelLarge,
      ),
      maintainState: false,
      shape: const Border(),
      collapsedShape: const Border(),
      childrenPadding: const EdgeInsets.fromLTRB(16, 0, 16, 8),
      children: [
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: SelectableText(
                details,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ),
            IconButton(
              icon: const Icon(Icons.copy),
              tooltip: context.l10n.copyDetails,
              onPressed: () => _copy(context),
            ),
          ],
        ),
      ],
    );
  }

  Future<void> _copy(BuildContext context) async {
    final ScaffoldMessengerState messenger = ScaffoldMessenger.of(context);
    final String confirmation = context.l10n.detailsCopied;

    await Clipboard.setData(ClipboardData(text: details));

    messenger.showSnackBar(SnackBar(content: Text(confirmation)));
  }
}
