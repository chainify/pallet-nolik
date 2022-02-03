use crate as pallet_nolik;
//use sp_core::H256;
use sp_runtime::testing::H256;
use frame_support::parameter_types;
use frame_support::sp_io::TestExternalities;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;
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
		Nolik: pallet_nolik::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
    pub const MaxAddressOwners: u32 = 4;
    pub const MaxWhiteListAddress: u32 = 4;
    pub const MaxBlackListAddress: u32 = 4;
}

impl pallet_nolik::Config for Test {
	type Event = Event;
    type MaxAddressOwners = MaxAddressOwners;
    type MaxWhiteListAddress = MaxWhiteListAddress;
    type MaxBlackListAddress = MaxBlackListAddress;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
