#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
mod types;
pub use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use frame_support::inherent::Vec;
	use super::*;
	pub type BalanceOf<T> = <T as pallet_assets::Config>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config{
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
	#[pallet::getter(fn transfer_items)]
	pub type TransferItems<T: Config> = StorageValue<_, Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>>>;

    #[pallet::storage]
	#[pallet::getter(fn usdu_id)]
	pub type USDUTokenId<T: Config> = StorageValue<_, T::AssetId>;

    #[pallet::storage]
	#[pallet::getter(fn exchange_staking_ownership_shares_item)]
	pub(super) type ExchangeStakingOwnershipSharesItems<T: Config>=  StorageValue<_, Vec<ExchangeStakingOwnershipShares<T::AccountId,T::AssetId>>>;

    #[pallet::storage]
	#[pallet::getter(fn exchange_share_transfers_items)]
	pub(super) type ExchangeShareTransfersItems<T: Config> = StorageValue<_, Vec<ExchangeShareTransfers<T::AccountId,T::AssetId>>>;

    #[pallet::storage]
	#[pallet::getter(fn user_feature_balances_items)]
	pub(super) type UserFeatureBalancesItems<T: Config> = StorageValue<_, Vec<UserFeatureBalances<T::AccountId,T::AssetId>>>;

    #[pallet::storage]
	#[pallet::getter(fn type_balances_item)]
	pub(super) type TypeBalancesItems<T: Config> = StorageValue<_, Vec<TypeBalances<T::AssetId>>>;


	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UserCreated{
            user : T::AccountId,
            id: u32
        },
        UpdatedPoolTokenBalance{
            currency: Vec<u8>,
             _token_id: T::AssetId
        },
        UpdatedPoolUsduBalance{
            currency: Vec<u8>,
            _token_id: T::AssetId
        },

    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        UserNotFound
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn check_and_update_user_balance(origin: OriginFor<T>, _token_id: T::AssetId) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // making sure storage value is not null
			if TransferItems::<T>::get().is_none(){
				let empty_vec : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = Vec::new();
				TransferItems::<T>::put(empty_vec);
			}

            let transfers = TransferItems::<T>::get().unwrap();
            let received_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = transfers.clone().into_iter().filter(|x| x.transfer_to_user_address == _who.clone()).collect();
            let spent_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> =  transfers.into_iter().filter(|x| x.transfer_from_user_address == _who.clone()).collect();

            let received_amount : u128 = received_transfers.iter().map(|s| s.amount).sum() ;
            let spent_amount: u128 =  spent_transfers.iter().map(|s| s.amount).sum() ;
            let _total_amount = received_amount - spent_amount;

            // we dont need to update anything as it is already done in assets pallet 

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn check_and_update_pool_token_balance(origin: OriginFor<T>, currency: Vec<u8>, _token_id: T::AssetId) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let token_type = "POOL".as_bytes().to_vec();

                        // making sure storage value is not null
			if TransferItems::<T>::get().is_none(){
				let empty_vec : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = Vec::new();
				TransferItems::<T>::put(empty_vec);
			}

            let mut transfers = TransferItems::<T>::get().unwrap();

            // only for this currency 
            transfers.retain(|x| x.currency == currency);
            transfers.retain(|x| x.transfer_from_feature == token_type || x.transfer_to_feature == token_type);
            transfers.retain(|x| x.transfer_from_feature_token_id == _token_id || x.transfer_to_feature_token_id == _token_id);

            let received_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = transfers.clone().into_iter().filter(|x| x.transfer_from_feature != token_type).collect();
            let spent_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> =  transfers.clone().into_iter().filter(|x| x.transfer_from_feature == token_type).collect();

            let received_amount : u128 = received_transfers.iter().map(|s| s.amount).sum() ;
            let spent_amount: u128 =  spent_transfers.iter().map(|s| s.amount).sum() ;
            let _total_amount = received_amount - spent_amount;

            // making sure storage value is not null
			if TypeBalancesItems::<T>::get().is_none(){
				let empty_vec : Vec<TypeBalances<T::AssetId>> = Vec::new();
				TypeBalancesItems::<T>::put(empty_vec);
			}

            // we dont need to update anything as it is already done in assets pallet 
            let mut type_balances_vec = TypeBalancesItems::<T>::get().unwrap();
            let type_balances = type_balances_vec.clone().into_iter().find(|x | x .currency == currency && x.token_id ==  _token_id );
            
            if type_balances.is_some() {
                
                let mut  type_balances_obj = type_balances.unwrap();   
                type_balances_vec.retain(|x| *x != type_balances_obj);

                type_balances_obj.balance = _total_amount;
                type_balances_vec.push(type_balances_obj);

                TypeBalancesItems::<T>::put(type_balances_vec);
            
            }
            else{

                let type_balances_obj= TypeBalances {
                    currency: currency.clone(),
                    token_type: token_type,
                    token_id: _token_id,
                    balance: _total_amount,
                };

                type_balances_vec.push(type_balances_obj);
                TypeBalancesItems::<T>::put(type_balances_vec);

            }

            Self::deposit_event(Event::UpdatedPoolTokenBalance{
                currency: currency,
                 _token_id: _token_id
            });

            Ok(())
        }


        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn check_and_update_pool_usdu_balance(origin: OriginFor<T>, currency: Vec<u8>, _token_id: T::AssetId) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let token_type = "POOL".as_bytes().to_vec();

                        // making sure storage value is not null
			if TransferItems::<T>::get().is_none(){
				let empty_vec : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = Vec::new();
				TransferItems::<T>::put(empty_vec);
			}

            let mut transfers = TransferItems::<T>::get().unwrap();

            // only for this currency 
            transfers.retain(|x| x.currency == currency);
            transfers.retain(|x| x.transfer_from_feature == token_type || x.transfer_to_feature == token_type);
            transfers.retain(|x| x.transfer_from_feature_token_id == _token_id || x.transfer_to_feature_token_id == _token_id);

            let received_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = transfers.clone().into_iter().filter(|x| x.transfer_from_feature != token_type).collect();
            let spent_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> =  transfers.clone().into_iter().filter(|x| x.transfer_from_feature == token_type).collect();

            let received_amount : u128 = received_transfers.iter().map(|s| s.amount).sum() ;
            let spent_amount: u128 =  spent_transfers.iter().map(|s| s.amount).sum() ;
            let _total_amount = received_amount - spent_amount;


            // making sure storage value is not null
			if TypeBalancesItems::<T>::get().is_none(){
				let empty_vec : Vec<TypeBalances<T::AssetId>> = Vec::new();
				TypeBalancesItems::<T>::put(empty_vec);
			}

            // we dont need to update anything as it is already done in assets pallet 
            let mut type_balances_vec = TypeBalancesItems::<T>::get().unwrap();
            let type_balances = type_balances_vec.clone().into_iter().find(|x | x .currency == currency && x.token_id ==  _token_id );
            
            if type_balances.is_some() {
                
                let mut  type_balances_obj = type_balances.unwrap();   
                type_balances_vec.retain(|x| *x != type_balances_obj);

                type_balances_obj.balance = _total_amount;
                type_balances_vec.push(type_balances_obj);

                TypeBalancesItems::<T>::put(type_balances_vec);
            
            }
            else{

                let type_balances_obj= TypeBalances {
                    currency: currency.clone(),
                    token_type: token_type,
                    token_id: _token_id,
                    balance: _total_amount,
                };

                type_balances_vec.push(type_balances_obj);
                TypeBalancesItems::<T>::put(type_balances_vec);

            }

            Self::deposit_event(Event::UpdatedPoolUsduBalance{
                currency: currency.clone(),
                 _token_id: _token_id
            });


            Ok(())
        }


        #[pallet::call_index(3)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn dcxupdatefeaturebalance(origin: OriginFor<T>, currency: Vec<u8>, _token_id: T::AssetId, token_type: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // making sure storage value is not null
			if TransferItems::<T>::get().is_none(){
				let empty_vec : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = Vec::new();
				TransferItems::<T>::put(empty_vec);
			}

            let mut transfers = TransferItems::<T>::get().unwrap();

            // only for this currency 
            transfers.retain(|x| x.currency == currency);
            transfers.retain(|x| x.transfer_from_feature == token_type || x.transfer_to_feature == token_type);
            transfers.retain(|x| x.transfer_from_feature_token_id == _token_id || x.transfer_to_feature_token_id == _token_id);

            let received_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = transfers.clone().into_iter().filter(|x| x.transfer_from_feature != token_type).collect();
            let spent_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> =  transfers.clone().into_iter().filter(|x| x.transfer_from_feature == token_type).collect();

            let received_amount : u128 = received_transfers.iter().map(|s| s.amount).sum() ;
            let spent_amount: u128 =  spent_transfers.iter().map(|s| s.amount).sum() ;
            let _total_amount = received_amount - spent_amount;


            // making sure storage value is not null
			if TypeBalancesItems::<T>::get().is_none(){
				let empty_vec : Vec<TypeBalances<T::AssetId>> = Vec::new();
				TypeBalancesItems::<T>::put(empty_vec);
			}

            // we dont need to update anything as it is already done in assets pallet 
            let mut type_balances_vec = TypeBalancesItems::<T>::get().unwrap();
            let type_balances = type_balances_vec.clone().into_iter().find(|x | x .currency == currency && x.token_id ==  _token_id );
            
            if type_balances.is_some() {
                
                let mut  type_balances_obj = type_balances.unwrap();   
                type_balances_vec.retain(|x| *x != type_balances_obj);

                type_balances_obj.balance = _total_amount;
                type_balances_vec.push(type_balances_obj);

                TypeBalancesItems::<T>::put(type_balances_vec);
            
            }
            else{

                let type_balances_obj= TypeBalances {
                    currency: currency,
                    token_type: token_type,
                    token_id: _token_id,
                    balance: _total_amount,
                };

                type_balances_vec.push(type_balances_obj);
                TypeBalancesItems::<T>::put(type_balances_vec);

            }

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn dcxupdateuserfeaturebalance(origin: OriginFor<T>, currency: Vec<u8>, _token_id: T::AssetId, token_type: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // making sure storage value is not null
			if TransferItems::<T>::get().is_none(){
				let empty_vec : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = Vec::new();
				TransferItems::<T>::put(empty_vec);
			}

            let mut transfers = TransferItems::<T>::get().unwrap();
            transfers.retain(|x| x.currency == currency);
            transfers.retain(|x| x.transfer_from_user_address == _who.clone() || x.transfer_to_user_address == _who.clone());
            transfers.retain(|x| x.transfer_from_feature == token_type || x.transfer_to_feature == token_type);
            transfers.retain(|x| x.transfer_from_feature_token_id == _token_id || x.transfer_to_feature_token_id == _token_id);

            let received_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> = transfers.clone().into_iter().filter(|x| x.transfer_from_feature != token_type).collect();
            let spent_transfers : Vec<Transfers<T::AccountId,T::AssetId,BalanceOf<T>>> =  transfers.clone().into_iter().filter(|x| x.transfer_from_feature == token_type).collect();

            let received_amount : u128 = received_transfers.iter().map(|s| s.amount).sum() ;
            let spent_amount: u128 =  spent_transfers.iter().map(|s| s.amount).sum() ;
            let _total_amount = received_amount - spent_amount;

            // making sure storage value is not null
			if UserFeatureBalancesItems::<T>::get().is_none(){
				let empty_vec : Vec<UserFeatureBalances<T::AccountId,T::AssetId>> = Vec::new();
				UserFeatureBalancesItems::<T>::put(empty_vec);
			}

            let mut user_feature_balances = UserFeatureBalancesItems::<T>::get().unwrap();

            let res = user_feature_balances.clone().into_iter().find(|x | x .currency == currency && x.token_id ==  _token_id && x.token_type == token_type && x.user_address == _who.clone());

            if res.is_some(){
                let mut result = res.unwrap();  

                user_feature_balances.retain(|x| *x != result);

                result.balance = _total_amount;
                user_feature_balances.push(result);

                UserFeatureBalancesItems::<T>::put(user_feature_balances);
            }
            else{
                let feature_obj = UserFeatureBalances {
                    currency: currency,
                    token_type: token_type,
                    token_id: _token_id,
                    balance: _total_amount,
                    user_address: _who.clone(),
                };

                user_feature_balances.push(feature_obj);
                UserFeatureBalancesItems::<T>::put(user_feature_balances);
            }
            
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn check_and_update_user_share_balance(origin: OriginFor<T>, _token_id: T::AssetId) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // making sure storage value is not null
			if ExchangeShareTransfersItems::<T>::get().is_none(){
				let empty_vec : Vec<ExchangeShareTransfers<T::AccountId,T::AssetId>> = Vec::new();
				ExchangeShareTransfersItems::<T>::put(empty_vec);
			}

            let mut _exchange_share_transfers = ExchangeShareTransfersItems::<T>::get().unwrap();
            _exchange_share_transfers.retain(|x| x.token_id == _token_id );
            _exchange_share_transfers.retain(|x| x.transfer_from_user_address == _who.clone() || x.transfer_to_user_address == _who.clone());

            let received_transfers : Vec<ExchangeShareTransfers<T::AccountId,T::AssetId>> = _exchange_share_transfers.clone().into_iter().filter(|x| x.transfer_from_user_address != _who.clone()).collect();
            let spent_transfers : Vec<ExchangeShareTransfers<T::AccountId,T::AssetId>> =  _exchange_share_transfers.clone().into_iter().filter(|x| x.transfer_from_user_address == _who.clone()).collect();

            let received_amount : u128 = received_transfers.iter().map(|s| s.amount).sum() ;
            let spent_amount: u128 =  spent_transfers.iter().map(|s| s.amount).sum() ;
            let _total_amount = received_amount - spent_amount;


            // making sure storage value is not null
			if ExchangeStakingOwnershipSharesItems::<T>::get().is_none(){
				let empty_vec : Vec<ExchangeStakingOwnershipShares<T::AccountId,T::AssetId>> = Vec::new();
				ExchangeStakingOwnershipSharesItems::<T>::put(empty_vec);
			}

            let mut exchange_staking_ownership_shares_items = ExchangeStakingOwnershipSharesItems::<T>::get().unwrap();

            let res = exchange_staking_ownership_shares_items.clone().into_iter().find(|x | x.token_id ==  _token_id  && x.user_address == _who.clone());

            if res.is_some(){
                let mut result = res.unwrap();
                exchange_staking_ownership_shares_items.retain(|x| *x != result);

                result.ownership_shares = _total_amount;
                exchange_staking_ownership_shares_items.push(result);
                ExchangeStakingOwnershipSharesItems::<T>::put(exchange_staking_ownership_shares_items);
            }else{

                let feature_obj = ExchangeStakingOwnershipShares {
                    token_id: _token_id,
                    user_address: _who.clone(),
                    ownership_shares: _total_amount
                };

                exchange_staking_ownership_shares_items.push(feature_obj);
                ExchangeStakingOwnershipSharesItems::<T>::put(exchange_staking_ownership_shares_items);
            }
            
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn set_usdu_token_id(origin: OriginFor<T>, _token_id: T::AssetId) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            USDUTokenId::<T>::put(_token_id);

            Ok(())
        }

   }

    impl<T: Config> Pallet<T> {

        pub fn u32_to_asset_balance(input: u32) -> BalanceOf<T> {
            input.try_into().ok().unwrap()
        }    

        pub fn asset_balance_to_u128(input: BalanceOf<T>) -> u128 {
            TryInto::<u128>::try_into(input).ok().unwrap()
        }
    

        pub fn dcxgettypebalance(currency: Vec<u8>, _token_id: T::AssetId, token_type: Vec<u8>)-> u128 {
            // making sure storage value is not null
			if TypeBalancesItems::<T>::get().is_none(){
				let empty_vec : Vec<TypeBalances<T::AssetId>> = Vec::new();
				TypeBalancesItems::<T>::put(empty_vec);
			}

            let type_balance = TypeBalancesItems::<T>::get().unwrap();
            let res = type_balance.clone().into_iter().find(|x | x .currency == currency && x.token_id ==  _token_id && x.token_type == token_type);
            if res.is_some(){
                let result = res.unwrap();
                result.balance
            }else{
                0
            }
        }

        pub fn dcxgetindivtokenbalance(user : T::AccountId, token_id: T::AssetId)-> u128{
           let balance =  <pallet_assets::Pallet<T>>::balance(token_id, user );
           Self::asset_balance_to_u128(balance)
        }

        pub fn dcxgetindivuserfeaturebalance(user : T::AccountId , currency: Vec<u8>, _token_id: T::AssetId, token_type: Vec<u8>)-> u128{
            // making sure storage value is not null
			if UserFeatureBalancesItems::<T>::get().is_none(){
				let empty_vec :  Vec<UserFeatureBalances<T::AccountId,T::AssetId>> = Vec::new();
				UserFeatureBalancesItems::<T>::put(empty_vec);
			}

            let type_balance = UserFeatureBalancesItems::<T>::get().unwrap();
            let res = type_balance.clone().into_iter().find(|x | x .currency == currency && x.token_id ==  _token_id && x.token_type == token_type && x.user_address == user);
            if res.is_some(){
                let result = res.unwrap();
                result.balance
            }else{
                0
            }
        }
 
        pub fn getstakingbalances(user : T::AccountId, _token_id: T::AssetId, currency: Vec<u8>)-> (u128,u128){
            let usdu_id= USDUTokenId::<T>::get().unwrap();
            let _user_token_balance = Self::dcxgetindivtokenbalance(user.clone(),_token_id);
            // 
            let _user_usdu_balance = Self::dcxgetindivtokenbalance(user.clone(),usdu_id);  //

            let _total_user_token_amount_in_pool = Self::dcxgetindivuserfeaturebalance(user.clone(), currency.clone() ,_token_id, "POOL".as_bytes().to_vec()); 
            // 
            let _total_user_usdu_amount_in_pool = Self::dcxgetindivuserfeaturebalance(user.clone(), "USDU".as_bytes().to_vec() ,usdu_id, "POOL".as_bytes().to_vec());  

            let total_token_amount_in_pool= Self::dcxgettypebalance(currency.clone(), _token_id, "POOL".as_bytes().to_vec());
            // 
            let total_usdu_amount_in_pool= Self::dcxgettypebalance(currency.clone(), usdu_id, "POOL".as_bytes().to_vec());

            let _total_amount= total_usdu_amount_in_pool + total_token_amount_in_pool;

            if total_usdu_amount_in_pool > 0 && total_token_amount_in_pool > 0{
                let token_in_usd = total_token_amount_in_pool / total_usdu_amount_in_pool;
                let usd_in_token = total_usdu_amount_in_pool / total_token_amount_in_pool;
                (token_in_usd, usd_in_token )
            }
            else{
                let token_in_usd = 0;
                let usd_in_token = 0;
                (token_in_usd, usd_in_token )
            }
        }

        pub fn dcxgetuserfeaturebalances(_token_id: T::AssetId, token_type: Vec<u8>)-> Vec<UserFeatureBalances<T::AccountId,T::AssetId>>{
            // making sure storage value is not null
			if UserFeatureBalancesItems::<T>::get().is_none(){
				let empty_vec :  Vec<UserFeatureBalances<T::AccountId,T::AssetId>> = Vec::new();
				UserFeatureBalancesItems::<T>::put(empty_vec);
			}

            let type_balance = UserFeatureBalancesItems::<T>::get().unwrap();
            let res = type_balance.clone().into_iter().filter(|x| x.token_id ==  _token_id && x.token_type == token_type).collect();
            res
        }

    }    
}

// have to add events 
// helper functions 