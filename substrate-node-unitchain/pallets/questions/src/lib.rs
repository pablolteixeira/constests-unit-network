#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::fungibles};
	use frame_system::pallet_prelude::*;

	pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
		<T as frame_system::Config>::AccountId,
	>>::AssetId;

	pub type BalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Assets Pallet.
		type Fungibles: fungibles::Inspect<Self::AccountId>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::InspectMetadata<Self::AccountId>
			+ fungibles::Transfer<Self::AccountId>;

		/// Max length of string
		#[pallet::constant]
		type MaxLength: Get<u32>;

		/// Minimum amount of asset to ask question
		#[pallet::constant]
		type MinAmountToAsk: Get<BalanceOf<Self>>;
	}

	// Details of each question.
	#[pallet::storage]
	#[pallet::getter(fn questions_data)]
	pub type QuestionsData<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128,
		types::Question<T::AccountId, AssetIdOf<T>, T::MaxLength>,
	>;

	// Details of answer in each question.
	#[pallet::storage]
	#[pallet::getter(fn answers_data)]
	pub type AnswersData<T: Config> =
		StorageMap<_, Blake2_128Concat, u128, types::Answer<T::AccountId, T::MaxLength>>;

	// Mapping for check account vote each question.
	// AccountId -> QuestionId -> boolean
	#[pallet::storage]
	#[pallet::getter(fn user_vote_data)]
	pub type UserVoteData<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		u128,
		bool,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn question_id)]
	/// QuestionId for create next question.
	pub type QuestionId<T: Config> = StorageValue<_, u128, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New question was added.
		AskedQuestion { who: T::AccountId, question_id: u128, title: BoundedVec<u8, T::MaxLength> },
		/// New answer was added.
		RepliedQuestion {
			who: T::AccountId,
			question_id: u128,
			answer: BoundedVec<u8, T::MaxLength>,
		},
		/// New vote was added to answer.
		VotedAnswer { who: T::AccountId, question_id: u128, votes: u128 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Question not found.
		QuestionNotFound,
		/// Not provides question title.
		NoQuestionTitle,
		/// Not provides asset_id.
		NoAssetId,
		/// Not provides description.
		NoDescription,
		/// Not provides video link.
		NoVideoLink,
		/// Not provides answer.
		NoAnswer,
		/// The question ID is unknown.
		UnknownQuestionId,
		/// The asset ID is unknown.
		UnknownAssetId,
		/// The answer not provide on question.
		NoAnserOnQuestion,
		/// Insuffienct balance to create question.
		InsuffienctBalance,
		/// Answer is reply to question already.
		QuestionAlreadyReplied,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		/// The origin can ask question and provides the detail.
		///
		/// Parameters:
		/// - `asset_id`: The id of asset that specific in question.
		/// - `title`: The title of qeustion.
		/// - `description`: The description of qeustion.
		/// - `video_link`: The video_link of question.
		///
		/// Emits `AskedQuestion` event when successful.
		pub fn ask_question(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			title: BoundedVec<u8, T::MaxLength>,
			description: BoundedVec<u8, T::MaxLength>,
			video_link: BoundedVec<u8, T::MaxLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let who_balance = <<T as Config>::Fungibles as fungibles::Inspect<
				<T as frame_system::Config>::AccountId,
			>>::balance(asset_id, &who);

			ensure!(who_balance >= T::MinAmountToAsk::get(), Error::<T>::InsuffienctBalance);

			let maybe_asset_name = <<T as Config>::Fungibles as fungibles::InspectMetadata<
				<T as frame_system::Config>::AccountId,
			>>::name(&asset_id);
			let asset_name = BoundedVec::try_from(maybe_asset_name.to_vec()).unwrap();

			Self::ensure_ask_question_parameter(
				title.clone(),
				description.clone(),
				video_link.clone(),
			)?;

			let id = Self::question_id();
			let question_detail = types::Question {
				owner: who.clone(),
				title: title.clone(),
				asset_id,
				asset_name,
				description,
				video_link,
			};

			QuestionsData::<T>::insert(id, question_detail);
			QuestionId::<T>::set(id.saturating_add(1));

			Self::deposit_event(Event::<T>::AskedQuestion { who, question_id: id, title });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		/// The origin can reply question and provides the answer.
		///
		/// Parameters:
		/// - `question_id`: The id of question to reply.
		/// - `answer`: The answer detail of question.
		///
		/// Emits `RepliedQuestion` event when successful.
		pub fn reply_question(
			origin: OriginFor<T>,
			question_id: u128,
			answer: BoundedVec<u8, T::MaxLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!answer.is_empty(), Error::<T>::NoAnswer);
			ensure!(QuestionsData::<T>::contains_key(question_id), Error::<T>::UnknownQuestionId);

			let question_data = QuestionsData::<T>::get(question_id).unwrap();

			let who_balance = <<T as Config>::Fungibles as fungibles::Inspect<
				<T as frame_system::Config>::AccountId,
			>>::balance(question_data.asset_id, &who);

			ensure!(who_balance >= T::MinAmountToAsk::get(), Error::<T>::InsuffienctBalance);
			ensure!(
				!AnswersData::<T>::contains_key(question_id),
				Error::<T>::QuestionAlreadyReplied
			);

			AnswersData::<T>::insert(
				question_id,
				types::Answer { owner: who.clone(), answer: answer.clone(), votes: 0 },
			);

			Self::deposit_event(Event::<T>::RepliedQuestion { who, question_id, answer });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		/// The origin can vote for answer.
		///
		/// Parameters:
		/// - `question_id`: The id of question to reply.
		///
		/// Emits `VotedAnswer` event when successful.
		pub fn vote(origin: OriginFor<T>, question_id: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(QuestionsData::<T>::contains_key(question_id), Error::<T>::UnknownQuestionId);
			ensure!(AnswersData::<T>::contains_key(question_id), Error::<T>::NoAnserOnQuestion);

			let user_vote = UserVoteData::<T>::get(who.clone(), question_id);
			let new_user_vote = !user_vote;

			UserVoteData::<T>::insert(who.clone(), question_id, new_user_vote);

			let mut total_votes = 0;

			AnswersData::<T>::try_mutate(question_id, |maybe_answer| -> DispatchResult {
				let answer = maybe_answer.as_mut().ok_or(Error::<T>::UnknownQuestionId)?;

				if new_user_vote {
					answer.votes = answer.votes.saturating_add(1);
				} else {
					answer.votes = answer.votes.saturating_sub(1);
				}

				total_votes = answer.votes;

				Ok(())
			})?;

			Self::deposit_event(Event::<T>::VotedAnswer { who, question_id, votes: total_votes });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// This is not a call, so it cannot be called directly by real-world users.
		// Still it has to be generic over the runtime types, and that's why we implement it on
		// Pallet rather than just defining a local function.
		fn ensure_ask_question_parameter(
			title: BoundedVec<u8, T::MaxLength>,
			description: BoundedVec<u8, T::MaxLength>,
			video_link: BoundedVec<u8, T::MaxLength>,
		) -> Result<(), Error<T>> {
			ensure!(!title.is_empty(), Error::<T>::NoQuestionTitle);
			ensure!(!description.is_empty(), Error::<T>::NoDescription);
			ensure!(!video_link.is_empty(), Error::<T>::NoVideoLink);

			Ok(())
		}
	}
}
