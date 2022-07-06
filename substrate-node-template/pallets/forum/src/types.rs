use crate::Config;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

#[derive(Encode, Decode, TypeInfo, RuntimeDebug)]
#[scale_info(skip_type_params(T))]
pub struct PostContent<T: Config> {
	post_id: u32,
	content: BoundedVec<u8, T::MaxContentLength>,
	author: T::AccountId,
}

impl<T: Config> MaxEncodedLen for PostContent<T> {
	fn max_encoded_len() -> usize {
		<(u32, BoundedVec<u8, T::MaxContentLength>, T::AccountId)>::max_encoded_len()
	}
}

impl<T: Config> PartialEq for PostContent<T> {
	fn eq(&self, other: &Self) -> bool {
		self.post_id == other.post_id
			&& self.content == other.content
			&& self.author == other.author
	}
}

impl<T: Config> PostContent<T> {
	pub fn new(
		post_id: u32,
		content: BoundedVec<u8, T::MaxContentLength>,
		author: T::AccountId,
	) -> Self {
		Self { post_id, content, author }
	}
}

#[derive(Encode, Decode, TypeInfo, RuntimeDebug)]
#[scale_info(skip_type_params(T))]
pub struct CommentContent<T: Config> {
	comment_id: u32,
	content: BoundedVec<u8, T::MaxContentLength>,
	author: T::AccountId,
	parent_item: Option<u32>,
}

impl<T: Config> MaxEncodedLen for CommentContent<T> {
	fn max_encoded_len() -> usize {
		<(u32, BoundedVec<u8, T::MaxContentLength>, T::AccountId, Option<u32>)>::max_encoded_len()
	}
}

impl<T: Config> CommentContent<T> {
	pub fn new(
		comment_id: u32,
		content: BoundedVec<u8, T::MaxContentLength>,
		author: T::AccountId,
		parent_item: Option<u32>,
	) -> Self {
		Self { comment_id, content, author, parent_item }
	}
}

impl<T: Config> PartialEq for CommentContent<T> {
	fn eq(&self, other: &Self) -> bool {
		self.comment_id == other.comment_id
			&& self.content == other.content
			&& self.author == other.author
			&& self.parent_item == other.parent_item
	}
}
