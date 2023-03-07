use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_success_when_add_new_oracle_key() {
	new_test_ext().execute_with(|| {
		let key = 4;
		let decimals = 8;

		assert_ok!(Oracle::add_oracle_key(RuntimeOrigin::signed(ALICE), key, decimals));

		System::assert_last_event(Event::NewOracleKey { who: ALICE, key, decimals }.into());
	});
}

#[test]
fn should_fail_when_add_new_oracle_with_same_key() {
	new_test_ext().execute_with(|| {
		let key = 1;
		let decimals = 8;

		assert_noop!(
			Oracle::add_oracle_key(RuntimeOrigin::signed(ALICE), key, decimals),
			Error::<Test>::AlreadyAdded
		);
	});
}

#[test]
fn should_fail_when_add_new_oracle_with_not_member() {
	new_test_ext().execute_with(|| {
		let key = 4;
		let decimals = 8;

		assert_noop!(
			Oracle::add_oracle_key(RuntimeOrigin::signed(DEV), key, decimals),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn should_success_when_feeder_feed_data() {
	new_test_ext().execute_with(|| {
		let key = 1;

		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(key, 1000)]));

		let raw_values = Oracle::get_raw_values(&key);
		assert_eq!(raw_values, vec![TimestampedValue { value: 1000, timestamp: 100 },]);

		let value = Oracle::get(&key).unwrap();
		assert_eq!(value.value, 1000);
	});
}

#[test]
fn should_fail_when_feeder_is_not_member() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Oracle::feed_values(RuntimeOrigin::signed(DEV), vec![(1, 1000)]),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn should_fail_when_feeder_feed_same_block() {
	new_test_ext().execute_with(|| {
		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(1, 1000)]));

		assert_noop!(
			Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(1, 1000)]),
			Error::<Test>::AlreadyFeeded
		);
	});
}

#[test]
fn should_fail_when_feeder_feed_with_no_key() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(5, 1000)]),
			Error::<Test>::NoOracleKeyProvided
		);
	});
}

#[test]
fn should_success_when_get_raw_values() {
	new_test_ext().execute_with(|| {
		let key: u32 = 1;

		let raw_values = Oracle::get_raw_values(&key);
		assert_eq!(raw_values, vec![]);

		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(key, 1000)]));
		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(BOB), vec![(key, 2000)]));

		let raw_values = Oracle::get_raw_values(&key);
		assert_eq!(
			raw_values,
			vec![
				TimestampedValue { value: 1000, timestamp: 100 },
				TimestampedValue { value: 2000, timestamp: 100 },
			]
		);
	});
}

#[test]
fn should_success_when_get_value() {
	new_test_ext().execute_with(|| {
		let key: u32 = 1;

		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(key, 1000)]));

		let value = Oracle::get(&key).unwrap();
		assert_eq!(value.value, 1000);
	});
}

#[test]
fn should_success_when_get_median_value() {
	new_test_ext().execute_with(|| {
		let key: u32 = 1;

		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(ALICE), vec![(key, 1000)]));
		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(BOB), vec![(key, 2000)]));
		assert_ok!(Oracle::feed_values(RuntimeOrigin::signed(CAROL), vec![(key, 3000)]));

		let value = Oracle::get(&key).unwrap();
		assert_eq!(value.value, 2000);
	});
}
