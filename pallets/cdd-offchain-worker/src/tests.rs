//! Tests for the module.

use crate::*;
use chrono::prelude::Utc;
use codec::{Decode, Encode};
use frame_support::{assert_err, assert_noop, assert_ok, dispatch::DispatchError};
use mock::*;
use primitives::{AccountKey, IdentityId, Signatory};
use sp_core::{
    offchain::{testing, OffchainExt, TransactionPoolExt},
    testing::KeyStore,
    traits::KeystoreExt,
    H256,
};
use sp_runtime::RuntimeAppPublic;
use test_client::AccountKeyring;

#[test]
fn check_the_initial_nominators_of_chain() {
    ExtBuilder::default()
        .nominate(true)
        .build()
        .execute_with(|| {
            // Check the initial nominator status and expiry
            assert!(Staking::nominators(account_from(101)).is_some());
            // Check the identity expiry
            assert!(Identity::has_valid_cdd(IdentityId::from(101)));
        });
}

#[test]
fn should_submit_unsigned_transaction_on_chain() {
    const PHRASE: &str =
        "foster nation swing usage bread mind donor door whisper lyrics token enroll";

    let (offchain, offchain_state) = testing::TestOffchainExt::new();
    let (pool, pool_state) = testing::TestTransactionPoolExt::new();
    let keystore = KeyStore::new();
    keystore
        .write()
        .sr25519_generate_new(
            crate::crypto::SignerId::ID,
            Some(&format!("{}/worker1", PHRASE)),
        )
        .unwrap();

    let mut txn = ExtBuilder::default().build();
    txn.register_extension(OffchainExt::new(offchain));
    txn.register_extension(TransactionPoolExt::new(pool));
    txn.register_extension(KeystoreExt(keystore));

    txn.execute_with(|| {
        let invalid_nominators = Staking::fetch_invalid_cdd_nominators(0);
        // CddOffchainWorker::remove_invalidate_nominators(invalid_nominators.clone()).unwrap();
        // let tx = pool_state.write().transactions.pop().unwrap();
        // assert!(pool_state.read().transactions.is_empty());
        // let tx = Extrinsic::decode(&mut &*tx).unwrap();
        // assert_eq!(tx.signature.unwrap().0, 0);
        // assert_eq!(tx.call, Call::remove_nominator(invalid_nominators.clone()));
    });
}
