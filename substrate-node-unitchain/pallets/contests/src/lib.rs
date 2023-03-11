#[cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::{
	sp_runtime::{
		traits::{
			Zero,
			AccountIdConversion,
			CheckedDiv,
			CheckedSub
		}, 
		FixedPointOperand},
	PalletId,
	traits::tokens::{
		Balance,
		fungibles::{
			Transfer, 
			Inspect
		}},
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
	pub struct Contest<T: Config> {
		pub contest_id: u32,
		pub title: BoundedVec<u8, T::MaxTitleLength>,
		pub user_address: T::AccountId,
		pub prize_token_id: AssetIdOf<T>,
		pub prize_token_amount: AssetBalanceOf<T>,
		pub prize_token_winner: u32,
		pub token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
		// statcode states -> true: open; false: closed.
		pub statcode: bool,
		pub contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength>,
		pub description: BoundedVec<u8, T::MaxDescriptionLength>
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ContestEntry<T: Config> {
		pub user_address: T::AccountId,
		pub contest_id: u32,
		pub entry_id: u32,
		pub winner: bool,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_contests)]
	// contest_id -> Contest
	pub type ContestsMap<T> = StorageMap<_, Blake2_128Concat, u32, Contest<T>>; 

	#[pallet::storage]
	#[pallet::getter(fn ger_entries)]
	// entry_id -> ContestEntry
	pub type EntriesMap<T> = StorageMap<_, Blake2_128Concat, u32, ContestEntry<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ContestCreted { who: T::AccountId, contest_id: u32, title: BoundedVec<u8, T::MaxTitleLength> },
		ContestUpdated { who: T::AccountId, contest_id: u32, title: BoundedVec<u8, T::MaxTitleLength>, 
				description: BoundedVec<u8, T::MaxDescriptionLength>, contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength> },
		EntryCreated { who: T::AccountId, contest_id: u32, entry_id: u32 },
		ContestWinnerAssigned { contest_id: u32, winner: T::AccountId, prize: AssetBalanceOf<T> },
		ContestClosed { who: T::AccountId, contest_id: u32 },
	}

	#[pallet::error]
	pub enum Error<T> {
		ContestIdAlreadyInUse,
		ContestIdDontExist,
		ContestAlreadyClosed,
		EntryIdAlreadyExist,
		EntryIdDontExist,
		AssetDontExist,
		TitleTooSmall,
		TokenSymbolTooSmall,
		DescriptionTooSmall,
		PrizeTokenWinnerTooSmall,
		AssetBalanceInsufficient,
		TokenAmountTooSmall,
		OnlyOwnerCanChange,
		OnlyOwnerCanAssignContestWinner,
		OnlyOwnerCanCloseContest,
		DivisionError
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

			let contest = Contest::<T> {
				contest_id: contest_id.clone(),
				title: title.clone(),
				user_address: who.clone(),
				prize_token_id: prize_token_id.clone(),
				prize_token_amount: prize_token_amount.clone(),
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
			title: BoundedVec<u8, T::MaxTitleLength>,
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::validate_update_contest(
				who.clone(),
				contest_id.clone(),
				title.clone(),
				description.clone(),
				contest_end_date.clone()
			)?;
			
			// Unwrap used because there is a function "validate_update_contest" above testing that the element exist with contest_id key 
			let mut contest = ContestsMap::<T>::get(contest_id.clone()).unwrap();

			contest.title = title.clone();
			contest.description = description.clone();
			contest.contest_end_date = contest_end_date.clone();

			ContestsMap::<T>::insert(contest_id, contest);

			Self::deposit_event(Event::<T>::ContestUpdated { who, contest_id, title, description, contest_end_date });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn create_contest_entry(
			origin: OriginFor<T>,
			contest_id: u32,
			entry_id: u32
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::validate_create_contest_entry(
				contest_id.clone(),
				entry_id.clone()
			)?;

			let entry_contest = ContestEntry::<T> {
				user_address: who.clone(),
				contest_id: contest_id.clone(),
				entry_id: entry_id.clone(),
				winner: false
			};

			EntriesMap::<T>::insert(entry_id.clone(), entry_contest);

			Self::deposit_event(Event::<T>::EntryCreated { who, contest_id, entry_id });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn assign_contest_winner(
			origin: OriginFor<T>,
			entry_id: u32
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut contest = Self::validate_assign_contest_winner(
				who.clone(),
				entry_id.clone()
			)?;

			let prize = match contest.prize_token_amount.checked_div(&contest.prize_token_winner.into()) {
					Some(value) => Ok(value),
					None => Err(Error::<T>::DivisionError)
			}?;

			let mut contest_entry = EntriesMap::<T>::get(entry_id).unwrap();

			T::Assets::transfer(
				contest.prize_token_id, 
				&T::PalletId::get().into_account_truncating(), 
				&contest_entry.user_address,
				prize.clone(),
				false	
			)?;

			contest.prize_token_winner = contest.prize_token_winner.checked_sub(1).unwrap_or(0);
			contest.prize_token_amount = contest.prize_token_amount.checked_sub(&prize).unwrap_or(AssetBalanceOf::<T>::zero());
			contest_entry.winner = true;

			if contest.prize_token_winner == 0 {
				contest.statcode = false;
			}

			let contest_id = contest_entry.contest_id.clone();
			let winner = contest_entry.user_address.clone();

			ContestsMap::<T>::insert(contest.contest_id, contest);
			EntriesMap::<T>::insert(entry_id, contest_entry);

			Self::deposit_event(Event::<T>::ContestWinnerAssigned { contest_id ,winner, prize });

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn close_contract(
			origin: OriginFor<T>,
			contest_id: u32
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut contest = Self::validate_close_contract(
				who.clone(),
				contest_id.clone()
			)?;

			T::Assets::transfer(
				contest.prize_token_id.clone(),
				&T::PalletId::get().into_account_truncating(),
				&who,
				contest.prize_token_amount.clone(),
				false
			)?;

			contest.statcode = false;
			contest.prize_token_amount = AssetBalanceOf::<T>::zero();

			let contest_id = contest.contest_id.clone();

			ContestsMap::<T>::insert(contest_id.clone(), contest);

			Self::deposit_event(Event::<T>::ContestClosed { who, contest_id });

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

		ensure!(!ContestsMap::<T>::contains_key(contest_id), Error::<T>::ContestIdAlreadyInUse);
		ensure!(T::Assets::asset_exists(prize_token_id.clone()), Error::<T>::AssetDontExist);
		ensure!(prize_token_winner >= T::MinTokenWinner::get(), Error::<T>::PrizeTokenWinnerTooSmall);
		ensure!(title.len() as u32 >= T::MinTitleLength::get(), Error::<T>::TitleTooSmall);
		ensure!(token_symbol.len() as u32 >= T::MinTokenSymbolLength::get(), Error::<T>::TokenSymbolTooSmall);
		ensure!(description.len() as u32 >= T::MinDescriptionLength::get(), Error::<T>::DescriptionTooSmall);
		ensure!(prize_token_amount >= T::MinTokenAmount::get().into(), Error::<T>::TokenAmountTooSmall);
		ensure!(T::Assets::balance(prize_token_id, &who) >= T::MinTokenAmount::get().into(), Error::<T>::AssetBalanceInsufficient);
		
		// Need to finish the contest_end_date validation yet

		Ok(())
	}

	fn validate_update_contest(
		who: T::AccountId,
		contest_id: u32,
		title: BoundedVec<u8, T::MaxTitleLength>,
		description: BoundedVec<u8, T::MaxDescriptionLength>,
		contest_end_date: BoundedVec<u8, T::MaxContestEndDateLength>		
	) -> DispatchResult {

		ensure!(ContestsMap::<T>::contains_key(contest_id.clone()), Error::<T>::ContestIdDontExist);
		ensure!(title.len() as u32 >= T::MinTitleLength::get(), Error::<T>::TitleTooSmall);
		ensure!(description.len() as u32 >= T::MinDescriptionLength::get(), Error::<T>::DescriptionTooSmall);

		// Unwrap used because there is a ensure! above testing that the element exist with contest_id key 
		let contest = ContestsMap::<T>::get(contest_id.clone()).unwrap();

		ensure!(contest.user_address == who, Error::<T>::OnlyOwnerCanChange);

		// Need to finish the contest_end_date verification.

		Ok(())
	}

	fn validate_create_contest_entry(
		contest_id: u32,
		entry_id: u32
	) -> DispatchResult {

		ensure!(!EntriesMap::<T>::contains_key(entry_id), Error::<T>::EntryIdAlreadyExist);		
		ensure!(ContestsMap::<T>::contains_key(contest_id), Error::<T>::ContestIdDontExist);

		Ok(())
	}

	fn validate_assign_contest_winner(
		who: T::AccountId,
		entry_id: u32
	) -> Result<Contest<T>, DispatchError> {

		ensure!(EntriesMap::<T>::contains_key(entry_id.clone()), Error::<T>::EntryIdDontExist);

		// Unwrap used because there is a ensure! above testing that the element exist with contest_id key 
		let contest_entry = EntriesMap::<T>::get(entry_id).unwrap();

		ensure!(ContestsMap::<T>::contains_key(contest_entry.contest_id), Error::<T>::ContestIdDontExist);
		
		// Unwrap used because there is a ensure! above testing that the element exist with contest_id key 
		let contest = ContestsMap::get(contest_entry.contest_id).unwrap();
		
		ensure!(contest.statcode, Error::<T>::ContestAlreadyClosed);
		ensure!(who == contest.user_address, Error::<T>::OnlyOwnerCanAssignContestWinner);
		
		Ok(contest)
	} 

	fn validate_close_contract(
		who: T::AccountId,
		contest_id: u32
	) -> Result<Contest<T> ,DispatchError> {

		ensure!(ContestsMap::<T>::contains_key(contest_id.clone()), Error::<T>::ContestIdDontExist);

		let contest = ContestsMap::<T>::get(contest_id.clone()).unwrap();

		ensure!(contest.statcode == true, Error::<T>::ContestAlreadyClosed);
		ensure!(contest.user_address == who, Error::<T>::OnlyOwnerCanCloseContest);

		Ok(contest)
	}
}