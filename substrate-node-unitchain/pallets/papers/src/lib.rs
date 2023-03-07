#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::EncodeLike;

use frame_support::{
	sp_runtime::{
		 FixedPointOperand,
	},
	traits::{
		fungibles::{Create, Inspect, Transfer, InspectMetadata, Mutate},
		tokens::{Balance},
		Currency,
	},
	dispatch::fmt::Debug,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		type GenericId: MaybeSerializeDeserialize
					+ MaxEncodedLen
					+ TypeInfo
					+ Clone
					+ Debug
					+ PartialEq
					+ EncodeLike
					+ Decode;

		type AssetId: MaybeSerializeDeserialize
					+ MaxEncodedLen
					+ TypeInfo
					+ Clone
					+ Debug
					+ PartialEq
					+ EncodeLike
					+ Decode;

		type Assets: Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::AssetBalance>
					+ InspectMetadata<Self::AccountId>
					+ Transfer<Self::AccountId>
					+ Create<Self::AccountId>
					+ Mutate<Self::AccountId>;

		type AssetBalance: Balance
					+ FixedPointOperand
					+ MaxEncodedLen
					+ MaybeSerializeDeserialize;

		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type TitleLimit: Get<u32>;

		#[pallet::constant]
		type TextLimit: Get<u32>;
	}
	
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	pub type AssetBalanceOf<T> = <T as Config>::AssetBalance;

	pub type AssetIdOf<T> = <T as Config>::AssetId;

	pub type PositionOf<T> = <T as Config>::GenericId;

	pub type PaperIdOf<T> = <T as Config>::GenericId;  	 
	

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, PartialEq, Default)]
	#[scale_info(skip_type_params(T))]
	pub struct Paper<T: Config, AssetId, Position, AccountId, PaperId> {
		pub asset_id: AssetId,
		pub position: Position,
		pub title: BoundedVec<u8, T::TitleLimit>,
		pub text: BoundedVec<u8, T::TextLimit>,
		pub user_address: AccountId,
		pub paper_id: PaperId
	}	

	impl<T: Config> Clone for Paper<T, AssetIdOf<T>, PositionOf<T>, AccountIdOf<T>, PaperIdOf<T>> {
        fn clone(&self) -> Self {
            Self {
                asset_id: self.asset_id.clone(),
				position: self.position.clone(),
				title: self.title.clone(),
				text: self.text.clone(),
				user_address: self.user_address.clone(),
				paper_id: self.paper_id.clone()
            }
        }
    } 

	pub type PaperOf<T> = Paper<T, AssetIdOf<T>, PositionOf<T>, AccountIdOf<T>, PaperIdOf<T>>;
	
	pub type PapersOf<T> = BoundedVec<PaperOf<T>, ConstU32::<100>>;

	// storage just for papers - account_id -> asset_id -> paper
	#[pallet::storage]
	#[pallet::getter(fn get_assets_papers)]
	pub(super) type AssetIdMap<T: Config> = StorageDoubleMap<_, Blake2_128Concat, AccountIdOf<T>, Blake2_128Concat, AssetIdOf<T>, PapersOf<T>>;
	
	// storage just for papers - account_id -> paper_id -> paper
	#[pallet::storage]
	#[pallet::getter(fn get_id_papers)]
	pub(super) type PaperIdMap<T: Config> = StorageDoubleMap<_, Blake2_128Concat, AccountIdOf<T>, Blake2_128Concat, PaperIdOf<T>, PaperOf<T>>;

	// storage just for subpapers - account_id -> paper_id -> vec<subspapers>
	#[pallet::storage]
	#[pallet::getter(fn get_subpaper)]
	pub(super) type SubPapersMap<T: Config> = StorageDoubleMap<_, Blake2_128Concat, AccountIdOf<T>, Blake2_128Concat, PaperIdOf<T>, PapersOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PaperCreated { who: T::AccountId, paper_id: PaperIdOf<T>, position: PositionOf<T> },
		PaperUpdated { who: T::AccountId, paper_id: PaperIdOf<T>, position: PositionOf<T>, title: BoundedVec<u8, T::TitleLimit>, text: BoundedVec<u8, T::TextLimit> },
	}
	
	#[pallet::error]
	pub enum Error<T> {
		AssetAlreadyExist,
		AssetDontExist,
		AssetDoesNotHavePaper,
		PaperIdDontExist,
		VectorFull,
		VectorErrorInsert
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn create_paper(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			paper_id: PaperIdOf<T>,
			position: PositionOf<T>,
			title: BoundedVec<u8, T::TitleLimit>,
			text: BoundedVec<u8, T::TextLimit>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(T::Assets::asset_exists(asset_id.clone()), Error::<T>::AssetDontExist);

			let paper = 
				Paper::<T, AssetIdOf<T>, PositionOf<T>, AccountIdOf<T>, PaperIdOf<T>> 
				{
					asset_id: asset_id.clone(),
					position: position.clone(),
					title: title,
					text: text,
					user_address: who.clone(),
					paper_id: paper_id.clone()
				};
			
			let mut vector_sub_papers: BoundedVec<PaperOf<T>, ConstU32::<100>>;
			let mut vector_assets_papers: BoundedVec<PaperOf<T>, ConstU32::<100>>;	

			if PaperIdMap::<T>::contains_key(who.clone(), paper_id.clone()) {
				// Unwrap function used but is sure that the value exist, so it will never panic. 
				vector_sub_papers = SubPapersMap::<T>::get(who.clone(), paper_id.clone()).unwrap();
				
				vector_assets_papers = AssetIdMap::<T>::get(who.clone(), asset_id.clone()).unwrap();

				vector_sub_papers.try_push(paper.clone()).map_err(|_| Error::<T>::VectorFull)?;
			} else {
				PaperIdMap::<T>::insert(who.clone(), paper_id.clone(), paper.clone());

				if AssetIdMap::<T>::contains_key(who.clone(), asset_id.clone()) {
					// Unwrap function used but is sure that the value exist, so it will never panic. 
					vector_assets_papers = AssetIdMap::<T>::get(who.clone(), asset_id.clone()).unwrap();
				} else {
					vector_assets_papers = Default::default();
				}

				vector_sub_papers = Default::default();

				vector_assets_papers.try_push(paper.clone()).map_err(|_| Error::<T>::VectorFull)?;
			}
			
			SubPapersMap::<T>::insert(who.clone(), paper_id.clone(), vector_sub_papers);
			AssetIdMap::<T>::insert(who.clone(), asset_id.clone(), vector_assets_papers);

			Self::deposit_event(Event::<T>::PaperCreated { who, paper_id, position});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_paper(
			origin: OriginFor<T>,
			paper_id: PaperIdOf<T>,
			position: PositionOf<T>,
			title: BoundedVec<u8, T::TitleLimit>,
			text: BoundedVec<u8, T::TextLimit>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(PaperIdMap::<T>::contains_key(who.clone(), paper_id.clone()), Error::<T>::PaperIdDontExist);

			let mut paper_updated = PaperIdMap::<T>::get(who.clone(), paper_id.clone()).unwrap();

			paper_updated.position = position.clone();
			paper_updated.title = title.clone();
			paper_updated.text = text.clone();

			PaperIdMap::<T>::insert(who.clone(), paper_id.clone(), paper_updated);

			Self::deposit_event(Event::<T>::PaperUpdated { who, paper_id, position, title, text });

			Ok(())
		}	
	}
}
