use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn should_work_when_ask_question() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(ALICE),
			0,
			question.clone(),
			description,
			video_link,
		));

		System::assert_last_event(
			Event::AskedQuestion { who: ALICE, question_id: 0, title: question }.into(),
		);
	});
}

#[test]
fn should_fail_when_ask_question_with_no_asset_balances() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_noop!(
			Questions::ask_question(
				RuntimeOrigin::signed(ALICE),
				0,
				question,
				description,
				video_link,
			),
			Error::<Test>::InsuffienctBalance
		);
	});
}

#[test]
fn should_fail_when_ask_question_with_empty_question() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_noop!(
			Questions::ask_question(
				RuntimeOrigin::signed(ALICE),
				0,
				question,
				description,
				video_link,
			),
			Error::<Test>::NoQuestionTitle
		);
	});
}

#[test]
fn should_work_when_reply_to_question() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(ALICE),
			0,
			question,
			description,
			video_link,
		));

		let answer: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Answer".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::reply_question(RuntimeOrigin::signed(ALICE), 0, answer.clone()));

		System::assert_last_event(
			Event::RepliedQuestion { who: ALICE, question_id: 0, answer }.into(),
		);
	});
}

#[test]
fn should_fail_when_reply_question_with_already_hash_answer() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(ALICE),
			0,
			question,
			description,
			video_link,
		));

		let answer: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Answer".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::reply_question(RuntimeOrigin::signed(ALICE), 0, answer.clone()));

		System::assert_last_event(
			Event::RepliedQuestion { who: ALICE, question_id: 0, answer: answer.clone() }.into(),
		);

		// Alice try to reply again.
		assert_noop!(
			Questions::reply_question(RuntimeOrigin::signed(ALICE), 0, answer),
			Error::<Test>::QuestionAlreadyReplied
		);
	});
}

#[test]
fn should_fail_when_reply_with_no_answer() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let answer: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("".as_bytes().to_vec()).unwrap();

		assert_noop!(
			Questions::reply_question(RuntimeOrigin::signed(ALICE), 0, answer),
			Error::<Test>::NoAnswer
		);
	});
}

#[test]
fn should_fail_when_reply_to_no_question() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let answer: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Answer".as_bytes().to_vec()).unwrap();

		assert_noop!(
			Questions::reply_question(RuntimeOrigin::signed(ALICE), 0, answer),
			Error::<Test>::UnknownQuestionId
		);
	});
}

#[test]
fn should_work_when_vote_answer() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(ALICE),
			0,
			question,
			description,
			video_link,
		));

		let answer: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Answer".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::reply_question(RuntimeOrigin::signed(ALICE), 0, answer.clone()));

		System::assert_last_event(
			Event::RepliedQuestion { who: ALICE, question_id: 0, answer }.into(),
		);

		// Alice try to upvote
		assert_ok!(Questions::vote(RuntimeOrigin::signed(ALICE), 0));

		System::assert_last_event(
			Event::VotedAnswer { who: ALICE, question_id: 0, votes: 1 }.into(),
		);

		let answer_data = AnswersData::<Test>::get(0).unwrap();
		let alice_vote = UserVoteData::<Test>::get(ALICE, 0);

		assert_eq!(answer_data.votes, 1);
		assert!(alice_vote);

		// Alice try to devote
		assert_ok!(Questions::vote(RuntimeOrigin::signed(ALICE), 0));
		System::assert_last_event(
			Event::VotedAnswer { who: ALICE, question_id: 0, votes: 0 }.into(),
		);

		let answer_data = AnswersData::<Test>::get(0).unwrap();
		let alice_vote = UserVoteData::<Test>::get(ALICE, 0);

		assert_eq!(answer_data.votes, 0);
		assert!(!alice_vote);
	});
}

#[test]
fn should_fail_when_vote_answer_with_no_answer_in_question() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(ALICE),
			0,
			question,
			description,
			video_link,
		));

		assert_noop!(
			Questions::vote(RuntimeOrigin::signed(ALICE), 0),
			Error::<Test>::NoAnserOnQuestion
		);
	});
}

#[test]
fn should_fail_when_vote_answer_with_no_question() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));

		assert_noop!(
			Questions::vote(RuntimeOrigin::signed(ALICE), 0),
			Error::<Test>::UnknownQuestionId
		);
	});
}

#[test]
fn lifecycle_should_work() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create asset
		assert_ok!(Assets::create(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		// Mint asset
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), ALICE, 100));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), BOB, 100));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(ALICE), 0.into(), CAROL, 100));

		// Alice create question.
		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(ALICE),
			0,
			question,
			description,
			video_link,
		));

		// Bob create question.
		let question: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let description: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();
		let video_link: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Question".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::ask_question(
			RuntimeOrigin::signed(BOB),
			0,
			question,
			description,
			video_link,
		));

		// Check total question.
		assert!(Questions::question_id() == 2);

		// Bob answer to alice question.
		let answer: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from("Test Answer".as_bytes().to_vec()).unwrap();

		assert_ok!(Questions::reply_question(RuntimeOrigin::signed(BOB), 0, answer.clone()));
		System::assert_last_event(
			Event::RepliedQuestion { who: BOB, question_id: 0, answer: answer.clone() }.into(),
		);

		// CAROL try to answer alice's question.
		assert_noop!(
			Questions::reply_question(RuntimeOrigin::signed(CAROL), 0, answer),
			Error::<Test>::QuestionAlreadyReplied
		);

		// BOB, CAROL and DEV Vote for bob question.
		assert_ok!(Questions::vote(RuntimeOrigin::signed(BOB), 0));
		assert_ok!(Questions::vote(RuntimeOrigin::signed(CAROL), 0));
		assert_ok!(Questions::vote(RuntimeOrigin::signed(DEV), 0));

		let answer_data = AnswersData::<Test>::get(0).unwrap();

		assert_eq!(answer_data.votes, 3);
	});
}
