use crate as pallet_oracle;
use crate::*;
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32, ConstU64, SortedMembers},
};
use frame_system as system;
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::H256;
use std::cell::RefCell;

use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

type Balance = u128;
type AccountId = u128;
type AssetId = u32;

type Key = u32;
type Value = u32;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;
pub const DEV: AccountId = 4;

thread_local! {
	static TIME: RefCell<u32> = RefCell::new(0);
}

pub struct Timestamp;
impl Time for Timestamp {
	type Moment = u32;

	fn now() -> Self::Moment {
		TIME.with(|v| *v.borrow())
	}
}

impl Timestamp {
	pub fn set_timestamp(val: u32) {
		TIME.with(|v| *v.borrow_mut() = val);
	}
}

pub struct Members;

impl SortedMembers<AccountId> for Members {
	fn sorted_members() -> Vec<AccountId> {
		OracleMembers::get()
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Oracle: pallet_oracle,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const AssetDeposit: Balance = 100;
	pub const ApprovalDeposit: Balance = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10;
	pub const MetadataDepositPerByte: Balance = 1;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = AssetId;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<1>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxLocks: u32 = 10;
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub static OracleMembers: Vec<AccountId> = vec![ALICE, BOB, CAROL];
}

impl pallet_oracle::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CombineData = DefaultCombineData<Self, ConstU32<1>, ConstU32<600>>;
	type Time = Timestamp;
	type Members = Members;
	type OracleKey = AssetId;
	type OracleValue = Value;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	sp_io::TestExternalities::from(storage.clone());

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(ALICE, 1000), (BOB, 1000), (CAROL, 1000), (DEV, 2000)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	pallet_oracle::GenesisConfig::<Test> { oracles_key: vec![(1, 12), (2, 12), (3, 12)] }
		.assimilate_storage(&mut storage)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);

	ext.execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(100);
	});

	ext
}
