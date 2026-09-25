import 'package:flutter_application/rust/api/authentication.dart';

typedef LoginUseCase =
    Future<LoginOutcome> Function({
      required String email,
      required String password,
    });
