use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok,
    traits::{fungibles::InspectEnumerable},
};
use sp_core::hexdisplay::HexDisplay;

use super::*;

// function to get all the created assets id
fn asset_ids() -> Vec<u32> {
	let mut s: Vec<_> = Assets::asset_ids().collect();
	s.sort();
	s
}

// aux function to print the entire state 
pub fn _print_state() {
	let mut key = vec![];
	while let Some(next) = sp_io::storage::next_key(&key) {
		let val = sp_io::storage::get(&next).unwrap().to_vec();
		println!("{} <=> {}",
		HexDisplay::from(&next),
		HexDisplay::from(&val));
		key = next;
	}
}

#[test]
fn check_initial_state() {
	new_test_ext().execute_with(|| {
		assert_eq!(asset_ids(), vec![999]);
		
	});
}


#[test]
fn create_rank_should_fail_no_owner() {
	new_test_ext().execute_with(|| {
		assert_noop!(RanksModule::create_rank(RuntimeOrigin::signed(1), 999, [1,1,1,1,1].into(), 1000), Error::<Test>::OnlyTokenOwner);
	});
}

#[test]
fn create_rank_should_succeed() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));

		System::assert_has_event(RuntimeEvent::RanksModule(crate::Event::RankCreated { asset_id: 999, rank: RankInfo{ name: [1,1,1,1,1].into(), min_tokens: 1000}  }));
	});
}

#[test]
fn update_non_existent_rank_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));

		assert_noop!(RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,2].into(), [2,2,2,2,2].into(), 2000), Error::<Test>::InvalidRankName);
	});
}

#[test]
fn update_rank_not_owner_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));

		// the rank name is valid
		assert_noop!(RanksModule::update_rank(RuntimeOrigin::signed(1), 999, [1,1,1,1,1].into(), [2,2,2,2,2].into(), 2000), Error::<Test>::OnlyTokenOwner);
	});
}

#[test]
fn update_rank_with_no_ranks_should_fail() {
	new_test_ext().execute_with(|| {

		assert_noop!(RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), [2,2,2,2,2].into(), 2000), Error::<Test>::NoRanksAvailable);
	});
}

#[test]
fn update_unexistent_asset_should_fail() {
	new_test_ext().execute_with(|| {
		assert_noop!(RanksModule::update_rank(RuntimeOrigin::signed(0), 0, [1,1,1,1,1].into(), [2,2,2,2,2].into(), 2000), Error::<Test>::InvalidAsset);
	});
}

// this test is to check if after an update there is no repeated rank names or ranks with the same min_tokens
#[test]
fn update_rank_invalid() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,2].into(), 2000));
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,3].into(), 3000));
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,4].into(), 4000));

		assert_noop!(RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), [1,1,1,1,2].into(), 5000), Error::<Test>::RankNameUsed);
		assert_noop!(RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), [1,1,1,1,5].into(), 2000), Error::<Test>::RankMinTokensUsed);

		// the owner can modify an existen rank with another name or min_tokens
		assert_ok!( RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), [1,1,1,1,1].into(), 5000));
		assert_ok!( RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), [1,1,1,1,5].into(), 5000));
	});
}

#[test]
fn update_rank_success() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));

		assert_ok!( RanksModule::update_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), [2,2,2,2,2].into(), 2000));

		System::assert_has_event(RuntimeEvent::RanksModule(crate::Event::RankUpdated { asset_id: 999, new_rank: RankInfo{ name: [2,2,2,2,2].into(), min_tokens: 2000}  }));
	});
}


#[test]
fn delete_rank_unexistent_asset_should_fail() {
	new_test_ext().execute_with(|| {
		assert_noop!(RanksModule::delete_rank(RuntimeOrigin::signed(0), 0, [1,1,1,1,1].into()), Error::<Test>::InvalidAsset);
	});
}

#[test]
fn delete_rank_without_ranks_created() {
	new_test_ext().execute_with(|| {
		assert_noop!(RanksModule::delete_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into()), Error::<Test>::NoRanksAvailable);
	});
}

#[test]
fn delete_rank_unexistent_rank() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));
		assert_noop!(RanksModule::delete_rank(RuntimeOrigin::signed(0), 999, [1,1,1,2].into()), Error::<Test>::InvalidRankName);
	});
}

#[test]
fn delete_rank_no_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));
		assert_noop!(RanksModule::delete_rank(RuntimeOrigin::signed(1), 999, [1,1,1,1].into()), Error::<Test>::OnlyTokenOwner);
	});
}

#[test]
fn delete_rank_success() {
	new_test_ext().execute_with(|| {
		assert_ok!(RanksModule::create_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into(), 1000));
		assert_ok!(RanksModule::delete_rank(RuntimeOrigin::signed(0), 999, [1,1,1,1,1].into()));

		System::assert_has_event(RuntimeEvent::RanksModule(crate::Event::RankDeleted { asset_id: 999, removed_rank: RankInfo{ name: [1,1,1,1,1].into(), min_tokens: 1000}  }));
	});
}