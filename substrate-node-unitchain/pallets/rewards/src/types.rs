use frame_support::pallet_prelude::*;
use frame_support::inherent::Vec;

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Rewards<AccountId, AssetId> {
    pub(super) asset_id: AssetId,
    pub(super) user_address: AccountId,
    pub(super) title: Vec<u8>,
    pub(super) url_link: Vec<u8>,
    pub(super) min_tokens: u32,
    pub(super) reward_id: u32,   
}


#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct StoreDetails<AccountId, AssetId> {
    pub(super) title: Vec<u8>,
    pub(super) description: Vec<u8>,
    pub(super) owner: AccountId,
    pub(super) username: Vec<u8>,
    pub(super) assets: Vec<AssetId>,
    pub(super) store_id: u32,
}