use crate::{mock::*, Error, Event, Rewards};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_reward__and_get_works() {
	new_test_ext().execute_with(|| {
		// Assert that the correct event was deposited
		assert_ok!(RewardsPallet::create_reward(RuntimeOrigin::signed(1), 1, "faiz".as_bytes().to_vec(), "https://faizsarwar.github.io".as_bytes().to_vec(), 1));

		let expected_value = Rewards {
			asset_id: 1,
			user_address: 1,
			title: "faiz".as_bytes().to_vec(),
			url_link: "https://faizsarwar.github.io".as_bytes().to_vec(),
			min_tokens: 1,
			reward_id: 1,   
		};

		// checking the storage 
		assert_eq!(RewardsPallet::reward_item(1),Some(expected_value));
	});
}

#[test]
fn update_reward_works() {
	new_test_ext().execute_with(|| {
		// Assert that the correct event was deposited
		assert_ok!(RewardsPallet::create_reward(RuntimeOrigin::signed(1), 1, "faiz".as_bytes().to_vec(), "https://faizsarwar.github.io".as_bytes().to_vec(), 1));

		let expected_value = Rewards {
			asset_id: 1,
			user_address: 1,
			title: "faiz".as_bytes().to_vec(),
			url_link: "https://faizsarwar.github.io".as_bytes().to_vec(),
			min_tokens: 1,
			reward_id: 1,   
		};

		// checking the storage 
		assert_eq!(RewardsPallet::reward_item(1),Some(expected_value));

		assert_ok!(RewardsPallet::update_reward(RuntimeOrigin::signed(1), 2, "faiz".as_bytes().to_vec(), "https://faizsarwar.github.io".as_bytes().to_vec(), 2, 1));

		let updated_expected_output = Rewards {
			asset_id: 2,
			user_address: 1,
			title: "faiz".as_bytes().to_vec(),
			url_link: "https://faizsarwar.github.io".as_bytes().to_vec(),
			min_tokens: 2,
			reward_id: 1,   
		};

		// checking the storage 
		assert_eq!(RewardsPallet::reward_item(1),Some(updated_expected_output));
	});
}


#[test]
fn delete_reward_works() {
	new_test_ext().execute_with(|| {
		// Assert that the correct event was deposited
		assert_ok!(RewardsPallet::create_reward(RuntimeOrigin::signed(1), 1, "faiz".as_bytes().to_vec(), "https://faizsarwar.github.io".as_bytes().to_vec(), 1));

		let expected_value = Rewards {
			asset_id: 1,
			user_address: 1,
			title: "faiz".as_bytes().to_vec(),
			url_link: "https://faizsarwar.github.io".as_bytes().to_vec(),
			min_tokens: 1,
			reward_id: 1,   
		};

		// checking the storage 
		assert_eq!(RewardsPallet::reward_item(1),Some(expected_value));

		assert_ok!(RewardsPallet::delete_reward(RuntimeOrigin::signed(1),1));

		// checking the storage 
		assert_eq!(RewardsPallet::reward_item(1),None);
	});
}
