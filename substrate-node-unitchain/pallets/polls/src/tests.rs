use crate::{mock::*, Error, *};
use frame_support::{assert_noop, assert_ok};

#[test]
fn vote_without_balance_should_fail() {
	new_test_ext().execute_with(|| {
		// Creates poll
		let pid = begin_poll(1);
		// Try to vote without balance
		let v = 2u8;
		assert_noop!(
			Polls::vote(RuntimeOrigin::signed(1), PollCurrency::Native, pid, v),
			Error::<Test>::InsufficientFunds
		);
	});
}

#[test]
fn vote_with_balances_should_work() {
	new_test_ext().execute_with(|| {
		let voter = 2;
		// Creates poll
		set_balances(voter);

		let pid = begin_poll(1);

		// Vote on poll
		let v = 2u8; // 3rd option, base 0 index

		assert_ok!(Polls::vote(RuntimeOrigin::signed(voter), PollCurrency::Native, pid, v.clone()));

		assert_eq!(Balances::free_balance(voter), 20);

		// Assert vote has been recorded
		assert_eq!(Polls::poll_details_of(PollCurrency::Native, pid).unwrap().votes, Votes(vec![0, 0, 20, 0]));

		
		assert_noop!(
			Polls::vote(RuntimeOrigin::signed(voter), PollCurrency::Native, pid, 3u8),
			Error::<Test>::AlreadyVoted,
		);
		
		fast_forward_to(11);

		// Check if winning option is correct.
		let poll = Polls::poll_details_of(PollCurrency::Native, pid).unwrap();
		if let PollStatus::Finished { winning_option, end } = poll.status {
			assert_eq!(winning_option, Some(2));
			assert_eq!(end, 10);
		} else {
			panic!("poll not finished");
		}
	});
}

#[test]
fn emergency_cancel_should_work() {
	new_test_ext().execute_with(|| {
		let voter = 5;
		// Creates poll
		set_balances(voter);
		let pid = begin_poll(1);

		// Vote on poll
		let v = 2u8; // 3rd option, base 0 index

		assert_ok!(Polls::vote(RuntimeOrigin::signed(voter), PollCurrency::Native, pid, v.clone()));

		next_block();

		// Cancel poll
		assert_ok!(Polls::emergency_cancel(RuntimeOrigin::signed(1), PollCurrency::Native, pid));

		let poll = Polls::poll_details_of(PollCurrency::Native, pid).unwrap();

		// Check status
		assert_eq!(poll.status, PollStatus::Cancelled(3));

		let voter_2 = 6;
		assert_noop!(
			Polls::vote(RuntimeOrigin::signed(voter_2), PollCurrency::Native, pid, v.clone()),
			Error::<Test>::PollAlreadyFinished,
		);
	});
}

#[test]
fn vote_with_assets_should_work() {
	new_test_ext().execute_with(|| {
		let poll_creator = 1;
		let voter = 2;
		let voter_balance = 20;

		// Creates poll
		set_balances(voter);
		let (pid, _) = begin_poll_with_asset(poll_creator, voter, voter_balance);

		// Vote on poll
		let v = 3u8;
		assert_ok!(Polls::vote(RuntimeOrigin::signed(voter), PollCurrency::Asset(0), pid, v.clone()));

		next_block();

		// Finish poll
		fast_forward_to(10);
		assert_eq!(Polls::poll_count(), 1);

		// Check if winning option is correct.
		let poll = Polls::poll_details_of(PollCurrency::Asset(0), pid).unwrap();
		if let PollStatus::Finished { winning_option, end } = poll.status {
			assert_eq!(winning_option, Some(3));
			assert_eq!(end, 10);
		} else {
			panic!("poll not finished");
		}
	});
} 
