use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_paper_error_asset_dont_exist() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let who = RuntimeOrigin::signed(ALICE);

		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UNIT TOKEN".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("TOKEN CREATED FOR TEST".as_bytes().to_vec()).unwrap();

		assert_noop!(Papers::create_paper(who.clone(), 0, 10, 0, title, text), Error::<Test>::AssetDontExist);
	});
}

#[test]
fn create_paper_success() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let who = RuntimeOrigin::signed(ALICE);
		
		// Crate Asset
		assert_ok!(Assets::create(who.clone(), 0.into(), ALICE, 100));
		
		// Create Paper
		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UNIT PAPER".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("PAPER CREATED FOR TEST".as_bytes().to_vec()).unwrap();

		// asset_id = 0, paper_id = 10, position = 0
		assert_ok!(Papers::create_paper(who.clone(), 0, 10, 0, title.clone(), text.clone()));

		// Check Paper was pushed AssetIdMao value vector
		assert!(AssetIdMap::<Test>::contains_key(ALICE, 0));

		let asset_id_vec = AssetIdMap::<Test>::get(ALICE, 0).unwrap();

		assert_eq!(asset_id_vec.len(), 1);

		// Check Paper inserted to PaperIdMap is correct
		let paper = PaperIdMap::<Test>::get(ALICE, 10).unwrap();
		
		assert_eq!(paper.asset_id, 0);
		assert_eq!(paper.position, 0);
		assert_eq!(paper.title, title);
		assert_eq!(paper.text, text);
		assert_eq!(paper.user_address, ALICE);
		assert_eq!(paper.paper_id, 10);

		System::assert_last_event(Event::PaperCreated { who: ALICE, paper_id: 10, position: 0 }.into());
	});
}

#[test]
fn create_subpaper() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let who = RuntimeOrigin::signed(ALICE);
		
		// Crate Asset
		assert_ok!(Assets::create(who.clone(), 0.into(), ALICE, 100));
		
		// Create Paper
		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UNIT PAPER".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("PAPER CREATED FOR TEST".as_bytes().to_vec()).unwrap();

		// asset_id = 0, paper_id = 10, position = 0
		assert_ok!(Papers::create_paper(who.clone(), 0, 10, 0, title.clone(), text.clone()));

		System::assert_last_event(Event::PaperCreated { who: ALICE, paper_id: 10, position: 0 }.into());

		// Create SubPaper
		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UNIT SUBPAPER".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("SUBPAPER CREATED FOR TEST".as_bytes().to_vec()).unwrap();
		
		// asset_id = 0, paper_id = 10, position = 100
		assert_ok!(Papers::create_paper(who.clone(), 0, 10, 0, title, text));

		System::assert_last_event(Event::PaperCreated { who: ALICE, paper_id: 10, position: 0 }.into());

		// Check SubPaper was not pushed AssetIdMao value vector
		assert!(AssetIdMap::<Test>::contains_key(ALICE, 0));

		let asset_id_vec = AssetIdMap::<Test>::get(ALICE, 0).unwrap();

		assert_eq!(asset_id_vec.len(), 1);

		// Check SubPaperMap len value was increased in 1
		assert!(SubPapersMap::<Test>::contains_key(ALICE, 10));

		let subpapers_vec = SubPapersMap::<Test>::get(ALICE, 10).unwrap();

		assert_eq!(subpapers_vec.len(), 1);
	});
}

#[test]
fn updated_paper_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let who = RuntimeOrigin::signed(ALICE);

		// Crate Asset
		assert_ok!(Assets::create(who.clone(), 0.into(), ALICE, 100));

		// Update Paper
		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UPDATED PAPER".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("UPDATED PAPER FOR TEST".as_bytes().to_vec()).unwrap();
		
		assert_noop!(Papers::update_paper(who.clone(), 0, 0, title.clone(), text.clone()), Error::<Test>::PaperIdDontExist);
	});
}

#[test]
fn updated_paper() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let who = RuntimeOrigin::signed(ALICE);

		// Crate Asset
		assert_ok!(Assets::create(who.clone(), 0.into(), ALICE, 100));

		// Create Paper
		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UNIT PAPER".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("PAPER CREATED FOR TEST".as_bytes().to_vec()).unwrap();

		// asset_id = 0, paper_id = 10, position = 0
		assert_ok!(Papers::create_paper(who.clone(), 0, 10, 0, title.clone(), text.clone()));

		System::assert_last_event(Event::PaperCreated { who: ALICE, paper_id: 10, position: 0 }.into());

		// Update Paper
		let title: BoundedVec<u8, <Test as pallet::Config>::TitleLimit> = BoundedVec::try_from("UPDATED PAPER".as_bytes().to_vec()).unwrap();
		let text: BoundedVec<u8, <Test as pallet::Config>::TextLimit> = BoundedVec::try_from("UPDATED PAPER FOR TEST".as_bytes().to_vec()).unwrap();
		
		assert_ok!(Papers::update_paper(who.clone(), 10, 20, title.clone(), text.clone()));
		
		System::assert_last_event(Event::PaperUpdated { who: ALICE, paper_id: 10, position: 20, title, text }.into());
	});
}
