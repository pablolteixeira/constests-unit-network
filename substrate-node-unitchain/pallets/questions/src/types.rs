use codec::{Decode, Encode};
use frame_support::{
	pallet_prelude::{BoundedVec, MaxEncodedLen},
	traits::Get,
};
use scale_info::TypeInfo;

// Question
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
#[scale_info(skip_type_params(S))]
pub struct Question<AccountId, AssetId, S: Get<u32>> {
	pub owner: AccountId,
	pub asset_id: AssetId,
	pub asset_name: BoundedVec<u8, S>,
	pub title: BoundedVec<u8, S>,
	pub description: BoundedVec<u8, S>,
	pub video_link: BoundedVec<u8, S>,
}

// Anwser
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
#[scale_info(skip_type_params(S))]
pub struct Answer<AccountId, S: Get<u32>> {
	pub owner: AccountId,
	pub answer: BoundedVec<u8, S>,
	pub votes: u128,
}
