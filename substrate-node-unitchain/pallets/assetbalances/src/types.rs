use frame_support::pallet_prelude::*;
use frame_support::inherent::Vec;

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TypeBalances<AssetId> {
    // pub(super) id: u64,
    pub(super) currency: Vec<u8>,
    pub(super) token_type: Vec<u8>,
    pub(super) token_id: AssetId,
    pub(super) balance: u128,
}

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct UserFeatureBalances<AccountId,AssetId> {
    // pub(super) id: u64,
    pub(super) user_address: AccountId,    
    pub(super) currency: Vec<u8>,
    pub(super) token_type: Vec<u8>,
    pub(super) token_id: AssetId,
    pub(super) balance: u128,
}

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ExchangeShareTransfers<AccountId,AssetId> {
    pub(super) id: u64,
    pub(super) transfer_from_user_address: AccountId,    
    pub(super) transfer_to_user_address: AccountId,
    pub(super) token_id: AssetId,
    pub(super) amount: u128,
}

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ExchangeStakingOwnershipShares<AccountId,AssetId> {
    // pub(super) id: u64,
    pub(super) user_address: AccountId,    
    pub(super) token_id: AssetId,
    pub(super) ownership_shares: u128,
}

// transfers
#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Transfers<AccountId,AssetId,Balance> {
    pub(super) id: u64,
    pub(super) transfer_from_user_address: AccountId,    
    pub(super) transfer_to_user_address: AccountId,
    pub(super) transfer_from_feature_token_id: AssetId,
    pub(super) transfer_from_feature: Vec<u8>,
    pub(super) transfer_to_feature_token_id: AssetId,
    pub(super) transfer_to_feature: Vec<u8>,
    pub(super) transfer_from_useraddress_balance_before_transfer: Balance,
    pub(super) transfer_from_useraddress_balance_after_transfer: Balance,
    pub(super) transfer_to_useraddress_balance_before_transfer: Balance,
    pub(super) transfer_to_useraddress_balance_after_transfer: Balance,
    pub(super) transfer_from_token_feature_balance_before_transfer: u64,
    pub(super) transfer_from_token_feature_balance_after_transfer: u64,
    pub(super) transfer_to_token_feature_balance_before_transfer: u64,
    pub(super) transfer_to_token_feature_balance_after_transfer: u64,
    pub(super) token_id: AssetId,
    pub(super) currency: Vec<u8>,
    pub(super) exchange_stakings: u64,
    pub(super) transfer_type: Vec<u8>,
    pub(super) reference: Vec<u8>,
    pub(super) transferuserid: Vec<u8>,
    pub(super) amount: u128,
}