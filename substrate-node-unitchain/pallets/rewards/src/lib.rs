#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
mod types;
pub use types::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*};
	use frame_support::inherent::Vec;
	// use scale_info::prelude::vec::Vec;
	// use types::*;
	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;


	#[pallet::storage]
	#[pallet::getter(fn reward_item)]
	pub(super) type RewardItem<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Rewards<T::AccountId,T::AssetId>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn reward_id)]
	pub type RewardId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn all_rewards)]
	pub type AllRewards<T: Config> = StorageValue<_, Vec<Rewards<T::AccountId, T::AssetId>>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
		RewardCreated { user_address: T::AccountId, asset_id: T::AssetId, title: Vec<u8>, url_link: Vec<u8>, min_tokens: u32 , reward_id: u32},			
		RewardUpdated { user_address: T::AccountId, asset_id: T::AssetId, title: Vec<u8>, url_link: Vec<u8>, min_tokens: u32 , reward_id: u32},			
		RewardDeleted { reward_id: u32 }
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_reward(origin: OriginFor<T>, asset_id: T::AssetId, title: Vec<u8>, url_link: Vec<u8>, min_tokens: u32 ) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// making sure storage value is not null
			if RewardId::<T>::get().is_none(){
				RewardId::<T>::put(0)
			}

            let current_store_id = RewardId::<T>::get();
			let new_id = current_store_id.unwrap() + 1;

			let reward = Rewards {
				asset_id: asset_id.clone(),
				user_address: who.clone(),
				title: title.clone(),
				url_link: url_link.clone(),
				min_tokens: min_tokens.clone(),
				reward_id: new_id.clone()
			};
			
			// making sure storage value is not null
			if AllRewards::<T>::get().is_none(){
				let empty_vec : Vec<Rewards<T::AccountId, T::AssetId>> = Vec::new();
				AllRewards::<T>::put(empty_vec);
			}

			// add to all rewards
			let mut all_rewards = AllRewards::<T>::get().unwrap();
			all_rewards.push(reward.clone());
			AllRewards::<T>::put(all_rewards);

            // add to store item
            RewardItem::<T>::insert(new_id, reward);

            // update the store id
            RewardId::<T>::put(new_id);

			Self::deposit_event(Event::RewardCreated { 	
				asset_id: asset_id,
				user_address: who,
				title: title,
				url_link: url_link,
				min_tokens: min_tokens,
				reward_id: new_id 
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn delete_reward(origin: OriginFor<T>, reward_id: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// removing from all rewards
			let mut all_rewards = AllRewards::<T>::get().unwrap();
			// have to remove old store 
			let old_store = RewardItem::<T>::get(reward_id).unwrap();
			all_rewards.retain(|x| *x != old_store);
			// updating the rewards
			AllRewards::<T>::put(all_rewards);

			// removing reward item
			RewardItem::<T>::remove(reward_id);

			Self::deposit_event(Event::RewardDeleted {
				reward_id: reward_id 
			});

			Ok(())
		}


		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_reward(origin: OriginFor<T>, asset_id: T::AssetId, title: Vec<u8>, url_link: Vec<u8>, min_tokens: u32, reward_id: u32 ) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// removing from all rewards
			let mut all_rewards = AllRewards::<T>::get().unwrap();
			// have to remove old store 
			let old_store = RewardItem::<T>::get(reward_id).unwrap();
			all_rewards.retain(|x| *x != old_store);

			let reward = Rewards {
				asset_id: asset_id.clone(),
				user_address: who.clone(),
				title: title.clone(),
				url_link: url_link.clone(),
				min_tokens: min_tokens.clone(),
				reward_id: reward_id.clone()
			};

			all_rewards.push(reward.clone());
			AllRewards::<T>::put(all_rewards);

            // add to store item
            RewardItem::<T>::insert(reward_id, reward);

			Self::deposit_event(Event::RewardUpdated { 	
				asset_id: asset_id,
				user_address: who,
				title: title,
				url_link: url_link,
				min_tokens: min_tokens,
				reward_id: reward_id 
			});

			Ok(())
		}
	}
}

// have to add events