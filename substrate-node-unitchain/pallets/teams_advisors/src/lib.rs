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
	pub trait Config: frame_system::Config + pallet_assets::Config + pallet_profile::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	pub type BalanceOf<T> = <T as pallet_assets::Config>::Balance;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct Member<Balance, BlockNumber> {
    	pub username: Vec<u8>,
		pub token_quantity: Balance,
		pub cliff_period: BlockNumber,
		pub vest_period: BlockNumber,
		pub user_id: u32,
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct Advisor {
    	pub username: Vec<u8>,
		pub user_id: u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type AllMembers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<Member<BalanceOf<T>, T::BlockNumber>>>;

	#[pallet::storage]
	#[pallet::getter(fn advisors)]
	pub type Advisors<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<Advisor>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MemberAdded {
			asset_id: T::AssetId,
			username: Vec<u8>,
			token_quantity: BalanceOf<T>,
			cliff_period: T::BlockNumber,
			vest_period: T::BlockNumber,
			user_id: u32,
		},
		MemberUpdated {
			asset_id: T::AssetId,
			username: Vec<u8>,
			new_token_quantity: BalanceOf<T>,
			new_cliff_period: T::BlockNumber,
			new_vest_period: T::BlockNumber,
		},
		MemberDeleted {
			asset_id: T::AssetId,
			username: Vec<u8>,
		},
		AdvisorCreated {
			asset_id: T::AssetId,
			username: Vec<u8>,
			user_id: u32,
		},
		AdvisorDeleted {
			asset_id: T::AssetId,
			username: Vec<u8>,
		}
	}
	

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Only the token owner can call this functions
		NotOwnerIssuerOrAdmin,
		/// Invalid asset id
		InvalidAsset,
		/// There is no profiles created
		NoProfilesCreated,
		/// A profile with the given username does not exist in the profiles pallet
		InvalidUsername,
		/// The user is already a member
		AlreadyMember,
		/// The member is not registered
		MemberDoesNotExist,
		/// A member has been deleted
		MemberDeleted,
		/// The user is already an advisor
		AdvisorAlreadyExists,
		/// The advisor is not registered
		AdvisorDoesNotExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_member(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>, token_quantity: BalanceOf<T>, cliff_period: T::BlockNumber, vest_period: T::BlockNumber) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			// check if the sender has permission to call this extrinsic
			Self::check_permission(who.clone(), asset_id.clone())?;
			
			let user_id = Self::is_valid_username(username.clone())?;

			let new_member = Member {
				username: username.clone(),
				token_quantity: token_quantity.clone(),
				cliff_period: cliff_period.clone(),
				vest_period: vest_period.clone(),
				user_id: user_id.clone(),
			};

			match AllMembers::<T>::get(asset_id) {
				None => {
					let mut members_vec = Vec::new();
					members_vec.push(new_member.clone());
					AllMembers::<T>::set(asset_id, Some(members_vec));
				} 
				Some(x) => {
					let mut members_vec = x.clone();
					// check if the user is already a member
					for member in members_vec.iter() {
						if member.username == username {
							return Err(Error::<T>::AlreadyMember.into());
						}
					}
					members_vec.push(new_member.clone());
					AllMembers::<T>::set(asset_id, Some(members_vec));
				} 
			}

			Self::deposit_event(Event::MemberAdded {
				asset_id: asset_id,
				username: username,
				token_quantity: token_quantity,
				cliff_period: cliff_period,
				vest_period: vest_period,
				user_id: user_id,
			});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_member(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>, new_token_quantity: BalanceOf<T>, new_cliff_period: T::BlockNumber, new_vest_period: T::BlockNumber) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			// check if the sender has permission to call this extrinsic
			Self::check_permission(who.clone(), asset_id.clone())?;
			
			let mut members_vec = AllMembers::<T>::get(asset_id).ok_or(Error::<T>::MemberDoesNotExist)?;
			// check if the member exists
			for member in members_vec.iter_mut() {
				if member.username == username {
					member.token_quantity = new_token_quantity.clone();
					member.cliff_period = new_cliff_period.clone();
					member.vest_period = new_vest_period.clone();
					AllMembers::<T>::set(asset_id, Some(members_vec));
					Self::deposit_event(Event::MemberUpdated {
						asset_id: asset_id,
						username: username,
						new_token_quantity: new_token_quantity,
						new_cliff_period: new_cliff_period,
						new_vest_period: new_vest_period,
					});
					return Ok(());
				}
			}

			Err(Error::<T>::MemberDoesNotExist.into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn delete_member(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			// check if the sender has permission to call this extrinsic
			Self::check_permission(who.clone(), asset_id.clone())?;
			
			let mut members_vec = AllMembers::<T>::get(asset_id).ok_or(Error::<T>::MemberDoesNotExist)?;
			// check if the member exists and remove it
			for member in members_vec.iter_mut() {
				if member.username == username {
					members_vec.retain(|x| x.username != username);
					AllMembers::<T>::set(asset_id, Some(members_vec));
					Self::deposit_event(Event::MemberDeleted {
						asset_id: asset_id,
						username: username,
					});
					return Ok(());
				}
			}

			Err(Error::<T>::MemberDoesNotExist.into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_advisor(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			// check if the sender has permission to call this extrinsic
			Self::check_permission(who.clone(), asset_id.clone())?;
			
			let user_id = Self::is_valid_username(username.clone())?;

			match Advisors::<T>::get(asset_id) {
				Some(mut advisors_vec) => {
					// check if the advisor exists
					for advisor in advisors_vec.iter() {
						if advisor.username == username {
							return Err(Error::<T>::AdvisorAlreadyExists.into());
						}
					}
					advisors_vec.push(Advisor {
						username: username.clone(),
						user_id: user_id,
					});
					Advisors::<T>::set(asset_id, Some(advisors_vec));
					Self::deposit_event(Event::AdvisorCreated {
						asset_id: asset_id,
						username: username,
						user_id: user_id,
					});
					return Ok(());
				},
				None => {
					let mut advisors_vec = Vec::new();
					advisors_vec.push(Advisor {
						username: username.clone(),
						user_id: user_id,
					});

					Advisors::<T>::set(asset_id, Some(advisors_vec));
					Self::deposit_event(Event::AdvisorCreated {
						asset_id: asset_id,
						username: username,
						user_id: user_id,
					});
					return Ok(());
				}
			}
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn remove_advisor(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			// check if the sender has permission to call this extrinsic
			Self::check_permission(who.clone(), asset_id.clone())?;
			
			let mut advisors_vec = Advisors::<T>::get(asset_id).ok_or(Error::<T>::AdvisorDoesNotExist)?;

			// check if the advisor exists and remove it
			for advisor in advisors_vec.iter_mut() {
				if advisor.username == username {
					advisors_vec.retain(|x| x.username != username);
					Advisors::<T>::set(asset_id, Some(advisors_vec));
					Self::deposit_event(Event::AdvisorDeleted {
						asset_id: asset_id,
						username: username,
					});
					return Ok(());
				}
			}

			Err(Error::<T>::AdvisorDoesNotExist.into())
		}
	}

	impl<T: Config> Pallet<T> {
		//check if the username is valid
		fn is_valid_username(username: Vec<u8>) -> Result<u32, DispatchError> {
			let users = pallet_profile::AllUsers::<T>::get().ok_or(Error::<T>::NoProfilesCreated)?;
			// find username in the users list
			for i in 0..users.len(){
				if users[i].username == username {
					return Ok(users[i].user_id);
				}
			}
			return Err(Error::<T>::InvalidUsername.into());
		}

		fn check_permission(who: T::AccountId, asset_id: T::AssetId) -> DispatchResult {
			// check if the sender has permission to add members
			let asset_owner = <pallet_assets::Pallet<T> as fungibles::roles::Inspect<T::AccountId>>::owner(asset_id).ok_or(Error::<T>::InvalidAsset)?;
			let asset_admin = <pallet_assets::Pallet<T> as fungibles::roles::Inspect<T::AccountId>>::admin(asset_id).ok_or(Error::<T>::InvalidAsset)?;
			let asset_issuer = <pallet_assets::Pallet<T> as fungibles::roles::Inspect<T::AccountId>>::issuer(asset_id).ok_or(Error::<T>::InvalidAsset)?;
			ensure!((who == asset_owner) || (who == asset_admin) || (who == asset_issuer), Error::<T>::NotOwnerIssuerOrAdmin);
			Ok(())
		}
	}
}

