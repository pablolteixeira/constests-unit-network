## Contests


### The asset pallet is available for reference to assets/tokens

For a particular asset id (i.e. token)
- A user may call method to enter details for a new contest for a specific token
- A user may create a contest with the prize value being a number of tokens
- Contest details may be updated by the creator - inc contest end time
- Any user may submit a contest entry
- Contest creator can assign an entry as a winner and issue the reward 
- Contest can be closed


#### 1. Files attached 
a) contestdata.rb - data schema 
b) contests_controller.rb - code to migrate to pallet
c) contestscreens - screen shots for reference
d) contestviews - view/ui code for reference
e) substrate-node-unitchain - includes some frame pallets (eg assets pallet)
 and contains a couple of local pallets for reference (** please add your pallet into this node) 
f) basictemplate - taken from substrate frame for reference


#### 2. Methods in controller to be handled in pallet : 

- module methods
- contest_new
- create_contest
- update_contest
- create_contest_entry
- assign_contest_winner
- close_contest

#### supplementary methods - these are methods to be accessed from the asset balances pallet 
- dcxgetindivtokenbalance
- check_and_update_user_balance
- dcxupdatefeaturebalance
