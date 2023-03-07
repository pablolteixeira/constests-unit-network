use frame_support::pallet_prelude::*;
use frame_support::inherent::Vec;

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct StoreDetails<AccountId, AssetId> {
    pub(super) title: Vec<u8>,
    pub(super) description: Vec<u8>,
    pub(super) owner: AccountId,
    pub(super) username: Vec<u8>,
    pub(super) assets: Vec<AssetId>,
    pub(super) store_id: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ProductDetails<AccountId,Balance> {
    pub(super) title: Vec<u8>,
    pub(super) description: Vec<u8>,
    pub(super) owner: AccountId,
    pub(super) username: Vec<u8>,
    pub(super) product_id: u32,
    pub(super) store_id: u32,
    pub(super) product_price: Balance,
    pub(super) stat_code: Vec<u8>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct OrderDetails<AccountId, AssetId> {
    pub(super) title: Vec<u8>,
    pub(super) description: Vec<u8>,
    pub(super) owner: AccountId,
    pub(super) product_id: u32,
    pub(super) store_id: u32,
    pub(super) order_id: u32,
    pub(super) product_price: u32,
    pub(super) stat_code: Vec<u8>,
    pub(super) asset: AssetId,
    pub(super) buyer_name: Vec<u8>,
    pub(super) order_confirmed: bool,
    pub(super) order_closed: bool,
}