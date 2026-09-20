//! What every [`CredentialStore`] must do, whatever it stores credentials in.
//!
//! Run the suite against an implementation with [`credential_store_contract!`]:
//!
//! ```ignore
//! test_support::credential_store_contract!(MyStore::new());
//! ```
//!
//! The expression is evaluated afresh for each check, so every check starts
//! from an empty store.

// Panicking when the contract is broken is the whole point of these functions.
#![allow(clippy::missing_panics_doc)]

use authentication::application::gateways::CredentialStore;
use authentication::domain::entities::{Credential, Email, Password};

fn credential(email: &str, password: &str) -> Credential {
    Credential::new(Email(email.to_owned()), Password(password.to_owned()))
}

fn loaded(store: &dyn CredentialStore) -> Option<(String, String)> {
    store
        .load()
        .map(|credential| (credential.email.0, credential.password.0))
}

pub fn a_new_store_holds_no_credential(store: &dyn CredentialStore) {
    assert_eq!(loaded(store), None);
}

pub fn a_saved_credential_is_loaded_back_unchanged(store: &dyn CredentialStore) {
    assert!(store.save(&credential("ana@example.com", "s3cret")).is_ok());

    assert_eq!(
        loaded(store),
        Some(("ana@example.com".to_owned(), "s3cret".to_owned()))
    );
}

pub fn saving_again_replaces_the_previous_credential(store: &dyn CredentialStore) {
    assert!(store.save(&credential("ana@example.com", "first")).is_ok());
    assert!(
        store
            .save(&credential("bruno@example.com", "second"))
            .is_ok()
    );

    assert_eq!(
        loaded(store),
        Some(("bruno@example.com".to_owned(), "second".to_owned()))
    );
}

pub fn clearing_forgets_the_credential(store: &dyn CredentialStore) {
    assert!(store.save(&credential("ana@example.com", "s3cret")).is_ok());

    assert!(store.clear().is_ok());

    assert_eq!(loaded(store), None);
}

pub fn clearing_an_empty_store_is_not_an_error(store: &dyn CredentialStore) {
    assert!(store.clear().is_ok());

    assert_eq!(loaded(store), None);
}

/// Generates one `#[test]` per check of the contract, run against a store
/// built by the given expression.
#[macro_export]
macro_rules! credential_store_contract {
    ($make_store:expr) => {
        #[test]
        fn a_new_store_holds_no_credential() {
            $crate::credential_store_contract::a_new_store_holds_no_credential(&$make_store);
        }

        #[test]
        fn a_saved_credential_is_loaded_back_unchanged() {
            $crate::credential_store_contract::a_saved_credential_is_loaded_back_unchanged(
                &$make_store,
            );
        }

        #[test]
        fn saving_again_replaces_the_previous_credential() {
            $crate::credential_store_contract::saving_again_replaces_the_previous_credential(
                &$make_store,
            );
        }

        #[test]
        fn clearing_forgets_the_credential() {
            $crate::credential_store_contract::clearing_forgets_the_credential(&$make_store);
        }

        #[test]
        fn clearing_an_empty_store_is_not_an_error() {
            $crate::credential_store_contract::clearing_an_empty_store_is_not_an_error(
                &$make_store,
            );
        }
    };
}
