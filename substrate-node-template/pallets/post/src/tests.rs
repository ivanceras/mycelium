use crate::{mock::*, Error};
use frame_support::BoundedSlice;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let content = BoundedVec::try_from(b"hello".to_vec()).unwrap();
		assert_ok!(PostModule::post_content(Origin::signed(1), content));
		// Read pallet storage and assert an expected result.
		assert_eq!(PostModule::total(), 1);
	});
}
