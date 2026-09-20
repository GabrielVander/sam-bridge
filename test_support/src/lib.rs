//! Test doubles and contract suites shared by the workspace's tests.
//!
//! Fakes here behave like the real thing (an in-memory store really remembers
//! what it is given), so tests can assert on outcomes instead of inspecting
//! which methods were called. Contract suites keep each fake honest by running
//! the same checks against it and against the real adapter.

pub mod credential_store_contract;
pub mod credentials;
pub mod sam_site;
pub mod students;
