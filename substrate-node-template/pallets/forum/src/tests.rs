use crate::pallet;
use crate::{mock::*, Error};
use frame_support::BoundedSlice;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_posting_content() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let content = BoundedVec::try_from(b"hello".to_vec()).unwrap();
		// item 0
		assert_ok!(ForumModule::post_content(Origin::signed(1), content.clone()));
		// Read pallet storage and assert an expected result.
		println!("post: {:#?}", ForumModule::post(0));
		assert_eq!(ForumModule::item_counter(), 1);

		assert_eq!(ForumModule::get_post(0), Some((content, 1)));

		let comment = BoundedVec::try_from(b"I'm a comment".to_vec()).unwrap();
		assert_ok!(ForumModule::comment_on(Origin::signed(2), 0, None, comment.clone()));

		assert_eq!(ForumModule::comment(0, 1), Some((comment, 2, None)));
	});
}
