#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

use frame_support::pallet_prelude::*;

use scale_info::prelude::vec;
use scale_info::prelude::vec::Vec;

use frame_support::{
	traits::{
		schedule::{DispatchTime, Named as ScheduleNamed},
		tokens::fungibles::{Balanced, Inspect, Transfer},
		Currency, LockIdentifier, LockableCurrency, ReservableCurrency,
	},
};

use sp_runtime::SaturatedConversion;

use codec::{Encode, Decode, HasCompact};

use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, Dispatchable, Saturating, Zero,
	},
	DispatchError};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

const POLLS_ID: LockIdentifier = *b"UnitPoll";

/// Balance type alias.
pub(crate) type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
/// Asset id type alias.
pub(crate) type AssetIdOf<T> =
	<<T as Config>::Fungibles as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
/// Block number type alias.
pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
/// Poll details type alias.
pub(crate) type PollTypeOf<T> = PollDetails<
	BalanceOf<T>,
	<T as frame_system::Config>::AccountId, 
	AssetIdOf<T>, 
	BlockNumberOf<T>
	>;

pub type IpfsCid = Vec<u8>;

/// Details of a poll.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PollDetails<Balance, AccountId, AssetId, BlockNumber> {
	/// Account who created this poll.
	pub created_by: AccountId,
	/// IPFS CID with all contextual information regarding this poll.
	pub ipfs_cid: IpfsCid,
	/// The number of poll options.
	pub options_count: u8,
	/// Info regrading stake on poll options.
	pub votes: Votes,
	/// Currency of the poll.
	pub currency: PollCurrency<AssetId>,
	/// Status of the poll.
	pub status: PollStatus<BlockNumber>,
	// Min balance to be able to vote on a poll.
	pub min_balance: Balance,
}

impl<Balance: AtLeast32BitUnsigned + Copy, AccountId: Clone + Eq, AssetId, BlockNumber>
	PollDetails<Balance, AccountId, AssetId, BlockNumber>
{
	/// Creates a new PollDetails with Ongoing status and empty Tally.
	pub fn new(
		created_by: AccountId,
		ipfs_cid: IpfsCid,
		options_count: u8,
		currency: PollCurrency<AssetId>,
		start: BlockNumber,
		end: BlockNumber,
		min_balance: Balance,
	) -> Self {
		Self {
			created_by,
			ipfs_cid,
			options_count,
			votes: Votes::new(options_count),
			currency,
			status: PollStatus::Ongoing { start, end },
			min_balance,
		}
	}

}

/// A vote for a poll.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Votes(pub Vec<u128>);

impl Votes {
	pub fn new(options_count: u8) -> Self {
		Self(vec![0; options_count as usize])
	}

	// returns the winning option, if any, if there is a tie return None
	pub fn winning_option(&self) -> Option<u8> {
		
		let mut max = 0;
		let mut max_index = 0;
		let mut tie = false;
		for (index, value) in self.0.iter().enumerate() {
			if *value > max {
				max = *value;
				max_index = index;
				tie = false;
			} else if *value == max {
				tie = true;
			}
		}
		if tie {
			None
		} else {
			Some(max_index as u8)
		}
	}

}

/// Status of a poll, present, cancelled, or past.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum PollStatus<BlockNumber> {
	/// Poll is happening, the args are the block number at which it will start and end.
	Ongoing {
		/// When voting on this poll will begin.
		start: BlockNumber,
		/// When voting on this poll will end.
		end: BlockNumber,
	},
	/// Poll has been cancelled at a given block.
	Cancelled(BlockNumber),
	/// Poll finished at `end`, and has `winning_option`.
	Finished {
		/// What poll option has won.
		winning_option: Option<u8>,
		/// When voting on this poll ended.
		end: BlockNumber,
	},
	/// State for situations where some condition of succes was not met
	Failed(BlockNumber),
}

impl<BlockNumber> PollStatus<BlockNumber> {
	pub fn is_ongoing(&self) -> bool {
		match self {
			PollStatus::Ongoing { .. } => true,
			_ => false,
		}
	}
}

