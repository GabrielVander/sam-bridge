# SAM Bridge

A cross-platform application built with Flutter and Rust that bridges the SAM
(Sistema de Administração Musical) portal. Flutter Rust Bridge (FRB) offloads
stateful HTTP sessions, HTML parsing, and business orchestration to Rust,
while the Flutter side stays a thin presentation layer.

## Architecture

The project follows Test-Driven Development and applies concepts of Clean
Architecture (organized as Vertical Slices) and Domain-Driven Design.

Dependencies point inward: presentation → application → domain.
Infrastructure implements core-defined gateway ports; the core never knows
about HTTP or HTML.

```plantuml
@startuml
title SAM Bridge Component Diagram

component "gui_application::flutter" as ui <<Main + Infrastructure>>
component gui_application <<Adapter>>
component authentication <<Domain + Application>>
component student <<Domain + Application>>
component sam <<Adapter + Infrastructure>>
component credential_store <<Adapter + Infrastructure>>

ui --> gui_application

gui_application --> authentication
gui_application --> student
gui_application --> sam
gui_application --> credential_store

sam --> authentication
sam --> student

credential_store --> authentication
@enduml
```

`ui` is the true composition root (`main.dart` calls `RustLib.init()`, builds the
facade, and wires the widget tree), so it owns `<<Main>>`; `gui_application`
exposes an `ApplicationFacade` that translates use cases into FRB-friendly
DTOs, making it an `<<Adapter>>` for the delivery mechanism rather than the
outermost layer. `sam` and `credential_store` implement gateway/store ports
declared by `authentication` and `student` (dependency inversion), which is
why the arrows point from adapter to domain and never the reverse.

### Authentication flows

Each port appears in the lane of the crate that implements it; the trait itself
is declared by `authentication`. Errors travel as `{ kind, details }` up to
`ApplicationFacade`, which turns them into an `ErrorReportDto`.

Logging in authorizes the credential against SAM and only then remembers it.
Failing to remember it still logs the user in.

```plantuml
@startuml
title SAM Bridge Login Sequence

actor User
box "gui_application::flutter"
  participant LoginScreen <<Widget>>
  participant AuthPresenter <<Presenter>>
end box
box "gui_application"
  participant ApplicationFacade <<Adapter>>
end box
box "authentication"
  participant LoginAndRememberCredentialsUseCase as login <<Use Case>>
end box
box "sam"
  participant AuthorizeCredentialGateway as authorizer <<Port>>
  participant SamClient
end box
box "credential_store"
  participant SaveCredentialGateway as storage <<Port>>
end box
participant "SAM portal" as portal

User -> LoginScreen : submit email and password
LoginScreen -> AuthPresenter : submitLogin(email, password)

alt email or password empty
  AuthPresenter --> LoginScreen : emit AuthMissingFields
else
  AuthPresenter --> LoginScreen : emit AuthLoading
  AuthPresenter -> ApplicationFacade : login(email, password)\nawaited across FRB
  ApplicationFacade -> login : execute(email, password)
  login -> authorizer : authorize(credential)
  authorizer -> SamClient : login(credentials)
  note right of SamClient
    Discards cached reads:
    the session may now
    belong to another user
  end note
  SamClient -> portal : POST /autenticar\n(cookie store keeps the session)
  portal --> SamClient : response

  alt 303 See Other
    SamClient --> authorizer : Ok
    authorizer --> login : Authorized
    login -> storage : save(credential)
    note right of storage : Encrypted file (cocoon)

    alt saved
      storage --> login : Ok
      login --> ApplicationFacade : LoggedIn
      ApplicationFacade --> AuthPresenter : Successful
      AuthPresenter --> LoginScreen : emit AuthSuccess
    else storage failed
      storage --> login : SaveCredentialGatewayError
      login --> ApplicationFacade : LoggedInWithoutRemembering
      ApplicationFacade --> AuthPresenter : SuccessfulWithoutRemembering { report }
      AuthPresenter --> LoginScreen : emit AuthSuccess(notRemembered)
    end

  else 200 with the invalid credentials marker
    SamClient --> authorizer : InvalidCredentials
    authorizer --> login : Unauthorized
    login --> ApplicationFacade : InvalidEmailOrPassword
    ApplicationFacade --> AuthPresenter : InvalidEmailOrPassword
    AuthPresenter --> LoginScreen : emit AuthUnauthorized

  else network failure or unexpected response
    SamClient --> authorizer : SamClientError
    authorizer --> login : AuthorizationError { kind, details }
    login --> ApplicationFacade : UnableToPerformAuthorization
    ApplicationFacade --> AuthPresenter : Failure { report }
    AuthPresenter --> LoginScreen : emit AuthFailure(report)
  end
end
@enduml
```

