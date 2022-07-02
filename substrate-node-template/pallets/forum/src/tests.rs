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
		assert_ok!(ForumModule::comment_on(
			Origin::signed(2),
			0,
			None,
			"I'm a comment".to_string()
		));

		assert_ok!(ForumModule::comment_on(
			Origin::signed(2),
			0,
			None,
			"This is a second comment".to_string()
		));

		assert_ok!(ForumModule::comment_on(
			Origin::signed(3),
			1,
			None,
			"> I'm a comment \nThis".to_string()
		));

		assert_eq!(ForumModule::comment(0, 1), Some((comment, 2, None)));
		assert_eq!(ForumModule::kids(0), Some(BoundedVec::try_from(vec![1, 2]).unwrap()));
		assert_eq!(ForumModule::kids(1), Some(BoundedVec::try_from(vec![3]).unwrap()));
		assert_eq!(ForumModule::kids(2), None);
	});
}
