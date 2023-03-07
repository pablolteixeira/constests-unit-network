# Pallet Teams/Advisors overview

 This document will explain the caracteristics of the Teams/Advisors pallet develop for Unit.
#### Storage Items
- This item is a map where the keys are the assets id and the values are a vector of members that has been registered.
```
    pub type AllMembers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<Member<BalanceOf<T>, T::BlockNumber>>>;
```
- The struct Member its defined as:
```
	pub struct Member<Balance, BlockNumber> {
    	pub username: Vec<u8>,
		pub token_quantity: Balance,
		pub cliff_period: BlockNumber,
		pub vest_period: BlockNumber,
		pub user_id: u32,
	}
```
Where the info of a member it is related to a vesting period and the user id it is also stored so the info for that member can be obtained easily from the profile pallet.

- This item is a map where the keys are the assets id and the values are a vector of advisors that has been registered for that asset.
```
    pub type Advisors<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<Advisor>>;
```
- The struct Advisor it is defined as:
```
	pub struct Advisor {
    	pub username: Vec<u8>,
		pub user_id: u32,
	}
```
The user id it is also stored with the username so the info for that advisor can be obtained easily from the profile pallet.

##### Storage consideration
I choose to store the user id for members and advisor because the profile pallet have the following map:
```
pub(super) type UserItem<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Users<T::AccountId , BalanceOf<T>>,
	>;
```
Then if you have the user id you can get it directly and not by iterating all the profiles created to get the info fro the member or advisor.

#### Extrinsics
##### Extrinsics consideration
All the extrinsics should be called only for the owner, admin or issuer of an asset.
The asset_id provided as a parameter to the extrinsics must correspond to an existing asset.
 - add_member:
```
  pub fn add_member(origin: OriginFor<T>,
  asset_id: T::AssetId,
  username: Vec<u8>,
  token_quantity: BalanceOf<T>,
  cliff_period: T::BlockNumber,
  vest_period: T::BlockNumber)
```
This extrincis takes as parameters an asset id and the variables needed to create the struct Member (username, token_quantiy, cliff_period and vest_period) the user id for that username its obtained in the logic.
The username should be registered in the profile pallet to be added as a member.
Cannot add an user twice.

- update_member:
```
  pub fn update_member(origin: OriginFor<T>,
  asset_id: T::AssetId,
  username: Vec<u8>,
  new_token_quantity: BalanceOf<T>,
  new_cliff_period: T::BlockNumber,
  new_vest_period: T::BlockNumber)
```
 This extrincis takes as parameters an asset id and the variables to set the new vesting schema. The username and user id cannot be modified.
 
 - delete_member
```
pub fn delete_member(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>)
```
This extrincis takes as parameters an asset id and the username to remove from the members list.

 - create_advisor
```
pub fn create_advisor(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>)
```
Create an advisor for an asset. The username and the user_id are stored for each advisor.

 - remove_advisor
```
pub fn remove_advisor(origin: OriginFor<T>, asset_id: T::AssetId, username: Vec<u8>)
```
Remove a registered advisor from the advisors list for the asset provided. 

#### Events
```
 pub enum Event<T: Config> {
		MemberAdded {
			asset_id: T::AssetId,
			username: Vec<u8>,
			token_quantity: BalanceOf<T>,
			cliff_period: T::BlockNumber,
			vest_period: T::BlockNumber,
			user_id: u32,
		},
		MemberUpdated {
			asset_id: T::AssetId,
			username: Vec<u8>,
			new_token_quantity: BalanceOf<T>,
			new_cliff_period: T::BlockNumber,
			new_vest_period: T::BlockNumber,
		},
		MemberDeleted {
			asset_id: T::AssetId,
			username: Vec<u8>,
		},
		AdvisorCreated {
			asset_id: T::AssetId,
			username: Vec<u8>,
			user_id: u32,
		},
		AdvisorDeleted {
			asset_id: T::AssetId,
			username: Vec<u8>,
		}
	}
```

#### Errors
```
	pub enum Error<T> {
		/// Only the token owner can call this functions
		NotOwnerIssuerOrAdmin,
		/// Invalid asset id
		InvalidAsset,
		/// There is no profiles created
		NoProfilesCreated,
		/// A profile with the given username does not exist in the profiles pallet
		InvalidUsername,
		/// The user is already a member
		AlreadyMember,
		/// The member is not registered
		MemberDoesNotExist,
		/// A member has been deleted
		MemberDeleted,
		/// The user is already an advisor
		AdvisorAlreadyExists,
		/// The advisor is not registered
		AdvisorDoesNotExist,
	}
```
#### Profile Pallet
I need to change the visibility of the username and user_id fields in the Users struct in order to access them and check the validity of the username provided as argument in some extrinsincs of the teams/advisors pallet
```
    pub struct Users<AccountId, Balance> {
    .Other fields
    . 
    pub username: Vec<u8>,
    .Other fields
    . 
    pub user_id: u32,
    }
```

 	
