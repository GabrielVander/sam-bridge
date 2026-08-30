import 'package:bloc_signals_flutter/bloc_signals_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/authentication/auth_presenter.dart';
import 'package:go_router/go_router.dart';

class LoginScreen extends StatelessWidget {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSignalListener<AuthPresenter, AuthState>(
      listener: (BuildContext context, AuthState state) {
        if (state is AuthSuccess) {
          context.go('/students');
        }
      },
      child: BlocSignalBuilder<AuthPresenter, AuthState>(
        builder: (BuildContext context, AuthState state) => switch (state) {
          AuthLoading() => const Center(child: CircularProgressIndicator()),
          AuthSuccess() => const Center(child: Icon(Icons.check_rounded)),
          AuthFailure(:final String message) => _LoginFormCard(
            errorMessage: message,
          ),
          _ => const _LoginFormCard(),
        },
      ),
    );
  }
}

final class _LoginFormCard extends StatefulWidget {
  final String? errorMessage;

  const _LoginFormCard({this.errorMessage});

  @override
  State<_LoginFormCard> createState() => _LoginFormCardState();
}

final class _LoginFormCardState extends State<_LoginFormCard> {
  final TextEditingController _username = TextEditingController();
  final TextEditingController _password = TextEditingController();

  @override
  void dispose() {
    _username.dispose();
    _password.dispose();
    super.dispose();
  }

  void _submit(BuildContext context) {
    FocusScope.of(context).unfocus();
    context.read<AuthPresenter>().submitLogin(_username.text, _password.text);
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
                    'Login to SamSite',
                    style: Theme.of(context).textTheme.headlineSmall,
                  ),
                  if (widget.errorMessage != null) ...[
                    const SizedBox(height: 12),
                    Text(
                      widget.errorMessage!,
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.error,
                      ),
                      textAlign: TextAlign.center,
                    ),
                  ],
                  const SizedBox(height: 24),
                  TextField(
                    controller: _username,
                    decoration: const InputDecoration(
                      labelText: 'Username',
                      prefixIcon: Icon(Icons.person),
                      border: OutlineInputBorder(),
                    ),
                    textInputAction: TextInputAction.next,
                  ),
                  const SizedBox(height: 16),
                  TextField(
                    controller: _password,
                    obscureText: true,
                    decoration: const InputDecoration(
                      labelText: 'Password',
                      prefixIcon: Icon(Icons.key),
                      border: OutlineInputBorder(),
                    ),
                    textInputAction: TextInputAction.done,
                    onSubmitted: (_) => _submit(context),
                  ),
                  const SizedBox(height: 32),
                  SizedBox(
                    width: double.infinity,
                    height: 48,
                    child: FilledButton(
                      onPressed: () => _submit(context),
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
