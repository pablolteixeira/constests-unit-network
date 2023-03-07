use frame_support::pallet_prelude::*;
use frame_support::inherent::Vec;

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Users<AccountId, Balance> {
    pub(super) user_address: AccountId,
    pub(super) first_name: Vec<u8>,
    pub(super) last_name: Vec<u8>,
    pub(super) date_of_birth: Vec<u8>,
    pub(super) bio: Vec<u8>,
    pub(super) email: Vec<u8>,
    pub(super) created_at: Vec<u8>,
    pub(super) updated_at: Vec<u8>,
    pub username: Vec<u8>,
    pub(super) location: Vec<u8>,
    pub(super) last_seen_at: Vec<u8>,
    pub(super) token_balance: Balance,   
    pub(super) language_code: Vec<u8>,
    pub(super) invited_by_user_id: u32,
    pub(super) startprice: u32,
    pub(super) website: Vec<u8>,
    pub(super) linkedin: Vec<u8>,
    pub(super) twitter: Vec<u8>,
    pub(super) instagram: Vec<u8>,
    pub(super) telegram: Vec<u8>,
    pub(super) youtube_url: Vec<u8>,
    pub(super) facebook: Vec<u8>,
    pub(super) vision: Vec<u8>,
    pub(super) tag_line: Vec<u8>,
    pub(super) unit_balance: u32,
    pub(super) unit_sent: u32,
    pub(super) unit_received: u32,
    pub(super) total_deposited_at_time_usd: u32,
    pub(super) total_deposited_now_usd: u32,
    pub(super) total_withdrawn_at_time_usd: u32,
    pub(super) total_withdrawn_now_usd: u32,
    pub(super) exchange_volume: u32,
    pub(super) pin_code: u32,
    pub(super) following_count: u32,
    pub(super) follower_count: u32,
    pub user_id: u32,
}


#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Connections<AccountId> {
    pub(super) connection_from_user_address: AccountId,
    pub(super) connection_to_user_address: AccountId,
    pub(super) last_message: Vec<u16>,
    pub(super) last_seen_at: Vec<u8>,
    pub(super) created_at: Vec<u8>,
    pub(super) updated_at: Vec<u8>,
    pub(super) last_message_time: Vec<u8>,
}


#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Follows<AccountId> {
    pub(super) follow_from_user_address: AccountId,
    pub(super) follow_from_token_symbol: Vec<u8>,
    pub(super) follow_to_user_address: AccountId,
    pub(super) follow_to_token_symbol: Vec<u8>,
    pub(super) created_at: Vec<u8>,
    pub(super) updated_at: Vec<u8>,
}

#[derive(Clone,Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Message<AccountId> {
    pub(super) message_from: AccountId,
    pub(super) message_to: AccountId,
    pub(super) message: Vec<u16>,
    pub(super) created_at: Vec<u8>,
    pub(super) updated_at: Vec<u8>,
}