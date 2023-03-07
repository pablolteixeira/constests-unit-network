#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{SortedMembers, Time},
	Parameter,
};
use sp_std::{prelude::*, vec};

pub use crate::combine_data::{CombineData, DefaultCombineData};

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

mod combine_data;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	// use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
	pub type TimestampedValueOf<T> = TimestampedValue<<T as Config>::OracleValue, MomentOf<T>>;

	#[derive(
		Encode,
		Decode,
		RuntimeDebug,
		Eq,
		PartialEq,
		Clone,
		Copy,
		Ord,
		PartialOrd,
		TypeInfo,
		MaxEncodedLen,
	)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct TimestampedValue<Value, Moment> {
		pub value: Value,
		pub timestamp: Moment,
	}

	#[derive(
		Encode,
		Decode,
		RuntimeDebug,
		Eq,
		PartialEq,
		Clone,
		Copy,
		Ord,
		PartialOrd,
		TypeInfo,
		MaxEncodedLen,
	)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct OracleKeyDetail<Key> {
		pub key: Key,
		pub decimals: u32,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Provide the implementation to combine raw values to produce
		/// aggregated value
		type CombineData: CombineData<Self::OracleKey, TimestampedValueOf<Self>>;

		/// The data key type
		type OracleKey: Parameter + Member + MaxEncodedLen + MaybeSerializeDeserialize;

		/// The data value type
		type OracleValue: Parameter + Member + Ord + MaxEncodedLen + MaybeSerializeDeserialize;

		/// Oracle operators.
		type Members: SortedMembers<Self::AccountId>;

		/// Time provider
		type Time: Time;
	}

	/// Raw values for each oracle operators
	#[pallet::storage]
	#[pallet::getter(fn raw_values)]
	pub type RawValues<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::AccountId,
		Twox64Concat,
		T::OracleKey,
		TimestampedValueOf<T>,
	>;

	/// Up to date combined value from Raw Values
	#[pallet::storage]
	#[pallet::getter(fn values)]
	pub type Values<T: Config> = StorageMap<_, Twox64Concat, T::OracleKey, TimestampedValueOf<T>>;

	/// Oracle key details
	#[pallet::storage]
	#[pallet::getter(fn key_details)]
	pub type OracleKeyDetails<T: Config> =
		StorageMap<_, Twox64Concat, T::OracleKey, OracleKeyDetail<T::OracleKey>>;

	/// If an oracle operator has fed a value in this block
	#[pallet::storage]
	pub type IsFeed<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::BlockNumber,
		bool,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// New feed data is submitted.
		NewFeedData { who: T::AccountId, values: Vec<(T::OracleKey, T::OracleValue)> },
		// New oracle key is added.
		NewOracleKey { who: T::AccountId, key: T::OracleKey, decimals: u32 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// Sender does not have permission
		NoPermission,
		// Feeder has already feeded at this block
		AlreadyFeeded,
		// No key provides in oracle
		NoOracleKeyProvided,
		// Oracle key has already added
		AlreadyAdded,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub oracles_key: Vec<(T::OracleKey, u32)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { oracles_key: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_oracle_key_detail(&self.oracles_key);
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn feed_values(
			origin: OriginFor<T>,
			values: Vec<(T::OracleKey, T::OracleValue)>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			Self::do_feed_values(who, values)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn add_oracle_key(
			origin: OriginFor<T>,
			key: T::OracleKey,
			decimals: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			ensure!(!OracleKeyDetails::<T>::contains_key(&key), Error::<T>::AlreadyAdded);
			ensure!(T::Members::contains(&who), Error::<T>::NoPermission);

			OracleKeyDetails::<T>::insert(&key, OracleKeyDetail { key: key.clone(), decimals });

			Self::deposit_event(Event::NewOracleKey { who, key, decimals });

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	// A function to initialize validators
	fn initialize_oracle_key_detail(oracle_details: &[(T::OracleKey, u32)]) {
		if !oracle_details.is_empty() {
			for detail in oracle_details {
				let oracle_value = OracleKeyDetail { key: detail.0.clone(), decimals: detail.1 };
				OracleKeyDetails::<T>::insert(detail.0.clone(), oracle_value);
			}
		}
	}

	pub fn get_raw_values(key: &T::OracleKey) -> Vec<TimestampedValueOf<T>> {
		T::Members::sorted_members()
			.iter()
			.filter_map(|x| Self::raw_values(x, key))
			.collect()
	}

	pub fn get(key: &T::OracleKey) -> Option<TimestampedValueOf<T>> {
		Self::values(key)
	}

	fn combined(key: &T::OracleKey) -> Option<TimestampedValueOf<T>> {
		let values = Self::get_raw_values(key);
		T::CombineData::combine_data(key, values, Self::values(key))
	}

	fn do_feed_values(
		who: T::AccountId,
		values: Vec<(T::OracleKey, T::OracleValue)>,
	) -> DispatchResult {
		// ensure feeder is authorized
		ensure!(T::Members::contains(&who), Error::<T>::NoPermission);

		// ensure account hasn't dispatched an updated yet
		ensure!(
			!IsFeed::<T>::get(&who, <frame_system::Pallet<T>>::block_number()),
			Error::<T>::AlreadyFeeded
		);

		let now = T::Time::now();
		for (key, value) in &values {
			ensure!(OracleKeyDetails::<T>::contains_key(key), Error::<T>::NoOracleKeyProvided);
			let timestamped = TimestampedValue { value: value.clone(), timestamp: now };
			RawValues::<T>::insert(&who, key, timestamped);

			// Update `Values` storage if `combined` yielded result.
			if let Some(combined) = Self::combined(key) {
				Values::<T>::insert(key, combined);
			}
		}

		IsFeed::<T>::insert(who.clone(), <frame_system::Pallet<T>>::block_number(), true);

		Self::deposit_event(Event::NewFeedData { who, values });
		Ok(())
	}
}
