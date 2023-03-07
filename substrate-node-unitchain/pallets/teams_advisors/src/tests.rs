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
fn add_member_success() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));

		//check that the user exists
		assert_eq!(Profile::all_users().is_some(), true);
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// check that the member was added
		assert_eq!(TeamsAdvisors::members(999).is_some(), true);
		assert_eq!(TeamsAdvisors::members(999).unwrap().len(), 1);

		// check events
		System::assert_has_event(RuntimeEvent::TeamsAdvisors(crate::Event::MemberAdded { asset_id: 999, username: dummy_vec.clone(), token_quantity: 100, cliff_period: 20, vest_period: 100, user_id: 1 }));
	});
}

#[test]
fn add_member_fail_no_permission() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			TeamsAdvisors::add_member(
				RuntimeOrigin::signed(1),
				999,
				vec![0],
				100,
				20,
				100
			),
			Error::<Test>::NotOwnerIssuerOrAdmin
		);
	});
}

#[test]
fn add_member_fail_already_member() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// cannot add a member twice
		assert_noop!(
			TeamsAdvisors::add_member(
				RuntimeOrigin::signed(0),
				999,
				dummy_vec.clone(),
				100,
				20,
				100
			),
			Error::<Test>::AlreadyMember
		);
	});
}

#[test]
fn add_member_fail_user_unexistent() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];

		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));

		//check that the user exists
		assert_eq!(Profile::all_users().is_some(), true);

		// cannot add a user that does not exist
		assert_noop!(
			TeamsAdvisors::add_member(
				RuntimeOrigin::signed(0),
				999,
				vec![1],
				100,
				20,
				100
			),
			Error::<Test>::InvalidUsername
		);
	});
}

#[test]
fn add_member_fail_no_profiles_created() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];

		// cannot add a user that does not exist
		assert_noop!(
			TeamsAdvisors::add_member(
				RuntimeOrigin::signed(0),
				999,
				dummy_vec.clone(),
				100,
				20,
				100
			),
			Error::<Test>::NoProfilesCreated
		);
	});
}

#[test]
fn update_member_success() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));

		//check that the user exists
		assert_eq!(Profile::all_users().is_some(), true);
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// check that the member was added
		assert_eq!(TeamsAdvisors::members(999).is_some(), true);
		assert_eq!(TeamsAdvisors::members(999).unwrap().len(), 1);

		// update the member
		assert_ok!(TeamsAdvisors::update_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 200, 30, 200));
		assert_eq!(TeamsAdvisors::members(999).unwrap()[0].token_quantity, 200);

		// check events
		System::assert_has_event(RuntimeEvent::TeamsAdvisors(crate::Event::MemberUpdated { asset_id: 999, username: dummy_vec.clone(), new_token_quantity: 200, new_cliff_period: 30, new_vest_period: 200}));
	});
}

#[test]
fn update_member_fail_no_permission() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// update should fail
		assert_noop!(
			TeamsAdvisors::update_member(
				RuntimeOrigin::signed(1),
				999,
				dummy_vec.clone(),
				200,
				30,
				200
			),
			Error::<Test>::NotOwnerIssuerOrAdmin
		);
	});
}

#[test]
fn update_member_fail_no_members_created() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];
	
		// update should fail
		assert_noop!(
			TeamsAdvisors::update_member(
				RuntimeOrigin::signed(0),
				999,
				dummy_vec.clone(),
				200,
				30,
				200
			),
			Error::<Test>::MemberDoesNotExist
		);
	});
}

#[test]
fn update_member_fail_member_not_exist() {
	new_test_ext().execute_with(|| {
		// create user and add it to team
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// update should fail
		assert_noop!(
			TeamsAdvisors::update_member(
				RuntimeOrigin::signed(0),
				999,
				vec![1],
				200,
				30,
				200
			),
			Error::<Test>::MemberDoesNotExist
		);
	});
}

