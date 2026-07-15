# SAM Bridge

A cross-platform mobile application built with Flutter and Rust, designed
to interface with the SAM (Sistema de Administração Musical) portal. By
leveraging the Flutter Rust Bridge (FRB), the application offloads HTML
parsing, stateful HTTP sessions, and core business logic to Rust code.

This project adheres to Clean Architecture principles and Vertical
Slices, ensuring clear separation of concerns across Bounded Contexts.

## Architecture

The codebase is structurally divided into three primary components, enforcing the
Dependency Rule where dependencies only point inward toward the core business logic:

- flutter_application (Presentation Layer): Built with Flutter and BLoC. It acts
as the UI and the Composition Root, utilizing a Rust-backed Facade to orchestrate
Dependency Injection and execute cross-boundary FFI calls.
- sam_integration (Infrastructure Layer): Handles all external I/O. It manages
stateful reqwest HTTP clients, parses complex HTML structures using scraper, maps
JSON responses to domain entities, and implements the Gateway traits defined in
the core.
- student_management (Core Layer): The heart of the application, containing pure
business logic. Organized into Vertical Slices by feature (Authentication,
Student Lessons, Student Roster), it houses the Application Use Cases and Domain
Entities with zero external dependencies.

```plantuml
@startuml
title SAM Bridge Component Diagram

component "flutter_application" {
  package "flutter_application/infra" {
    rectangle "Dart UI" as DartUI
  }

  package "flutter_application/adapters" {
    rectangle "Controller (BLoC)" as controller_bloc
    rectangle "Facade" as sam_site_facade
  }
}

component "sam_integration" {
  package "sam_integration/infra" {
    rectangle "SamClient" as sam_client
  }
}

component "student_management" {
  package "student_management/application" {
    rectangle "Use Cases" as use_cases
    interface "Gateways (Traits)" as gateways
  }

  package "student_management/domain" {
    rectangle "Domain Entities" as entities
  }
}

DartUI -> controller_bloc
controller_bloc ..> sam_site_facade : FRB FFI Calls

' Facade acts as Composition Root
sam_site_facade ..> use_cases : executes
sam_site_facade ..> sam_client : instantiates

' Core logic dependencies
use_cases ..> gateways : depends on
use_cases ..> entities : orchestrates

' Dependency Inversion: Infrastructure implements Core interfaces
sam_client ..|> gateways : implements
@enduml
```
