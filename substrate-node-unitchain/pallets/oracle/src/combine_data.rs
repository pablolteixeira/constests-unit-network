use crate::{Config, MomentOf, TimestampedValueOf};
use frame_support::traits::{Get, Time};
use sp_runtime::traits::Saturating;
use sp_std::{marker, prelude::*};

/// Combine data provided by operators
pub trait CombineData<Key, TimestampedValue> {
	/// Combine data provided by operators
	fn combine_data(
		key: &Key,
		values: Vec<TimestampedValue>,
		prev_value: Option<TimestampedValue>,
	) -> Option<TimestampedValue>;
}

/// Sort by value and returns median timestamped value.
/// Returns prev_value if not enough valid values.
pub struct DefaultCombineData<T, MinimumCount, ExpiresIn>(
	marker::PhantomData<(T, MinimumCount, ExpiresIn)>,
);

impl<T, MinimumCount, ExpiresIn> CombineData<<T as Config>::OracleKey, TimestampedValueOf<T>>
	for DefaultCombineData<T, MinimumCount, ExpiresIn>
where
	T: Config,
	MinimumCount: Get<u32>,
	ExpiresIn: Get<MomentOf<T>>,
{
	fn combine_data(
		_key: &<T as Config>::OracleKey,
		mut values: Vec<TimestampedValueOf<T>>,
		prev_value: Option<TimestampedValueOf<T>>,
	) -> Option<TimestampedValueOf<T>> {
		let expires_in = ExpiresIn::get();
		let now = T::Time::now();

		values.retain(|x| x.timestamp.saturating_add(expires_in) > now);

		let count = values.len() as u32;
		let minimum_count = MinimumCount::get();
		if count < minimum_count || count == 0 {
			return prev_value
		}

		let mid_index = count / 2;
		// Won't panic as `values` ensured not empty.
		let (_, value, _) =
			values.select_nth_unstable_by(mid_index as usize, |a, b| a.value.cmp(&b.value));
		Some(value.clone())
	}
}
