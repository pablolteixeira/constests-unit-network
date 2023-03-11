use crate as pallet_contests;
use frame_support::{
	parameter_types,
	PalletId,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, ConstU64, ConstU128}};
use frame_system::{EnsureSigned, EnsureRoot};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
        Assets: pallet_assets,
        Balances: pallet_balances,
		Contests: pallet_contests,
	}
);

impl frame_system::Config for Test {
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
	type AssetId = u32;
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
	pub const MaxLocks: u32 = 100;
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
    pub const ContestPalletId: PalletId = PalletId(*b"unittask");
    pub const MaxTitleLength: u32 = 50;
    pub const MinTitleLength: u32 = 10;
    pub const MaxTokenSymbolLength: u32 = 10;
    pub const MinTokenSymbolLength: u32 = 3;
    pub const MaxContestEndDateLength: u32 = 12;
    pub const MinContestEndDateLength: u32 = 8;
    pub const MaxDescriptionLength: u32 = 350;
    pub const MinDescriptionLength: u32 = 100;
    pub const MinTokenAmount: u32 = 10;
    pub const MinTokenWinner: u32 = 1;
}

impl pallet_contests::Config for Test {
	type RuntimeEvent = RuntimeEvent;
    type Assets = Assets;
    type AssetBalance = u128;
    type AssetId = u32;
    type PalletId = ContestPalletId;
    type MaxTitleLength = MaxTitleLength;
    type MinTitleLength = MinTitleLength;
    type MaxTokenSymbolLength = MaxTokenSymbolLength;
    type MinTokenSymbolLength = MinTokenSymbolLength;
    type MaxContestEndDateLength = MaxContestEndDateLength;
    type MinContestEndDateLength = MinContestEndDateLength;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MinDescriptionLength = MinDescriptionLength;
    type MinTokenAmount = MinTokenAmount;
    type MinTokenWinner = MinTokenWinner;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
