#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

use frame_support::{
	pallet_prelude::*};

use frame_system::pallet_prelude::*;


#[frame_support::pallet]
pub mod pallet {
	use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

	pub type AssetIdOf<T> = <T as Config>::AssetId;

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>; 

		type AssetId: Member 
					+ Parameter 
					+ Copy 
					+ MaybeSerializeDeserialize 
					+ MaxEncodedLen
					+ Default;

		#[pallet::constant]
		type MaxTitleLength: Get<u32>;

		#[pallet::constant]
		type MaxTokenSymbolLength: Get<u32>;

    }

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct Contests<T: Config> {
		title: Vec<u8>,
		user_address: T::AccountId,
		prize_token_id: AssetIdOf<T>,
		prize_token_winner: u32,
		token_symbol: Vec<u8>,
		// statcode states -> true: open; false: closed.
		statcode: bool,
		contest_end_date: Vec<u8>,
		description: Vec<u8>
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct ContestsEntries<T: Config> {
		user_address: T::AccountId,
		contest_id: u32,
		winner: bool,
		winner_transfer_id: u32
	}

	#[pallet::event]
	pub enum Event<T> {

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn contest_new(
			origin: OriginFor<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn create_contest(
			origin: OriginFor<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn update_contest(
			origin: OriginFor<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn create_contest_entry(
			origin: OriginFor<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn assign_contest_winner(
			origin: OriginFor<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn close_contract(
			origin: OriginFor<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}
	}
}