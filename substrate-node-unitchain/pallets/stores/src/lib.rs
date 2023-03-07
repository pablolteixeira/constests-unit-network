// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
mod types;
pub use types::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*, Origin};
	use frame_support::sp_runtime::traits::StaticLookup;
	// use types::*;
	use super::*;
	use frame_support::inherent::Vec;

	pub type BalanceOf<T> = <T as pallet_assets::Config>::Balance;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config  + pallet_assets::Config{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn order_id)]
	pub type OrderId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn store_id)]
	pub type StoreId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn product_id)]
	pub type ProductId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn all_stores)]
	pub type AllStores<T: Config> = StorageValue<_, Vec<StoreDetails<T::AccountId, T::AssetId>>>;

	#[pallet::storage]
	#[pallet::getter(fn all_order)]
	pub type AllOrders<T: Config> = StorageValue<_, Vec<OrderDetails<T::AccountId,T::AssetId>>>;

	#[pallet::storage]
	#[pallet::getter(fn all_products)]
	pub type AllProducts<T: Config> = StorageValue<_, Vec<ProductDetails<T::AccountId,BalanceOf<T>>>>;

	#[pallet::storage]
	#[pallet::getter(fn store_item)]
	pub(super) type StoreItem<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Option<StoreDetails<T::AccountId,T::AssetId>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn product_item)]
	pub(super) type ProductItem<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Option<ProductDetails<T::AccountId,BalanceOf<T>>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn order_item)]
	pub(super) type OrderItem<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Option<OrderDetails<T::AccountId,T::AssetId>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn store_products)]
	pub(super) type StoreProducts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Vec<ProductDetails<T::AccountId,BalanceOf<T>>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn products_orders)]
	pub(super) type ProductOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Vec<OrderDetails<T::AccountId,T::AssetId>>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
		// emits event with store owner, and store Id
		StoreCreated(T::AccountId, u32),
		// emits event with store owner, and store Id
		StoreModified(T::AccountId, u32),
		// emits productId
		ProductAdded(u32),
		// emits productId
		ProductModified(u32),
		// emits orderId
		OrderAdded(u32),
		// removed store
		RemovedStore(u32),
		// removed product
		RemovedProduct(u32),
		// added assetId to store
		AddedAssetId(u32, T::AssetId),
		OrderConfirmed(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
        /// Store Not Found
        StoreNotFound,
        /// ProductNotFound
        ProductNotFound,
        /// StorageOverflow
        StorageOverflow,
        /// AssetNotAcceptable
        AssetNotAcceptable,
        /// YouAreNotTheStoreOwner
        YouAreNotTheStoreOwner,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_order(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,   store_id : u32, product_id : u32 , product_price: u32 , stat_code: Vec<u8>, asset: T::AssetId, buyer_name: Vec<u8>, order_confirmed: bool) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);
            ensure!(ProductItem::<T>::contains_key(product_id),Error::<T>::ProductNotFound);

            let store = StoreItem::<T>::get(store_id).unwrap().unwrap();
            let mut asset_available_iter= store.assets.into_iter();
            ensure!(asset_available_iter.find(|&x| x == asset).is_some(), Error::<T>::AssetNotAcceptable);

            let order_id = OrderId::<T>::get().unwrap();
            let new_order_id = order_id + 1;
            let order  = OrderDetails {
                title: title,
                description: description,
                owner: _who,
                store_id: store_id,
                product_id: product_id,
                order_id: new_order_id,
                product_price: product_price,
                stat_code: stat_code,
                asset: asset,
                buyer_name: buyer_name,
                order_confirmed: order_confirmed,
                order_closed: false,
            };

            // add to all order
            let mut all_order = AllOrders::<T>::get().unwrap();
            all_order.push(order.clone());
            AllOrders::<T>::put(all_order);

            // add to order item
            OrderItem::<T>::insert(new_order_id, Some(order.clone()));

            // add order to store product
            let mut products_orders = ProductOrders::<T>::get(product_id).unwrap();
            products_orders.push(order);
            ProductOrders::<T>::insert(product_id, products_orders);

            // updating the order id
            OrderId::<T>::put(new_order_id);

            Self::deposit_event(Event::OrderAdded(new_order_id));
            Ok(())
        }		

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create_store(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,  username: Vec<u8>, asset_id: Option<T::AssetId>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let mut store_assets = Vec::new();
            if asset_id.is_some(){
                store_assets.push(asset_id.unwrap());
            }
            let current_store_id = StoreId::<T>::get().unwrap();
            let new_id = current_store_id + 1;
            let store = StoreDetails {
                title: title,
                description: description,
                owner: _who.clone(),
                username: username,
                assets: store_assets,
                store_id: new_id,
            };

            // add to all stores
            let mut all_stores = AllStores::<T>::get().unwrap();
            all_stores.push(store.clone());
            AllStores::<T>::put(all_stores);

            // add to store item
            StoreItem::<T>::insert(new_id, Some(store));

            // update the store id
            StoreId::<T>::put(new_id);

            Self::deposit_event(Event::StoreCreated(_who, new_id));

            Ok(())
        }


		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn add_product(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,  username: Vec<u8>, store_id : u32 , product_price: BalanceOf<T> , stat_code: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);

            let store = StoreItem::<T>::get(store_id).unwrap().unwrap();
            ensure!(store.owner == _who.clone(),Error::<T>::YouAreNotTheStoreOwner);

            let product_id = ProductId::<T>::get().unwrap();
            let new_product_id = product_id + 1;

            let product = ProductDetails {
                title: title,
                description: description,
                username: username,
                store_id: store_id,
                product_price: product_price,
                stat_code: stat_code,
                owner: _who,
                product_id: new_product_id
            };

            // add to all product
            let mut all_product = AllProducts::<T>::get().unwrap();
            all_product.push(product.clone());
            AllProducts::<T>::put(all_product);

            // add to product item
            ProductItem::<T>::insert(new_product_id, Some(product.clone()));

            // add products to store products
            let mut store_products = StoreProducts::<T>::get(store_id).unwrap();
            store_products.push(product);
            StoreProducts::<T>::insert(store_id, store_products);

            // updating the product id
            ProductId::<T>::put(new_product_id);

            Self::deposit_event(Event::ProductAdded(new_product_id));
            Ok(())
        }


		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn confirm_order(origin: OriginFor<T>, order_id : u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            // check if order exists
            ensure!(OrderItem::<T>::contains_key(order_id),Error::<T>::StoreNotFound);
            let _order = OrderItem::<T>::get(order_id).unwrap().unwrap();
            // check if store and product exists
            ensure!(StoreItem::<T>::contains_key(_order.store_id),Error::<T>::StoreNotFound);
            ensure!(ProductItem::<T>::contains_key(_order.product_id),Error::<T>::ProductNotFound);
            let store = StoreItem::<T>::get(_order.store_id).unwrap().unwrap();
            let product = ProductItem::<T>::get(_order.product_id).unwrap().unwrap();
            <pallet_assets::Pallet<T>>::transfer( Origin::<T>::Signed(store.owner.clone()).into()  , _order.asset.into() , T::Lookup::unlookup(_order.owner.clone()), product.product_price)?;
            
            let mut updated_order = OrderItem::<T>::get(order_id).unwrap().unwrap();
            updated_order.order_closed = true;
            
            // Update to all Orders
            let mut all_orders = AllOrders::<T>::get().unwrap();
            // have to remove old store 
            all_orders.retain(|x| *x != _order);
            all_orders.push(updated_order.clone());
            AllOrders::<T>::put(all_orders);

            // Update Product Orders
            let mut products_orders = ProductOrders::<T>::get(order_id).unwrap();
            products_orders.retain(|x| *x != _order);
            products_orders.push(updated_order.clone());
            ProductOrders::<T>::insert(order_id, products_orders);

            // update Order Item
            OrderItem::<T>::insert(order_id, Some(updated_order));

            Self::deposit_event(Event::OrderConfirmed(order_id));

            Ok(())
        }

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn edit_store(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>, store_id : u32, username: Vec<u8>, asset_id: Option<T::AssetId>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let mut store_assets = Vec::new();

            // check if store exists
            ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);

            let store_details = StoreItem::<T>::get(store_id).unwrap().unwrap();
            ensure!(store_details.owner == _who.clone(),Error::<T>::YouAreNotTheStoreOwner);

            if asset_id.is_some(){
                store_assets.push(asset_id.unwrap());
            }

            let store = StoreDetails {
                title: title,
                description: description,
                owner: _who.clone(),
                username: username,
                assets: store_assets,
                store_id: store_id,
            };

            // add to all stores
            let mut all_stores = AllStores::<T>::get().unwrap();
            // have to remove old store 
            let old_store = StoreItem::<T>::get(store_id).unwrap().unwrap();
            all_stores.retain(|x| *x != old_store);
            // adding new store
            all_stores.push(store.clone());
            AllStores::<T>::put(all_stores);

            // add to store item
            StoreItem::<T>::insert(store_id,Some(store));

            Self::deposit_event(Event::StoreModified(_who, store_id));

            Ok(())
        }

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn edit_product(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,  username: Vec<u8>, store_id : u32 ,product_id : u32 , product_price: BalanceOf<T> , stat_code: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);
            ensure!(ProductItem::<T>::contains_key(product_id),Error::<T>::ProductNotFound);

            let store_details = StoreItem::<T>::get(store_id).unwrap().unwrap();
            ensure!(store_details.owner == _who.clone(),Error::<T>::YouAreNotTheStoreOwner);

            let product = ProductDetails {
                title: title,
                description: description,
                username: username,
                store_id: store_id,
                product_price: product_price,
                stat_code: stat_code,
                owner: _who,
                product_id: product_id
            };

            // add to all product and remove the old one
            let mut all_products = AllProducts::<T>::get().unwrap();
            let old_product = ProductItem::<T>::get(product_id).unwrap().unwrap();
            all_products.retain(|x| *x != old_product);
            all_products.push(product.clone());
            AllProducts::<T>::put(all_products);

            // add to product item
            ProductItem::<T>::insert(product_id, Some(product.clone()));

            // add products to store products and remove the old one
            let mut store_products = StoreProducts::<T>::get(store_id).unwrap();
            store_products.retain(|x| *x != old_product);
            store_products.push(product);
            StoreProducts::<T>::insert(store_id, store_products);

            Self::deposit_event(Event::ProductModified(product_id));
            Ok(())
        }


        // Delete
		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_store(origin: OriginFor<T>, store_id : u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            // check if store exists
            ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);

            let store_details = StoreItem::<T>::get(store_id).unwrap().unwrap();
            ensure!(store_details.owner == _who.clone(),Error::<T>::YouAreNotTheStoreOwner);

            // removing from all stores
            let mut all_stores = AllStores::<T>::get().unwrap();
            // have to remove old store 
            let old_store = StoreItem::<T>::get(store_id).unwrap().unwrap();
            all_stores.retain(|x| *x != old_store);
            // updating the stores
            AllStores::<T>::put(all_stores);

            // remove store item
            StoreItem::<T>::remove(store_id);

            Self::deposit_event(Event::RemovedStore(store_id));
            Ok(())
        }

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_product(origin: OriginFor<T>,  store_id : u32 ,product_id : u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);
            ensure!(ProductItem::<T>::contains_key(product_id),Error::<T>::ProductNotFound);

            let store_details = StoreItem::<T>::get(store_id).unwrap().unwrap();
            ensure!(store_details.owner == _who.clone(),Error::<T>::YouAreNotTheStoreOwner);

            // remove from all product and remove the old one
            let mut all_products = AllProducts::<T>::get().unwrap();
            let old_product = ProductItem::<T>::get(product_id).unwrap().unwrap();
            all_products.retain(|x| *x != old_product);
            AllProducts::<T>::put(all_products);
            
            // remove products to store products and remove the old one
            let mut store_products = StoreProducts::<T>::get(store_id).unwrap();
            store_products.retain(|x| *x != old_product);
            StoreProducts::<T>::insert(store_id, store_products);
            
            // remove the product item
            ProductItem::<T>::remove(product_id);

            Self::deposit_event(Event::RemovedProduct(product_id));
            Ok(())
        }

		// add assetId to store 
		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn add_asset_to_store(origin: OriginFor<T>,  store_id : u32 , asset_id : T::AssetId) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			ensure!(StoreItem::<T>::contains_key(store_id),Error::<T>::StoreNotFound);
		
			let store_details = StoreItem::<T>::get(store_id).unwrap().unwrap();
			ensure!(store_details.owner == _who.clone(),Error::<T>::YouAreNotTheStoreOwner);
		
			// have to remove old store 
			let mut updated_store = StoreItem::<T>::get(store_id).unwrap().unwrap();
			updated_store.assets.push(asset_id);
		
			// add to all stores
			let mut all_stores = AllStores::<T>::get().unwrap();
			// have to remove old store 
			let old_store = StoreItem::<T>::get(store_id).unwrap().unwrap();
			all_stores.retain(|x| *x != old_store);
			// adding new store
			all_stores.push(updated_store.clone());
			AllStores::<T>::put(all_stores);
		
			// add to store item
			StoreItem::<T>::insert(store_id,Some(updated_store));
			// AddedAssetId
			Self::deposit_event(Event::AddedAssetId(store_id,asset_id));
			Ok(())
		}

	}
}