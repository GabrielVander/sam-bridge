import 'package:flutter_application/rust/api/authentication.dart';

typedef LoginUseCase =
    Future<LoginOutcomeDto> Function({
      required String email,
      required String password,
    });
