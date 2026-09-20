use test_support::credentials::InMemoryCredentialStore;

test_support::credential_store_contract!(InMemoryCredentialStore::new());
