use crate as pallet_polls;
use crate::*;
use frame_support::{ 
parameter_types, assert_ok,
traits::{ConstU16, ConstU64, ConstU32, AsEnsureOriginWithArg, EqualPrivilegeOnly, Hooks},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
pub use sp_runtime::{Perbill, Permill};


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

type Balance = u64;
type AccountId = u64;
type PollIndex = u64;

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
		Scheduler: pallet_scheduler,
		Polls: pallet_polls,
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
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type MaxLocks = ConstU32<10>;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const AssetDeposit: Balance = 0;
	pub const AssetAccountDeposit: Balance = 0;
	pub const ApprovalDeposit: Balance = 0;
	pub const MetadataDepositBase: Balance = 0;
	pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type RuntimeEvent = RuntimeEvent;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetIdParameter = codec::Compact<u32>;
	type Balance = Balance;
	type AssetId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = frame_support::traits::ConstU32<20>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
}

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(
			Weight::from_parts(frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
		);
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = frame_system::EnsureRoot<u64>;
	type MaxScheduledPerBlock = ConstU32<100>;
	type WeightInfo = ();
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = ();
}

impl pallet_polls::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Fungibles = Assets;
	type PollIndex = PollIndex;
	type PollCall = RuntimeCall;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn next_block() {
	System::set_block_number(System::block_number() + 1);
	Scheduler::on_initialize(System::block_number());
}


pub fn fast_forward_to(n: u64) {
	while System::block_number() < n {
		next_block();
	}
}

pub fn begin_poll(who: u64) -> PollIndex {
	System::set_block_number(0);
	// create a testint poll
	let res = Polls::create_poll(
		RuntimeOrigin::signed(who),
		(0..46).collect(),
		4,
		pallet_polls::PollCurrency::Native,
		1,
		10,
		// set min balance as 0
		0,
	);
	assert_ok!(res);
	fast_forward_to(2);
	1
}

pub fn begin_poll_with_asset(
	who: u64,
	voter: u64,
	balance: Balance,
) -> (PollIndex, u32) {
	System::set_block_number(0);
	// Create asset
	let asset_id = 0;
	assert_ok!(Assets::create(RuntimeOrigin::signed(who), asset_id.into(), who, 1));
	assert_ok!(Assets::mint(RuntimeOrigin::signed(who), asset_id.into(), voter, balance));

	// Create poll
	let res = Polls::create_poll(
		RuntimeOrigin::signed(who),
		(0..46).collect(),
		4,
		PollCurrency::Asset(0),
		1,
		10,
		// set min balance as 0
		0,
	);

	assert_ok!(res);
	fast_forward_to(2);
	(1, asset_id)
}

pub fn set_balances(acc: u64) {
	assert_ok!(Balances::set_balance(RuntimeOrigin::root(), acc, 20, 0));
	//assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Polls::account_id(), 1, 0));
	assert_eq!(Balances::free_balance(acc), 20);
}
