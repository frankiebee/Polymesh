use crate::asset::{self, AssetTrait};
use crate::utils;

use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait + utils::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Asset: asset::AssetTrait<Self::AccountId, Self::TokenBalance>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as exemption {
        // Mapping -> ExemptionList[ticker][TM][Account] = true/false
        ExemptionList get(exemption_list): map (Vec<u8>, u16, T::AccountId) => bool;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

        fn modify_exemption_list(origin, _ticker: Vec<u8>, _tm: u16, asset_holder: T::AccountId, exempted: bool) -> Result {
            let ticker = utils::bytes_to_upper(_ticker.as_slice());
            let sender = ensure_signed(origin)?;
            ensure!(Self::is_owner(ticker.clone(), sender.clone()), "Sender must be the token owner");
            let isExempted = Self::exemption_list((ticker.clone(), _tm, asset_holder.clone()));
            ensure!(isExempted != exempted, "No change in the state");

            <ExemptionList<T>>::insert((ticker.clone(), _tm, asset_holder.clone()), exempted);
            Self::deposit_event(RawEvent::ModifyExemptionList(ticker, _tm, asset_holder, exempted));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ModifyExemptionList(Vec<u8>, u16, AccountId, bool),
    }
);

impl<T: Trait> Module<T> {
    pub fn is_owner(_ticker: Vec<u8>, sender: T::AccountId) -> bool {
        let ticker = utils::bytes_to_upper(_ticker.as_slice());
        T::Asset::is_owner(ticker.clone(), sender)
    }

    pub fn is_exempted(_ticker: Vec<u8>, _tm: u16, who: T::AccountId) -> bool {
        let ticker = utils::bytes_to_upper(_ticker.as_slice());
        Self::exemption_list((ticker, _tm, who))
    }
}

/// tests for this module
#[cfg(test)]
mod tests {
    // use super::*;

    // use primitives::{Blake2Hasher, H256};
    // use runtime_io::with_externalities;
    // use runtime_primitives::{
    //     testing::{Digest, DigestItem, Header},
    //     traits::{BlakeTwo256, IdentityLookup},
    //     BuildStorage,
    // };
    // use support::{assert_ok, impl_outer_origin};

    // impl_outer_origin! {
    //     pub enum Origin for Test {}
    // }

    // // For testing the module, we construct most of a mock runtime. This means
    // // first constructing a configuration type (`Test`) which `impl`s each of the
    // // configuration traits of modules we want to use.
    // #[derive(Clone, Eq, PartialEq)]
    // pub struct Test;
    // impl system::Trait for Test {
    //     type Origin = Origin;
    //     type Index = u64;
    //     type BlockNumber = u64;
    //     type Hash = H256;
    //     type Hashing = BlakeTwo256;
    //     type Digest = Digest;
    //     type AccountId = u64;
    //     type Lookup = IdentityLookup<Self::AccountId>;
    //     type Header = Header;
    //     type Event = ();
    //     type Log = DigestItem;
    // }
    // impl Trait for Test {
    //     type Event = ();
    // }
    // type exemption = Module<Test>;

    // // This function basically just builds a genesis storage key/value store according to
    // // our desired mockup.
    // fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
    //     system::GenesisConfig::<Test>::default()
    //         .build_storage()
    //         .unwrap()
    //         .0
    //         .into()
    // }

    // #[test]
    // fn it_works_for_default_value() {
    //     with_externalities(&mut new_test_ext(), || {
    //         // Just a dummy test for the dummy funtion `do_something`
    //         // calling the `do_something` function with a value 42
    //         assert_ok!(exemption::do_something(Origin::signed(1), 42));
    //         // asserting that the stored value is equal to what we stored
    //         assert_eq!(exemption::something(), Some(42));
    //     });
    // }
}