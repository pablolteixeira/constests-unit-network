#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
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