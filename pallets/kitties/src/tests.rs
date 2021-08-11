pub super::*;

use crate as kitties;
use sp_core::H256;
use frame_support::{
    parameter_types, assert_ok, assert_noop, error::BadOrigin, unsigned::ValidateUnsigned,
};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, testing::Header,
    testing::TestXt,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where 
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config, Event<T>},
        KittiesModule: kitties::{Pallet, Call, Storage, Event<T>, Config},
        NFT: orml_nft::{Pallet, Storage, Config<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}