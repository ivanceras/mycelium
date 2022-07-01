#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The maximum length a name may be.
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The post (post_id, (content, author))
	#[pallet::storage]
	#[pallet::getter(fn post)]
	pub type Post<T: Config> =
		StorageMap<_, Twox64Concat, u32, (BoundedVec<u8, T::MaxLength>, T::AccountId)>;

	/// The comment (post_id, comment_id, (content, author, parent_comment))
	#[pallet::storage]
	#[pallet::getter(fn comment)]
	pub type Comment<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		u32,
		Twox64Concat,
		u32,
		(BoundedVec<u8, T::MaxLength>, T::AccountId, Option<u32>),
	>;

	/// Keeps track of the item added into the system
	/// increments as more post or item is added
	#[pallet::storage]
	#[pallet::getter(fn item_counter)]
	pub(super) type ItemCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A post is submitted with post_id, and the author
		PostSubmitted(u32, T::AccountId),
		/// A comment is submmited with comment_id and the author
		CommentSubmitted(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add a post content to the storage, emit and event
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn post_content(
			origin: OriginFor<T>,
			content: BoundedVec<u8, T::MaxLength>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;

			// use the total number of items as post_id
			let post_id = ItemCounter::<T>::get();

			Post::<T>::insert(post_id, (content, who.clone()));
			// increment the item counter
			Self::increment_item_counter();
			// Emit a PostSubmitted event
			Self::deposit_event(Event::PostSubmitted(post_id, who));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn comment_on(
			origin: OriginFor<T>,
			post_id: u32,
			parent_comment: Option<u32>,
			content: BoundedVec<u8, T::MaxLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let comment_id = ItemCounter::<T>::get();
			Comment::<T>::insert(
				post_id,
				comment_id,
				(content.clone(), who.clone(), parent_comment),
			);
			Self::increment_item_counter();
			Self::deposit_event(Event::CommentSubmitted(post_id, who));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// increment th ItemCounter storage value
		fn increment_item_counter() {
			ItemCounter::<T>::mutate(|i| {
				*i = i.saturating_add(1);
			});
		}

		pub fn get_post(post_id: u32) -> Option<(BoundedVec<u8, T::MaxLength>, T::AccountId)> {
			log::info!("getting post_id: {}", post_id);
			Post::<T>::get(post_id)
		}
	}
}
