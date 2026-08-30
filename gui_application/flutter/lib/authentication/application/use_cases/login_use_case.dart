import 'package:flutter_application/rust/bootstrap/infra/application.dart';

typedef LoginUseCase =
    Future<LoginResult> Function({
      required String email,
      required String password,
    });
