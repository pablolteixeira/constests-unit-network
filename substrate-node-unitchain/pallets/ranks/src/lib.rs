#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::traits::fungibles;
use sp_std::vec::Vec;
use sp_std::vec;	

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	pub type BalanceOf<T> = <T as pallet_assets::Config>::Balance;

	// Ranks type
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct RankInfo<Balance> {
    	pub name: Vec<u8>,
		pub min_tokens: Balance,
	}

	#[pallet::storage]
	#[pallet::getter(fn ranks)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Ranks<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<RankInfo<BalanceOf<T>>>
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RankCreated {
			asset_id: T::AssetId, rank: RankInfo<BalanceOf<T>>
		},
		RankUpdated {
			asset_id: T::AssetId, new_rank: RankInfo<BalanceOf<T>>
		},
		RankDeleted {
			asset_id: T::AssetId, removed_rank: RankInfo<BalanceOf<T>>
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Only the token owner can call this functions
		OnlyTokenOwner,
		/// Invalid asset id
		InvalidAsset,
		/// No ranks set for Asset
		NoRanksAvailable,
		/// A rank with the same name already exist
		RankNameUsed,
		/// A rank with the same amount of tokens exists
		RankMinTokensUsed,
		/// The rank name dont exist
		InvalidRankName
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_rank(origin: OriginFor<T>, asset_id: T::AssetId, name: Vec<u8>, min_tokens: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			let new_rank = RankInfo{
				name: name.clone(),
				min_tokens: min_tokens.clone(),
			};

			// get asset owner and check that only the caller its the owner
			let asset_owner = <pallet_assets::Pallet<T> as fungibles::roles::Inspect<T::AccountId>>::owner(asset_id).ok_or(Error::<T>::InvalidAsset)?;
			ensure!(who == asset_owner, Error::<T>::OnlyTokenOwner);

			// if there is already ranks, push the new rank to the vec or create the first rank for the asset
			match Ranks::<T>::get(asset_id) {
				None => {
					let mut rank_vec = Vec::new();
					rank_vec.push(new_rank.clone());
					Ranks::<T>::set(asset_id, Some(rank_vec));
				} 
				Some(x) => {
					// check if already exist a rank with the same name or the same amount of tokens
					for i in x.iter() {
						if i.name == name {
							return Err(Error::<T>::RankNameUsed.into());
						} else if i.min_tokens == min_tokens {
							return Err(Error::<T>::RankMinTokensUsed.into());
						}
					}
					let mut rank_vec = x.clone();
					rank_vec.push(new_rank.clone());
					Ranks::<T>::set(asset_id, Some(rank_vec));
				} 
			}

			Self::deposit_event(Event::RankCreated{asset_id, rank: new_rank});

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn update_rank(origin: OriginFor<T>, asset_id: T::AssetId, rank_name: Vec<u8>, new_rank_name: Vec<u8>, new_min_tokens: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let asset_owner = <pallet_assets::Pallet<T> as fungibles::roles::Inspect<T::AccountId>>::owner(asset_id).ok_or(Error::<T>::InvalidAsset)?;
			ensure!(who == asset_owner, Error::<T>::OnlyTokenOwner);

			let mut ranks = Ranks::<T>::get(asset_id).ok_or(Error::<T>::NoRanksAvailable)?;

			for i in 0..ranks.len(){
				if ranks[i].name == rank_name {
					ranks[i].name = new_rank_name;
					ranks[i].min_tokens = new_min_tokens;
					// check if theres a rank with the same name or the same amount of tokens
					for j in 0..ranks.len(){
						if i != j {
							if ranks[i].name == ranks[j].name {
								return Err(Error::<T>::RankNameUsed.into());
							} else if ranks[i].min_tokens == ranks[j].min_tokens {
								return Err(Error::<T>::RankMinTokensUsed.into());
							}
						}
					}
					Ranks::<T>::set(asset_id, Some(ranks.clone()));
					Self::deposit_event(Event::RankUpdated{asset_id, new_rank: ranks[i].clone()});
					return Ok(());
				}
			}
			return Err(Error::<T>::InvalidRankName.into());
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn delete_rank(origin: OriginFor<T>, asset_id: T::AssetId, rank_name: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let asset_owner = <pallet_assets::Pallet<T> as fungibles::roles::Inspect<T::AccountId>>::owner(asset_id).ok_or(Error::<T>::InvalidAsset)?;
			ensure!(who == asset_owner, Error::<T>::OnlyTokenOwner);

			let mut ranks = Ranks::<T>::get(asset_id).ok_or(Error::<T>::NoRanksAvailable)?;

			for i in 0..ranks.len(){
				if ranks[i].name == rank_name {
					Self::deposit_event(Event::RankDeleted{asset_id, removed_rank: ranks[i].clone()});
					ranks.remove(i);
					Ranks::<T>::set(asset_id, Some(ranks));
					return Ok(());
				}
			}

			return Err(Error::<T>::InvalidRankName.into());
		}
	}
}
