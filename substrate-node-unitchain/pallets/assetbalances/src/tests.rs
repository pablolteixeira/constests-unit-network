use crate::{mock::*};
use frame_support::{ assert_ok};

#[test] 
fn create_reward_and_get_works() {
	new_test_ext().execute_with(|| {
		// Assert that the correct event was deposited
		assert_ok!(Profile::create_user(RuntimeOrigin::signed(1), 1, "faizsarwar856@gmail.com".as_bytes().to_vec(), "faiz".as_bytes().to_vec(), "sarwar".as_bytes().to_vec(),
		"19-10-12".as_bytes().to_vec(), "this is bio ".as_bytes().to_vec(), "faizi".as_bytes().to_vec(), "".as_bytes().to_vec(), "".as_bytes().to_vec(), "".as_bytes().to_vec(),
		"".as_bytes().to_vec(), "".as_bytes().to_vec(), "".as_bytes().to_vec(), "".as_bytes().to_vec(), "".as_bytes().to_vec(), "".as_bytes().to_vec(), 1010, 10
		));

		// checking the storage 
		assert_eq!(Profile::user_item(1).unwrap().username , "faizi".as_bytes().to_vec() );
		assert_eq!(Profile::user_id(),Some(1));
	});
}

