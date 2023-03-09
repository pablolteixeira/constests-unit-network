#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

use frame_support::{
	sp_runtime::{
		traits::AccountIdConversion, 
		FixedPointOperand},
	PalletId,
	traits::tokens::{
		Balance,
		fungibles::{Transfer, Inspect}},
	pallet_prelude::*};

use frame_system::pallet_prelude::*;


#[frame_support::pallet]
pub mod pallet {
	use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>; 

		type Assets: Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::AssetBalance>
					+ Transfer<Self::AccountId>;

		type AssetBalance: Balance
					+ FixedPointOperand;

		type AssetId: Member 
					+ Parameter 
					+ Copy 
					+ MaybeSerializeDeserialize 
					+ MaxEncodedLen
					+ Default;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type MaxTitleLength: Get<u32>;

		#[pallet::constant]
		type MinTitleLength: Get<u32>;

		#[pallet::constant]
		type MaxTokenSymbolLength: Get<u32>;

		#[pallet::constant]
		type MinTokenSymbolLength: Get<u32>;

		#[pallet::constant]
		type MaxContestEndDateLength: Get<u32>;

		#[pallet::constant]
		type MinContestEndDateLength: Get<u32>;

		#[pallet::constant]
		type MaxDescriptionLength: Get<u32>;

		#[pallet::constant]
		type MinDescriptionLength: Get<u32>;

		#[pallet::constant]
		type MinTokenAmount: Get<u32>;

		#[pallet::constant]
		type MinTokenWinner: Get<u32>;
    }

	pub type AssetBalanceOf<T> = <T as Config>::AssetBalance;

	pub type AssetIdOf<T> = <T as Config>::AssetId;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Contests<T: Config> {
		contest_id: u32,
		title: BoundedVec<u8, T::MaxTitleLength>,
		user_address: T::AccountId,
		prize_token_id: AssetIdOf<T>,
		prize_token_winner: u32,
		token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
		// statcode states -> true: open; false: closed.
		statcode: bool,
		contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength>,
		description: BoundedVec<u8, T::MaxDescriptionLength>
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct ContestsEntries<T: Config> {
		user_address: T::AccountId,
		contest_id: u32,
		winner: bool,
		winner_transfer_id: u32
	}

	#[pallet::storage]
	#[pallet::getter(fn get_contests)]
	pub type ContestsMap<T> = StorageMap<_, Blake2_128Concat, u32, Contests<T>>; 

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ContestCreted { who: T::AccountId, contest_id: u32, title: BoundedVec<u8, T::MaxTitleLength> }
	}

	#[pallet::error]
	pub enum Error<T> {
		ContestIdAlreadyInUse,
		AssetDontExist,
		TitleTooLarge,
		TitleTooSmall,
		TokenSymbolTooLarge,
		TokenSymbolTooSmall,
		DescriptionTooLarge,
		DescriptionTooSmall,
		PrizeTokenWinnerTooSmall,
		AssetBalanceInsufficient,
		TokenAmountTooSmall
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn contest_new(
			origin: OriginFor<T>,
			contest_id: u32,
			title: BoundedVec<u8, T::MaxTitleLength>,
			prize_token_id: AssetIdOf<T>,
			prize_token_amount: AssetBalanceOf<T>,
			prize_token_winner: u32,
			token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
			contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength>,
			description: BoundedVec<u8, T::MaxDescriptionLength>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::validate_contest_new(
				who.clone(),
				contest_id.clone(),
				title.clone(),
				prize_token_id.clone(),
				prize_token_amount.clone(),
				prize_token_winner.clone(),
				token_symbol.clone(),
				contest_end_date.clone(),
				description.clone()
			)?;

			let contest = Contests::<T> {
				contest_id: contest_id.clone(),
				title: title.clone(),
				user_address: who.clone(),
				prize_token_id: prize_token_id.clone(),
				prize_token_winner: prize_token_winner.clone(),
				token_symbol: token_symbol.clone(),
				statcode: true,
				contest_end_date: contest_end_date.clone(),
				description: description.clone()
			};
			
			T::Assets::transfer(
				prize_token_id, 
				&who, 
				&T::PalletId::get().into_account_truncating(),
				prize_token_amount,
				false	
			)?;

			ContestsMap::<T>::insert(contest_id, contest);

			Self::deposit_event(Event::<T>::ContestCreted { who, contest_id, title } );

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn update_contest(
			origin: OriginFor<T>,
			contest_id: u32,

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

impl<T: Config> Pallet<T> {
	fn validate_contest_new(
		who: T::AccountId,
		contest_id: u32,
		title: BoundedVec<u8, T::MaxTitleLength>,
		prize_token_id: AssetIdOf<T>,
		prize_token_amount: AssetBalanceOf<T>,
		prize_token_winner: u32,
		token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
		contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength>,
		description: BoundedVec<u8, T::MaxDescriptionLength>
	) -> DispatchResult {

		ensure!(ContestsMap::<T>::contains_key(contest_id.clone()), Error::<T>::ContestIdAlreadyInUse);
		ensure!(T::Assets::asset_exists(prize_token_id.clone()), Error::<T>::AssetDontExist);
		ensure!(prize_token_winner < T::MinTokenWinner::get(), Error::<T>::PrizeTokenWinnerTooSmall);
		ensure!(title.len() as u32> T::MinTitleLength::get(), Error::<T>::TitleTooSmall);
		ensure!(token_symbol.len() as u32 > T::MinTokenSymbolLength::get(), Error::<T>::TokenSymbolTooSmall);
		ensure!(description.len() as u32 > T::MinDescriptionLength::get(), Error::<T>::DescriptionTooSmall);
		ensure!(prize_token_amount < T::MinTokenAmount::get().into(), Error::<T>::TokenAmountTooSmall);
		ensure!(T::Assets::balance(prize_token_id, &who) >= T::MinTokenAmount::get().into(), Error::<T>::AssetBalanceInsufficient);
		
		// Need to finish the contest_end_date validation yet

		Ok(())
	}
}