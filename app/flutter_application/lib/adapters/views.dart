import 'package:flutter/material.dart';
import 'package:flutter_application/adapters/view_models.dart';

class LoginForm extends StatefulWidget {
  final void Function(String, String) onSubmitted;

  const LoginForm({super.key, required this.onSubmitted});

  @override
  State<LoginForm> createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();

  @override
  void dispose() {
    _usernameController.dispose();
    _passwordController.dispose();
    super.dispose();
  }

  void _submitLogin(BuildContext context) {
    FocusScope.of(context).unfocus();

    final String user = _usernameController.text;
    final String pass = _passwordController.text;

    widget.onSubmitted(user, pass);
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: Card(
            elevation: 4,
            child: Padding(
              padding: const EdgeInsets.all(32.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Icon(Icons.lock_person, size: 48, color: Colors.cyan),
                  const SizedBox(height: 16),
                  Text(
                    "Login to SamSite",
                    style: Theme.of(context).textTheme.headlineSmall,
                  ),
                  const SizedBox(height: 32),
                  TextField(
                    controller: _usernameController,
                    decoration: const InputDecoration(
                      labelText: 'Username',
                      prefixIcon: Icon(Icons.person),
                      border: OutlineInputBorder(),
                    ),
                    textInputAction: TextInputAction.next,
                  ),
                  const SizedBox(height: 16),
                  TextField(
                    controller: _passwordController,
                    obscureText: true,
                    decoration: const InputDecoration(
                      labelText: 'Password',
                      prefixIcon: Icon(Icons.key),
                      border: OutlineInputBorder(),
                    ),
                    textInputAction: TextInputAction.done,
                    onSubmitted: (_) => _submitLogin(context),
                  ),
                  const SizedBox(height: 32),
                  SizedBox(
                    width: double.infinity,
                    height: 48,
                    child: FilledButton(
                      onPressed: () => _submitLogin(context),
                      child: const Text('Login'),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class LoadingView extends StatelessWidget {
  final String message;

  const LoadingView({super.key, required this.message});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const CircularProgressIndicator(),
          const SizedBox(height: 24),
          Text(message, style: Theme.of(context).textTheme.titleMedium),
        ],
      ),
    );
  }
}

class ErrorView extends StatelessWidget {
  final String errorMessage;
  final void Function() onBackButtonPressed;

  const ErrorView({
    super.key,
    required this.errorMessage,
    required this.onBackButtonPressed,
  });

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 64, color: Colors.redAccent),
            const SizedBox(height: 16),
            Text(
              "ERROR",
              style: Theme.of(
                context,
              ).textTheme.headlineMedium?.copyWith(color: Colors.redAccent),
            ),
            const SizedBox(height: 8),
            Text(
              errorMessage,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyLarge,
            ),
            const SizedBox(height: 32),
            ElevatedButton.icon(
              icon: const Icon(Icons.arrow_back),
              label: const Text("Back"),
              onPressed: onBackButtonPressed,
            ),
          ],
        ),
      ),
    );
  }
}

class TableView extends StatelessWidget {
  final List<SingleStudentViewModel> students;

  const TableView({super.key, required this.students});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: SingleChildScrollView(
            scrollDirection: Axis.vertical,
            child: SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: DataTable(
                headingRowColor: WidgetStateProperty.all(
                  Theme.of(context).colorScheme.surfaceContainerHighest,
                ),
                columns: const [
                  DataColumn(
                    label: Text(
                      'Name',
                      style: TextStyle(
                        color: Colors.cyan,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  DataColumn(
                    label: Text(
                      'Location',
                      style: TextStyle(
                        color: Colors.cyan,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  DataColumn(
                    label: Text(
                      'Position',
                      style: TextStyle(
                        color: Colors.cyan,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                ],
                rows: students.map((SingleStudentViewModel student) {
                  return DataRow(
                    cells: [
                      DataCell(Text(student.name)),
                      DataCell(Text(student.location)),
                      DataCell(Text(student.position)),
                    ],
                  );
                }).toList(),
              ),
            ),
          ),
        ),
        Container(
          width: double.infinity,
          color: Theme.of(context).colorScheme.surfaceContainerHighest,
          // padding: const EdgeInsets.all(12.0),
          child: const Text(
            " SAM Portal ",
            textAlign: TextAlign.center,
            style: TextStyle(color: Colors.white70),
          ),
        ),
      ],
    );
  }
}
