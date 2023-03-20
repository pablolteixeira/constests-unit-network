use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

/*
* FUNCTIONS
- contest_new
	ensure!(!ContestsMap::<T>::contains_key(contest_id), Error::<T>::ContestIdAlreadyInUse);
	ensure!(T::Assets::asset_exists(prize_token_id.clone()), Error::<T>::AssetDontExist);
	ensure!(prize_token_winner >= T::MinTokenWinner::get(), Error::<T>::PrizeTokenWinnerTooSmall);
	ensure!(title.len() as u32 >= T::MinTitleLength::get(), Error::<T>::TitleTooSmall);
	ensure!(token_symbol.len() as u32 >= T::MinTokenSymbolLength::get(), Error::<T>::TokenSymbolTooSmall);
	ensure!(description.len() as u32 >= T::MinDescriptionLength::get(), Error::<T>::DescriptionTooSmall);
	ensure!(prize_token_amount >= T::MinTokenAmount::get().into(), Error::<T>::TokenAmountTooSmall);
	ensure!(T::Assets::balance(prize_token_id, &who) >= T::MinTokenAmount::get().into(), Error::<T>::AssetBalanceInsufficient);

- update_contest
	ensure!(ContestsMap::<T>::contains_key(contest_id.clone()), Error::<T>::ContestIdDontExist);
	ensure!(title.len() as u32 >= T::MinTitleLength::get(), Error::<T>::TitleTooSmall);
	ensure!(description.len() as u32 >= T::MinDescriptionLength::get(), Error::<T>::DescriptionTooSmall);

	// Unwrap used because there is a ensure! above testing that the element exist with contest_id key 
	let contest = ContestsMap::<T>::get(contest_id.clone()).unwrap();

	ensure!(contest.user_address == who, Error::<T>::OnlyOwnerCanChange);

- create_entry_contest
	ensure!(!EntriesMap::<T>::contains_key(entry_id), Error::<T>::EntryIdAlreadyExist);		
	ensure!(ContestsMap::<T>::contains_key(contest_id), Error::<T>::ContestIdDontExist);

- assign_contest_winner
	ensure!(EntriesMap::<T>::contains_key(entry_id.clone()), Error::<T>::EntryIdDontExist);

	// Unwrap used because there is a ensure! above testing that the element exist with contest_id key 
	let contest_entry = EntriesMap::<T>::get(entry_id).unwrap();

	ensure!(ContestsMap::<T>::contains_key(contest_entry.contest_id), Error::<T>::ContestIdDontExist);
	
	// Unwrap used because there is a ensure! above testing that the element exist with contest_id key 
	let contest = ContestsMap::get(contest_entry.contest_id).unwrap();
	
	ensure!(contest.statcode, Error::<T>::ContestAlreadyClosed);
	ensure!(who == contest.user_address, Error::<T>::OnlyOwnerCanAssignContestWinner);

- close_contest
	ensure!(ContestsMap::<T>::contains_key(contest_id.clone()), Error::<T>::ContestIdDontExist);

	let contest = ContestsMap::<T>::get(contest_id.clone()).unwrap();

	ensure!(contest.statcode == true, Error::<T>::ContestAlreadyClosed);
	ensure!(contest.user_address == who, Error::<T>::OnlyOwnerCanCloseContest);
*/

fn create_contest() {
	let title: BoundedVec<u8, <Test as pallet::Config>::MaxTitleLength> = BoundedVec::try_from("UNIT CONTEST".as_bytes().to_vec()).unwrap();
	let token_symbol: BoundedVec<u8, <Test as pallet::Config>::MaxTokenSymbolLength> = BoundedVec::try_from("UNIT".as_bytes().to_vec()).unwrap();
	let contest_end_date: BoundedVec<u8, <Test as pallet::Config>::MaxContestEndDateLength> = BoundedVec::try_from("20/10/2023".as_bytes().to_vec()).unwrap();
	let description: BoundedVec<u8, <Test as pallet::Config>::MaxDescriptionLength> = BoundedVec::try_from("Roseum tenerum flores prunorum in aura tepida veris saltantes.".as_bytes().to_vec()).unwrap();

	assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 1));

	assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 1_000_000_000_000_000));
	assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), Contests::account_id(), 1_000_000_000_000_000));

	assert_ok!(Contests::contest_new(
		RuntimeOrigin::signed(ALICE),
		0,
		title,
		0,
		100,
		2,
		token_symbol,
		contest_end_date,
		description)
	);
}

#[test]
fn create_contest_sucessfull() {
	new_test_ext().execute_with(|| {
		create_contest();

		System::assert_last_event(Event::ContestCreated { who: ALICE, contest_id: 0, title:  BoundedVec::try_from("UNIT CONTEST".as_bytes().to_vec()).unwrap()}.into())
	});
}

#[test]
fn update_contest_sucessfull() {
	new_test_ext().execute_with(|| {
		create_contest();

		let title: BoundedVec<u8, <Test as pallet::Config>::MaxTitleLength> = BoundedVec::try_from("UNIT CONTEST UPDATED".as_bytes().to_vec()).unwrap();
		let token_symbol: BoundedVec<u8, <Test as pallet::Config>::MaxTokenSymbolLength> = BoundedVec::try_from("UNIT UPDATED".as_bytes().to_vec()).unwrap();
		let contest_end_date: BoundedVec<u8, <Test as pallet::Config>::MaxContestEndDateLength> = BoundedVec::try_from("20/10/2030".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as pallet::Config>::MaxDescriptionLength> = BoundedVec::try_from("UPDATED UPDATED UPDATED UPDATED UPDATED UPDATED UPDATED UPDATED UPDATED UPDATED.".as_bytes().to_vec()).unwrap();

		Contests::update_contest(
			RuntimeOrigin::singed(ALICE),
			0.into(),
			title,
			description,
			contest_end_date
		);

		System::assert_last_event(Event::ContestUpdated { who: ALICE, contest_id: 0, title, description, contest_end_date }.into())
	});
}

#[test]
fn create_entry_sucessfull() {
	new_test_ext().execute_with(|| {
		create_contest();

		Constests::create_contest_entry(
			RuntimeOrigin::signed(BOB),
			0,
			10
		);

		System::assert_last_event(Event::EntryCreated { who: BOB, contest_id: 0, entry: 10 }.into());
	});
}

#[test]
fn assign_contest_winner_sucessfull() {
	new_test_ext().execute_with(|| {
		create_contest();

		Constests::create_contest_entry(
			RuntimeOrigin::signed(BOB),
			0,
			10
		);

		Contests::assign_contest_winner(
			RuntimeOrigin::signed(ALICE),
			10
		);

		System::assert_last_event(Event::ContestWinnerAssigned { contest_id: 0, winner: BOB, prize: 50.into() }.into());
	});
}

#[test]
fn close_contest_sucessfull() {
	new_test_ext().execute_with(|| {
		create_contest();

		Contests::close_contest(
			RuntimeOrigin::signed(ALICE),
			0
		);

		System::assert_last_event(Event::ContestClosed { who: ALICE, contest_id: 0 }.into());
	}); 
}