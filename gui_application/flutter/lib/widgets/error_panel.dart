import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_application/presentation_models.dart';

/// A friendly error message with a retry action, plus the technical details
/// tucked into a collapsed expander so regular users are not overwhelmed but
/// anyone can copy them into a bug report.
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
              Text(report.userMessage, textAlign: TextAlign.center),
              const SizedBox(height: 16),
              FilledButton.tonal(
                onPressed: onRetry,
                child: const Text('Tentar novamente'),
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

/// Collapsed "Detalhes técnicos" expander with a copy button.
class TechnicalDetails extends StatelessWidget {
  final String details;

  const TechnicalDetails({super.key, required this.details});

  @override
  Widget build(BuildContext context) {
    return ExpansionTile(
      title: Text(
        'Detalhes técnicos',
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
              tooltip: 'Copiar detalhes',
              onPressed: () => _copy(context),
            ),
          ],
        ),
      ],
    );
  }

  Future<void> _copy(BuildContext context) async {
    final messenger = ScaffoldMessenger.of(context);
    await Clipboard.setData(ClipboardData(text: details));
    messenger.showSnackBar(const SnackBar(content: Text('Detalhes copiados')));
  }
}