#[test]
fn delete_member_success() {
	new_test_ext().execute_with(|| {
		// create user and add it to team
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// delete the member
		assert_ok!(TeamsAdvisors::delete_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// check that the member was deleted
		assert_eq!(TeamsAdvisors::members(999).unwrap().len(), 0);

		// check events
		System::assert_has_event(RuntimeEvent::TeamsAdvisors(crate::Event::MemberDeleted { asset_id: 999, username: dummy_vec.clone() }));
	});
}

#[test]
fn delete_member_fail_no_permission() {
	new_test_ext().execute_with(|| {
		// create user and add it to team
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// delete the member fail
		assert_noop!(
			TeamsAdvisors::delete_member(
				RuntimeOrigin::signed(1),
				999,
				dummy_vec.clone()
			),
			Error::<Test>::NotOwnerIssuerOrAdmin
		);
	});
}

#[test]
fn delete_member_fail_no_members_created() {
	new_test_ext().execute_with(|| {
		let dummy_vec = vec![0];

		// delete the member fail
		assert_noop!(
			TeamsAdvisors::delete_member(
				RuntimeOrigin::signed(0),
				999,
				dummy_vec.clone()
			),
			Error::<Test>::MemberDoesNotExist
		);
	});
}

#[test]
fn delete_member_fail_member_does_not_exist() {
	new_test_ext().execute_with(|| {
		// create user and add it to team
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to the team
		assert_ok!(TeamsAdvisors::add_member(RuntimeOrigin::signed(0), 999, dummy_vec.clone(), 100, 20, 100));

		// delete the member fail
		assert_noop!(
			TeamsAdvisors::delete_member(
				RuntimeOrigin::signed(0),
				999,
				vec![1]
			),
			Error::<Test>::MemberDoesNotExist
		);
	});
}

#[test]
fn create_advisor_success() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to advisors
		assert_ok!(TeamsAdvisors::create_advisor(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// check that the advisor was added
		assert_eq!(TeamsAdvisors::advisors(999).unwrap().len(), 1);
		assert_eq!(TeamsAdvisors::advisors(999).unwrap()[0].username, dummy_vec.clone());

		// check events
		System::assert_has_event(RuntimeEvent::TeamsAdvisors(crate::Event::AdvisorCreated { asset_id: 999, username: dummy_vec.clone(), user_id: 1 }));
	});
}

#[test]
fn create_advisor_fail_no_permission() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to advisors fail
		assert_noop!(
			TeamsAdvisors::create_advisor(
				RuntimeOrigin::signed(1),
				999,
				dummy_vec.clone()
			),
			Error::<Test>::NotOwnerIssuerOrAdmin
		);
	});
}

#[test]
fn create_advisor_fail_advisor_already_exist() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to advisors
		assert_ok!(TeamsAdvisors::create_advisor(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// add the member to advisors fail
		assert_noop!(
			TeamsAdvisors::create_advisor(
				RuntimeOrigin::signed(0),
				999,
				dummy_vec.clone()
			),
			Error::<Test>::AdvisorAlreadyExists
		);
	});
}

#[test]
fn create_advisor_fail_user_unexistent() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add unexistent user to advisors fail
		assert_noop!(
			TeamsAdvisors::create_advisor(
				RuntimeOrigin::signed(0),
				999,
				vec![1]
			),
			Error::<Test>::InvalidUsername
		);
	});
}

#[test]
fn create_advisor_fail_no_users_created() {
	new_test_ext().execute_with(|| {
		// pallet profile is empty
		assert_noop!(
			TeamsAdvisors::create_advisor(
				RuntimeOrigin::signed(0),
				999,
				vec![1]
			),
			Error::<Test>::NoProfilesCreated
		);
	});
}

#[test]
fn remove_advisor_success() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to advisors
		assert_ok!(TeamsAdvisors::create_advisor(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// remove the user from advisors
		assert_ok!(TeamsAdvisors::remove_advisor(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// check that the advisor was removed
		assert_eq!(TeamsAdvisors::advisors(999).unwrap().len(), 0);

		// check events
		System::assert_has_event(RuntimeEvent::TeamsAdvisors(crate::Event::AdvisorDeleted { asset_id: 999, username: dummy_vec.clone() }));
	});
}

#[test]
fn remove_advisor_fail_no_permission() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to advisors
		assert_ok!(TeamsAdvisors::create_advisor(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// remove the user from advisors fail
		assert_noop!(
			TeamsAdvisors::remove_advisor(
				RuntimeOrigin::signed(1),
				999,
				dummy_vec.clone()
			),
			Error::<Test>::NotOwnerIssuerOrAdmin
		);
	});
}

#[test]
fn remove_advisor_fail_no_advisors_created() {
	new_test_ext().execute_with(|| {
		// delete advisors with no advisors created should fail
		assert_noop!(
			TeamsAdvisors::remove_advisor(
				RuntimeOrigin::signed(0),
				999,
				vec![1]
			),
			Error::<Test>::AdvisorDoesNotExist
		);
	});
}

#[test]
fn remove_advisor_fail_advisor_not_found() {
	new_test_ext().execute_with(|| {
		// create user and add it to advisors
		let dummy_vec = vec![0];
		// create member in profile pallet
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(0), 1, dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), dummy_vec.clone(), 1234, 10, dummy_vec.clone() ));
		
		// add the member to advisors
		assert_ok!(TeamsAdvisors::create_advisor(RuntimeOrigin::signed(0), 999, dummy_vec.clone()));

		// delete advisor that does not exist should fail
		assert_noop!(
			TeamsAdvisors::remove_advisor(
				RuntimeOrigin::signed(0),
				999,
				vec![1]
			),
			Error::<Test>::AdvisorDoesNotExist
		);
	});
}
	