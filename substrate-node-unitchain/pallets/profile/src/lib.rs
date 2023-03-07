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
	#[pallet::getter(fn user_item)]
	pub(super) type UserItem<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Users<T::AccountId , BalanceOf<T>>,
	>;


	#[pallet::storage]
	#[pallet::getter(fn all_follows)]
	pub type AllFollows<T: Config> = StorageValue<_, Vec<Follows<T::AccountId>>>;

    #[pallet::storage]
	#[pallet::getter(fn all_messages)]
	pub type AllMessage<T: Config> = StorageValue<_, Vec<Message<T::AccountId>>>;

    #[pallet::storage]
	#[pallet::getter(fn all_connections)]
	pub type AllConnection<T: Config> = StorageValue<_, Vec<Connections<T::AccountId>>>;

	#[pallet::storage]
	#[pallet::getter(fn user_id)]
	pub type UserId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn all_users)]
	pub type AllUsers<T: Config> = StorageValue<_, Vec<Users<T::AccountId , BalanceOf<T>>>>;

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
        UserUpdated{
            user : T::AccountId,
            id: u32
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
        pub fn create_user(origin: OriginFor<T>, user_address: T::AccountId , email: Vec<u8>, first_name: Vec<u8>, last_name: Vec<u8>, date_of_birth: Vec<u8>,
            bio: Vec<u8>, username: Vec<u8> , website: Vec<u8>, linkedin: Vec<u8>, twitter: Vec<u8>, instagram: Vec<u8>
            ,telegram: Vec<u8>, youtube_url: Vec<u8>, facebook: Vec<u8>, vision: Vec<u8>, tag_line: Vec<u8> , pin_code: u32, exchange_volume: u32 , current_time : Vec<u8>
        ) -> DispatchResult {

            let _who = ensure_signed(origin)?;

            // making sure storage value is not null
			if UserId::<T>::get().is_none(){
				UserId::<T>::put(0)
			}
            
            let current_user_id = UserId::<T>::get().unwrap();
            let new_id = current_user_id + 1;

            let user = Users {
                user_address: user_address.clone(),
                first_name: first_name,
                last_name: last_name,
                date_of_birth: date_of_birth,
                bio: bio,
                email: email,
                created_at:  current_time.clone(),
                updated_at:  current_time.clone(),
                username: username,
                location: "".as_bytes().to_vec(),
                last_seen_at:  current_time.clone(),
                token_balance: Self::u32_to_asset_balance(0),   
                language_code: "en".as_bytes().to_vec(),
                invited_by_user_id: 0,
                startprice: 0,
                website: website,
                linkedin: linkedin,
                twitter: twitter,
                instagram: instagram,
                telegram: telegram,
                youtube_url: youtube_url,
                facebook: facebook,
                vision: vision,
                tag_line: tag_line,
                unit_balance: 0,
                unit_sent: 0,
                unit_received: 0,
                total_deposited_at_time_usd: 0,
                total_deposited_now_usd: 0,
                total_withdrawn_at_time_usd: 0,
                total_withdrawn_now_usd: 0,
                exchange_volume: exchange_volume,
                pin_code: pin_code,
                following_count: 0,
                follower_count: 0,
                user_id: new_id
            };

            // making sure storage value is not null
			if AllUsers::<T>::get().is_none(){
				let empty_vec : Vec<Users<T::AccountId, BalanceOf<T>>> = Vec::new();
				AllUsers::<T>::put(empty_vec);
			}

            // add to all users
            let mut all_users = AllUsers::<T>::get().unwrap();
            all_users.push(user.clone());
            AllUsers::<T>::put(all_users);

            // add to user item
            UserItem::<T>::insert(new_id, user);

            // update the user id
            UserId::<T>::put(new_id);

            Self::deposit_event(Event::UserCreated{
                user : user_address,
                id: new_id
            });

            Ok(())
        }


        #[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn update_user(origin: OriginFor<T>, user_id: u32 ,email: Vec<u8>, first_name: Vec<u8>, last_name: Vec<u8>, date_of_birth: Vec<u8>,
            bio: Vec<u8>, username: Vec<u8> , website: Vec<u8>, linkedin: Vec<u8>, twitter: Vec<u8>, instagram: Vec<u8>
            ,telegram: Vec<u8>, youtube_url: Vec<u8>, facebook: Vec<u8>, vision: Vec<u8>, tag_line: Vec<u8> , pin_code: u32, exchange_volume: u32 , current_time: Vec<u8>
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // check if user exists
            ensure!(UserItem::<T>::contains_key(user_id),Error::<T>::UserNotFound);

            let user_details = UserItem::<T>::get(user_id).unwrap();
            ensure!(user_details.user_address == _who.clone(),Error::<T>::UserNotFound);

            let user =  Users {
                user_address: user_details.user_address.clone(),
                first_name: first_name,
                last_name: last_name,
                date_of_birth: date_of_birth,
                bio: bio,
                email: email,
                created_at: current_time.clone(),
                updated_at: current_time.clone(),
                username: username,
                location: "".as_bytes().to_vec(),
                last_seen_at:  current_time.clone(),
                token_balance: Self::u32_to_asset_balance(0),   
                language_code: "en".as_bytes().to_vec(),
                invited_by_user_id: 0,
                startprice: 0,
                website: website,
                linkedin: linkedin,
                twitter: twitter,
                instagram: instagram,
                telegram: telegram,
                youtube_url: youtube_url,
                facebook: facebook,
                vision: vision,
                tag_line: tag_line,
                unit_balance: 0,
                unit_sent: 0,
                unit_received: 0,
                total_deposited_at_time_usd: 0,
                total_deposited_now_usd: 0,
                total_withdrawn_at_time_usd: 0,
                total_withdrawn_now_usd: 0,
                exchange_volume: exchange_volume,
                pin_code: pin_code,
                following_count: 0,
                follower_count: 0,
                user_id: user_id
            };

            // add to all users
            let mut all_users = AllUsers::<T>::get().unwrap();
            // have to remove old user 
            let old_user = UserItem::<T>::get(user_id).unwrap();
            all_users.retain(|x| *x != old_user);
            // adding new user
            all_users.push(user.clone());
            AllUsers::<T>::put(all_users);

            // add to user item
            UserItem::<T>::insert(user_id,user);
            // UserUpdated
            Self::deposit_event(Event::UserCreated{
                user : user_details.user_address,
                id: user_id
            });
            Ok(())
        }


        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn follow_user(origin: OriginFor<T>, _follow_from_user_address: T::AccountId, 
            _follow_to_user_address: T::AccountId, follow_from_token_symbol: Vec<u8>, follow_to_token_symbol: Vec<u8>, current_time: Vec<u8>
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut all_users = AllUsers::<T>::get().unwrap();

            // removing old _follow_from_user_address details form all user 
            let mut _user = all_users.iter().find(|x| x.user_address == _follow_from_user_address);
            ensure!(_user.is_some() , Error::<T>::UserNotFound);
            let follow_form_user_id = _user.unwrap().user_id;
            let mut follow_form_user_object = UserItem::<T>::get(follow_form_user_id).unwrap();
            // have to remove old user
            all_users.retain(|x| *x != follow_form_user_object);

            // removing old _follow_from_user_address details form all user 
            let mut _user = all_users.iter().find(|x| x.user_address == _follow_to_user_address);
            ensure!(_user.is_some() , Error::<T>::UserNotFound);
            let _follow_to_user_id = _user.unwrap().user_id;
            let mut follow_to_user_object = UserItem::<T>::get(_follow_to_user_id).unwrap();
            // have to remove old user
            all_users.retain(|x| *x != follow_to_user_object);

            // updating followers and following count 
            follow_form_user_object.following_count += 1;
            follow_to_user_object.follower_count += 1;

            // updating the storage 
            UserItem::<T>::insert(follow_form_user_id, follow_form_user_object.clone());
            UserItem::<T>::insert(_follow_to_user_id, follow_to_user_object.clone());

            // updating all_vec storage 
            all_users.push(follow_form_user_object);
            all_users.push(follow_to_user_object);

            AllUsers::<T>::put(all_users);

            // 
            let follow_item = Follows {
                follow_from_user_address: _follow_from_user_address,
                follow_to_user_address: _follow_to_user_address,
                follow_from_token_symbol: follow_from_token_symbol,
                follow_to_token_symbol: follow_to_token_symbol,
                created_at: current_time.clone(),
                updated_at: current_time.clone()
            };

            // making sure storage value is not null
			if AllFollows::<T>::get().is_none(){
				let empty_vec : Vec<Follows<T::AccountId>> = Vec::new();
				AllFollows::<T>::put(empty_vec);
			}


            let mut all_follows =  AllFollows::<T>::get().unwrap();
            all_follows.push(follow_item);

            AllFollows::<T>::put(all_follows);

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn unfollow_user(origin: OriginFor<T>, _follow_from_user_address: T::AccountId,_follow_to_user_address: T::AccountId ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut all_users = AllUsers::<T>::get().unwrap();

            // removing old _follow_from_user_address details form all user 
            let mut _user = all_users.iter().find(|x| x.user_address == _follow_from_user_address);
            ensure!(_user.is_some() , Error::<T>::UserNotFound);
            let follow_form_user_id = _user.unwrap().user_id;
            let mut follow_form_user_object = UserItem::<T>::get(follow_form_user_id).unwrap();
            // have to remove old user
            all_users.retain(|x| *x != follow_form_user_object);

            // removing old _follow_from_user_address details form all user 
            let mut _user = all_users.iter().find(|x| x.user_address == _follow_to_user_address);
            ensure!(_user.is_some() , Error::<T>::UserNotFound);
            let _follow_to_user_id = _user.unwrap().user_id;
            let mut follow_to_user_object = UserItem::<T>::get(_follow_to_user_id).unwrap();
            // have to remove old user
            all_users.retain(|x| *x != follow_to_user_object);

            // updating followers and following count 
            follow_form_user_object.following_count -= 1;
            follow_to_user_object.follower_count -= 1;

            // updating the storage 
            UserItem::<T>::insert(follow_form_user_id, follow_form_user_object.clone());
            UserItem::<T>::insert(_follow_to_user_id, follow_to_user_object.clone());

            // updating all_vec storage 
            all_users.push(follow_form_user_object);
            all_users.push(follow_to_user_object);

            AllUsers::<T>::put(all_users);

            let mut all_follows =  AllFollows::<T>::get().unwrap();

            all_follows.retain(|x| x.follow_from_user_address == _follow_from_user_address && x.follow_to_user_address == _follow_to_user_address);
            AllFollows::<T>::put(all_follows);

            Ok(())
        }


        #[pallet::call_index(4)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn send_message(origin: OriginFor<T>, _tousername: Vec<u8> , _messagefrom: T::AccountId, _messageto: T::AccountId , message: Vec<u16>, current_time: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let all_users = AllUsers::<T>::get().unwrap();

            // removing old _follow_from_user_address details form all user 
            let _user1 = all_users.iter().find(|x| x.user_address == _messagefrom);
            ensure!(_user1.is_some() , Error::<T>::UserNotFound);
            let _message_form_user_object = _user1.unwrap();

            // removing old _follow_from_user_address details form all user 
            let _user2 = all_users.iter().find(|x| x.user_address == _messageto);
            ensure!(_user2.is_some() , Error::<T>::UserNotFound);
            let _message_to_user_object = _user2.unwrap();

            let _message = Message {
                message_from: _messagefrom.clone(),
                message_to: _messageto.clone(),
                message: message.clone(),
                created_at: current_time.clone(),
                updated_at: current_time.clone()
            };

            // making sure storage value is not null
			if AllMessage::<T>::get().is_none(){
				let empty_vec : Vec<Message<T::AccountId>> = Vec::new();
				AllMessage::<T>::put(empty_vec);
			}


            let mut all_message = AllMessage::<T>::get().unwrap();
            all_message.push(_message);

            // making sure storage value is not null
			if AllConnection::<T>::get().is_none(){
				let empty_vec : Vec<Connections<T::AccountId>> = Vec::new();
				AllConnection::<T>::put(empty_vec);
			}

            let mut _all_connection = AllConnection::<T>::get().unwrap();
            let connection = _all_connection.iter().find(|x| x.connection_from_user_address == _messagefrom && x.connection_to_user_address == _messageto);

            if connection.is_none(){
                let _connection1 = Connections {
                    connection_from_user_address: _messagefrom.clone(),
                    connection_to_user_address: _messageto.clone(),
                    last_message: message.clone(),
                    last_seen_at: current_time.clone(),
                    created_at:  current_time.clone(),
                    updated_at:  current_time.clone(),
                    last_message_time: current_time.clone(),
                };
                let _connection2 = Connections {
                    connection_from_user_address: _messageto.clone(),
                    connection_to_user_address:  _messagefrom.clone(),
                    last_message: message.clone(),
                    last_seen_at: current_time.clone(),
                    created_at:  current_time.clone(),
                    updated_at:  current_time.clone(),
                    last_message_time: current_time.clone(),
                };
                _all_connection.push(_connection1);
                _all_connection.push(_connection2);
                AllConnection::<T>::put(_all_connection);
            }else{

                let mut connection1 = _all_connection.clone().into_iter().find(|x| x.connection_from_user_address == _messagefrom && x.connection_to_user_address == _messageto).unwrap();
                let mut connection2 = _all_connection.clone().into_iter().find(|x| x.connection_from_user_address == _messageto && x.connection_to_user_address == _messagefrom).unwrap();
                _all_connection.retain(|x| *x != connection1);
                _all_connection.retain(|x| *x != connection2);

                connection1.last_seen_at = current_time.clone();
                connection1.updated_at = current_time.clone();
                connection1.last_message_time = current_time.clone();
                connection1.last_message = message.clone() ;
                connection2.last_seen_at = current_time.clone();
                connection2.updated_at = current_time.clone();
                connection2.last_message_time = current_time.clone();
                connection2.last_message = message.clone() ;

                _all_connection.push(connection1);
                _all_connection.push(connection2);
                AllConnection::<T>::put(_all_connection);
            }

            Ok(())
        }       
    }

    impl<T: Config> Pallet<T> {
        pub fn u32_to_asset_balance(input: u32) -> BalanceOf<T> {
            input.try_into().ok().unwrap()
        }    
    }    
}