#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum PollCurrency<AssetId> {
	/// AssetId from the Assets Pallet.
	Asset(AssetId),
	/// Native Balances currency of the network.
	Native,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The fungibles instance used for transfers in assets.
		/// The Balance type should be the same as in balances pallet.
		type Fungibles: Inspect<Self::AccountId, Balance = BalanceOf<Self>>
			+ Transfer<Self::AccountId>
			+ Balanced<Self::AccountId>;

		/// Currency type for this pallet.
		/// The Balance type should be the same as in assets pallet.
		type Currency: ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		/// Identifier and index for polls.
		type PollIndex: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// Overarching type of all pallets origins.
		type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;

		/// The overarching call type for Scheduler.
		type PollCall: Parameter + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin> + From<Call<Self>>;
			
		/// The Scheduler.
		type Scheduler: ScheduleNamed<Self::BlockNumber, Self::PollCall, Self::PalletsOrigin>;

	}

	/// The number of polls that have been made so far.
	#[pallet::storage]
	#[pallet::getter(fn poll_count)]
	pub type PollCount<T: Config> = StorageValue<_, T::PollIndex, ValueQuery>;

	/// Details of polls.
	#[pallet::storage]
	#[pallet::getter(fn poll_details_of)]
	pub(super) type PollDetailsOf<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		PollCurrency<AssetIdOf<T>>,
		Blake2_128Concat,
		T::PollIndex,
		PollDetails<BalanceOf<T>, T::AccountId, AssetIdOf<T>, BlockNumberOf<T>>,
	>;

	/// All votes for a particular voter. So to avoid voting twice.
	#[pallet::storage]
	#[pallet::getter(fn voting_of)]
	pub type VotingOf<T: Config> =
		StorageMap<_, Blake2_128Concat, (T::AccountId, T::PollIndex), u8>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A poll was created.
		PollCreated { currency: PollCurrency<AssetIdOf<T>>, poll_id: T::PollIndex, cid: IpfsCid, creator: T::AccountId },
		/// An account has voted in a poll.
		Voted { voter: T::AccountId, currency: PollCurrency<AssetIdOf<T>>, poll_id: T::PollIndex, vote: u8 },
		/// A poll was finished.
		Finished { currency: PollCurrency<AssetIdOf<T>>, poll_id: T::PollIndex },
		/// A poll was updated
		PollUpdated { currency: PollCurrency<AssetIdOf<T>>, poll_id: T::PollIndex, cid: IpfsCid, creator: T::AccountId },
		/// A poll was cancelled
		Cancelled { currency: PollCurrency<AssetIdOf<T>>, poll_id: T::PollIndex },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Invalid poll_id given for a poll.
		PollInvalid,
		/// The poll has already finished.
		PollAlreadyFinished,
		/// The poll has already finished.
		PollNotFound,
		/// 
		UnexpectedBehavior,
		/// Invalid poll details given.
		InvalidPollDetails,
		/// Invalid poll period given.
		InvalidPollPeriod,
		/// The asset Id should be valid.
		InvalidPollCurrency,
		/// The option is not valid.
		InvalidPollVote,
		/// To vote, the poll should be in progress.
		PollNotStarted,
		/// User needs a minimal balance to be able to vote.
		InsufficientFunds,
		/// Can´t modify a poll that already started.
		PollAlreadyStarted,
		/// Only poll creator can update it
		NotPollCreator,
		/// A user can only vote once
		AlreadyVoted,
		/// Poll options should be more than ohe
		InvalidPollOptions,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Create a poll
		/// 
		/// The dispatch origin of this must be _Signed_.
		/// 
		///	- `ipfs_cid`: The IPFS CID of the poll.
		///	- `options_count`: The number of poll options.
		///	- `currency`: Currency of the poll.
		///	- `start`: When voting on this poll will begin.
		///	- `end`:  When voting on this poll will end.
		///	- `min_balance`: Minimum balance required to vote. 
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn create_poll(
			origin: OriginFor<T>,
			ipfs_cid: IpfsCid,
			options_count: u8,
			currency: PollCurrency<AssetIdOf<T>>,
			start: BlockNumberOf<T>,
			end: BlockNumberOf<T>,
			min_balance: BalanceOf<T>,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;

			// Create poll details struct.
			let poll = PollDetails::new(
				who.clone(),
				ipfs_cid.clone(),
				options_count,
				currency.clone(),
				start,
				end,
				min_balance,
			);

			// Call inner function.
			let poll_id = Self::try_create_poll(poll)?;

			// Emit an event.
			Self::deposit_event(Event::PollCreated { currency, poll_id, cid: ipfs_cid, creator: who });

			Ok(())
		}

		/// Vote in a poll.
		///
		/// The dispatch origin of this call must be _Signed_.
		///
		///	- `poll_currency`: Currency of the poll.
		/// - `poll_id`: The index of the poll to vote for.
		/// - `vote`: The index of the option to vote for.
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn vote(
			origin: OriginFor<T>,
			poll_currency: PollCurrency<AssetIdOf<T>>,
			poll_id: T::PollIndex,
			vote: u8,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;
			// Call inner function.
			Self::try_vote(&who, poll_currency, poll_id, vote.clone())?;
			// Emit an event.
			Self::deposit_event(Event::<T>::Voted { voter: who, currency: poll_currency,  poll_id: poll_id, vote: vote });

			Ok(())
		}

		/// Enact poll end.
		///
		/// The dispatch origin of this call must be _ROOT_.
		///
		///	- `poll_currency`: Currency of the poll.
		/// - `poll_id`: The index of the poll to enact end.
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn enact_poll_end(
			origin: OriginFor<T>,
			poll_currency: PollCurrency<AssetIdOf<T>>, 
			poll_id: T::PollIndex
		) -> DispatchResult {

			ensure_root(origin)?;

			Self::do_enact_poll_end(poll_currency, poll_id)?;

			// Emit an event.
			Self::deposit_event(Event::Finished { currency: poll_currency, poll_id });

			Ok(())
		}

		/// Cancel a poll in emergency.
		///
		/// The dispatch origin of this call must be _Signed_.
		///
		///	- `poll_currency`: Currency of the poll.
		/// - `poll_id`: The index of the poll to enact end.
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn emergency_cancel(
			origin: OriginFor<T>,
			poll_currency: PollCurrency<AssetIdOf<T>>, 
			poll_id: T::PollIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Call inner function.
			Self::try_emergency_cancel(&who, poll_currency, poll_id)?;
			// Emit an event.
			Self::deposit_event(Event::<T>::Cancelled { currency: poll_currency, poll_id });
			Ok(())
		}

		/// Cancel a poll in emergency.
		///
		/// Only poll creator can call this function.
		/// 
		/// The dispatch origin of this call must be _Signed_.
		///
		/// - `poll_id`: The index of the poll to enact end.
		/// - `ipfs_cid`: The IPFS CID of the poll.
		///	- `options_count`: The number of poll options.
		///	- `currency`: Currency of the poll.
		///	- `start`: When voting on this poll will begin.
		///	- `end`: When voting on this poll will end.
		///	- `min_balance`: Minimum balance required to vote.
		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn update_poll(
			origin: OriginFor<T>,
			poll_id: T::PollIndex,
			ipfs_cid: IpfsCid,
			options_count: u8,
			currency: PollCurrency<AssetIdOf<T>>,
			start: BlockNumberOf<T>,
			end: BlockNumberOf<T>,
			min_balance: BalanceOf<T>,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;

			// Call inner function.
			Self::try_update_poll(
				&who,
				currency.clone(),
				poll_id.clone(),
				ipfs_cid.clone(),
				options_count,
				start,
				end,
				min_balance,
			)?;

			// Emit an event.
			Self::deposit_event(Event::PollUpdated { currency, poll_id, cid: ipfs_cid, creator: who });

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	
	/// Actually create a poll.
	fn try_create_poll(poll: PollTypeOf<T>) -> Result<T::PollIndex, DispatchError> {

		// Validate poll options is a number grater than 1
		ensure!(poll.options_count > 1, Error::<T>::InvalidPollOptions);

		let (start, end) = match poll.status {
			PollStatus::Ongoing { start, end } => (start, end),
			_ => return Err(Error::<T>::InvalidPollDetails.into()),
		};

		// Ensure start and end blocks are valid.
		let now = <frame_system::Pallet<T>>::block_number();
		ensure!(start >= now && end > now && end > start, Error::<T>::InvalidPollPeriod);

		// Ensure currency asset exists.
		if let PollCurrency::Asset(asset_id) = poll.currency {
			let total_issuance = <T::Fungibles as Inspect<T::AccountId>>::total_issuance(asset_id);
			ensure!(total_issuance > BalanceOf::<T>::zero(), Error::<T>::InvalidPollCurrency);
		}

		// Get next poll_id from storage.
		let mut poll_id = PollCount::<T>::get();

		poll_id.saturating_inc();

		PollDetailsOf::<T>::insert(poll.currency, poll_id, poll.clone());

		// Updates poll count.
		PollCount::<T>::put(poll_id);
		
		// Actually schedule end of the poll.
		if T::Scheduler::schedule_named(
			(POLLS_ID, poll.currency, poll_id).encode(),
			DispatchTime::At(end),
			None,
			63,
			frame_system::RawOrigin::Root.into(),
			Call::enact_poll_end { poll_currency: poll.currency, poll_id }.into(),
		)
		.is_err()

		{
			frame_support::print("LOGIC ERROR: try_create_poll/schedule_named failed");
		}

		Ok(poll_id)
	}


	// Update details of a poll only it is not ongoing yet.
	fn try_update_poll(
		who: &T::AccountId,
		poll_currency: PollCurrency<AssetIdOf<T>>,
		poll_id: T::PollIndex,
		ipfs_cid: IpfsCid,
		options_count: u8,
		start: BlockNumberOf<T>,
		end: BlockNumberOf<T>,
		min_balance: BalanceOf<T>,
	) -> DispatchResult {

		// Get poll details.
		let mut poll = PollDetailsOf::<T>::get(poll_currency, poll_id).ok_or(Error::<T>::PollNotFound)?;

		// Ensure poll did not started yet
		if let PollStatus::Ongoing { start, .. } = poll.status {
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now < start, Error::<T>::PollAlreadyStarted);
		}

		// Ensure poll creator is the same as who.
		ensure!(poll.created_by == *who, Error::<T>::NotPollCreator);

		// Update poll details.
		poll.ipfs_cid = ipfs_cid.clone();
		poll.options_count = options_count;
		poll.status = PollStatus::Ongoing { start, end };
		poll.min_balance = min_balance;

		// Update poll details in storage.
		PollDetailsOf::<T>::insert(poll_currency, poll_id, poll.clone());

		// Emit an event.
		Self::deposit_event(Event::PollUpdated { currency: poll_currency, poll_id, cid: ipfs_cid, creator: who.clone() });

		Ok(())
	}

	// 
	fn check_balance(
	 	who: &T::AccountId,
	 	currency: PollCurrency<AssetIdOf<T>>,
	 	poll: &PollTypeOf<T>,
	) -> bool {

		// Get min balance from poll.
		let min_balance = poll.min_balance;

	 	match currency {
	 		
	 		PollCurrency::Native => min_balance < T::Currency::free_balance(who).into(),
	 		PollCurrency::Asset(asset_id) =>
	 		min_balance < <T::Fungibles as Inspect<T::AccountId>>::balance(asset_id, who).into(),
	 	}
	}

	fn vote_weight(
		who: &T::AccountId,
		currency: PollCurrency<AssetIdOf<T>>,
   	) -> u128 {

		match currency {
			
			PollCurrency::Native => Self::balance_to_u128_saturated(T::Currency::free_balance(who)),
			PollCurrency::Asset(asset_id) =>
			Self::balance_to_u128_saturated(<T::Fungibles as Inspect<T::AccountId>>::balance(asset_id, who)),
		}
   	}

	fn try_vote(
		who: &T::AccountId,
		poll_currency: PollCurrency<AssetIdOf<T>>,
		poll_id: T::PollIndex,
		votes: u8,
	) -> DispatchResult {
		let mut poll = Self::poll_status(poll_currency, poll_id)?;

		// Check if Votes has valid number of options.
		ensure!(votes <= poll.options_count, Error::<T>::InvalidPollVote);

		// Ensure start and end blocks are valid.
		if let PollStatus::Ongoing { start, .. } = poll.status {
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(start <= now, Error::<T>::PollNotStarted);
		}

		// check account has enough balance to vote
		ensure!(
			Self::check_balance(who, poll.currency, &poll),
			Error::<T>::InsufficientFunds,
		);

		// check account has not already voted
		ensure!(
		 	!VotingOf::<T>::contains_key((who, poll_id)),
		 	Error::<T>::AlreadyVoted,
		);

		// Get vote weight
		let vote_weight = Self::vote_weight(who, poll.currency);

		// Add vote weight to chosen option
		poll.votes.0[votes as usize] += vote_weight;


		// Update poll in storage.
		VotingOf::<T>::insert((who, poll_id), votes);
		PollDetailsOf::<T>::insert(poll.currency, poll_id, poll);
		Ok(())
	}


	fn poll_status(
		poll_currency: PollCurrency<AssetIdOf<T>>,
		poll_id: T::PollIndex,
	) -> Result<PollDetails<BalanceOf<T>, T::AccountId, AssetIdOf<T>, T::BlockNumber>, DispatchError>
	{
		let poll = PollDetailsOf::<T>::get(poll_currency, poll_id).ok_or(Error::<T>::PollInvalid)?;
		match poll.status.is_ongoing() {
			true => Ok(poll),
			_ => Err(Error::<T>::PollAlreadyFinished.into()),
		}
	}

	/// Finish the poll
	fn do_enact_poll_end(
		poll_currency: PollCurrency<AssetIdOf<T>>,
		poll_id: T::PollIndex,
	) -> DispatchResult {
		let mut poll = PollDetailsOf::<T>::get(poll_currency, poll_id).ok_or(Error::<T>::PollNotFound)?;
		// Shouldn't be any other status than Ongoing, but better be safe.
		let end = match poll.status {
			PollStatus::Ongoing { end, .. } => end,
			_ => return Err(Error::<T>::PollAlreadyFinished.into()),
		};
		
		let winning_option = poll.votes.winning_option(); // .ok_or(Error::<T>::UnexpectedBehavior)?
		
		poll.status = PollStatus::Finished { winning_option: winning_option, end };
		
		// Update poll in storage.
		PollDetailsOf::<T>::insert(poll_currency, poll_id, poll);
		Ok(())
	}


	// Emergency cancel
	fn try_emergency_cancel(who: &T::AccountId, poll_currency: PollCurrency<AssetIdOf<T>>, poll_id: T::PollIndex) -> DispatchResult {
		let mut poll = PollDetailsOf::<T>::get(poll_currency, poll_id).ok_or(Error::<T>::PollNotFound)?;
		// Check if origin is entitled to cancel the poll.
		ensure!(poll.created_by.eq(who), Error::<T>::NotPollCreator);
		// Cancel dispatch.
		T::Scheduler::cancel_named((POLLS_ID, poll.currency, poll_id).encode())
			.map_err(|_| Error::<T>::UnexpectedBehavior)?;
		// Set status to Cancelled and update polls storage.
		let now = <frame_system::Pallet<T>>::block_number();
		poll.status = PollStatus::Cancelled(now);
		PollDetailsOf::<T>::insert(poll_currency, poll_id, poll);
		Ok(())
	}

	// Note the warning above about saturated conversions
	pub fn balance_to_u128_saturated(input: <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance) -> u128 {
    	input.saturated_into::<u128>()
	}


	
}