At startup, `main.dart` restores the session before running the app. Credentials
that SAM rejects are cleared, and no failure here keeps the user from reaching
the login form.

```plantuml
@startuml
title SAM Bridge Session Restore Sequence

box "gui_application::flutter"
  participant main <<Main>>
  participant AuthPresenter <<Presenter>>
end box
box "gui_application"
  participant ApplicationFacade <<Adapter>>
end box
box "authentication"
  participant RestoreSessionUseCase as restore <<Use Case>>
end box
box "credential_store"
  participant LoadCredentialGateway as loader <<Port>>
  participant ClearCredentialGateway as clearer <<Port>>
end box
box "sam"
  participant AuthorizeCredentialGateway as authorizer <<Port>>
end box

main -> AuthPresenter : restoreSession()
AuthPresenter -> ApplicationFacade : restoreSession()\nawaited across FRB
ApplicationFacade -> restore : execute()
restore -> loader : load()

alt nothing stored, or undecryptable
  loader --> restore : None
  restore --> ApplicationFacade : NoStoredCredentials
  ApplicationFacade --> AuthPresenter : NotAvailable
  AuthPresenter -> AuthPresenter : emit AuthIdle
else credential loaded
  loader --> restore : credential
  restore -> authorizer : authorize(credential)
  note right of authorizer : Same SAM login as\nthe Login Sequence

  alt Authorized
    authorizer --> restore : Authorized
    restore --> ApplicationFacade : Restored
    ApplicationFacade --> AuthPresenter : Restored
    AuthPresenter -> AuthPresenter : emit AuthSuccess
  else Unauthorized
    authorizer --> restore : Unauthorized
    restore -> clearer : clear()
    clearer --> restore : Ok, or ClearCredentialGatewayError
    restore --> ApplicationFacade : CredentialsRejected, or\nUnableToClearRejectedCredentials
    ApplicationFacade --> AuthPresenter : NotAvailable
    AuthPresenter -> AuthPresenter : emit AuthIdle
  else unable to authorize
    authorizer --> restore : AuthorizationError { kind, details }
    restore --> ApplicationFacade : UnableToPerformOperation
    ApplicationFacade --> AuthPresenter : Failure { report }
    AuthPresenter -> AuthPresenter : emit AuthIdle
    note right of AuthPresenter : A failed restore never\nblocks startup
  end
end

AuthPresenter --> main
main -> main : runApp
@enduml
```

## Development

Common commands (from the repository root unless noted):

```sh
# Rust
cargo test --workspace
cargo clippy --workspace --all-targets
cargo llvm-cov nextest --workspace --summary-only --ignore-filename-regex 'frb_generated\.rs|/tests/support/'

# Flutter (inside gui_application/flutter)
flutter pub get
flutter analyze
flutter test
flutter run linux
flutter build linux --debug     # rm -rf build/linux if CMake cache goes stale

# Both stacks (repository root): combined line-coverage percentage
scripts/coverage.sh

# Regenerate FRB bindings after changing anything under gui_application/src/api/
cd gui_application/flutter && flutter_rust_bridge_codegen generate
```

## Localization

The UI speaks Brazilian Portuguese and English, chosen from the OS language:
any Portuguese locale gets Portuguese, everything else gets English.
Translations live in `gui_application/flutter/lib/l10n/app_<language>.arb`,
with `app_pt.arb` as the template. Mappers hand widgets enums and dates, and
widgets turn them into text through `context.l10n`.

To add or change a message, edit every ARB file (a test checks they share the
same keys) and regenerate the committed bindings:

```sh
cd gui_application/flutter && flutter gen-l10n
```

To add a language, add `app_<language>.arb`. When the OS language is not
supported, Flutter falls back to the first entry of the generated
`supportedLocales`, which gen-l10n sorts alphabetically, so a new language
sorting before `en` must come with a `localeResolutionCallback` to keep English
as the fallback.
