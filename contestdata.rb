   table "contests" 
    t.string "title" 
    t.integer "useraddress" 
    t.string "prize_token_id" 
    t.integer "prize_token_amount" 
    t.integer "prize_token_winners" 
    t.string "token_symbol"
    t.string "statcode" 
    t.datetime "contest_end_date"
    t.text "description"
  end

  table "contest_entries", 
    t.string "useraddress" 
    t.integer "contest_id" 
    t.string "submission_url" 
    t.boolean "winner" 
    t.integer "winner_transfer_id"
  end


  create_table "transfers" 
    t.decimal "amount", precision: 45, scale: 10
    t.string "token_id"
    t.text "reference"
    t.string "transfer_from_useraddress"
    t.string "transfer_to_useraddress"
    t.string "transferuserid", default: 0, null: false
    t.string "transfer_from_feature_token_id"
    t.string "transfer_from_feature"
    t.string "transfer_to_feature_token_id"
    t.string "transfer_to_feature"
    t.string "transfer_from_useraddress_balance_before_transfer"
    t.string "transfer_from_useraddress_balance_after_transfer"
    t.string "transfer_to_useraddress_balance_before_transfer"
    t.string "transfer_to_useraddress_balance_after_transfer"
    t.integer "transfer_from_token_feature_balance_before_transfer"
    t.integer "transfer_from_token_feature_balance_after_transfer"
    t.integer "transfer_to_token_feature_balance_before_transfer"
    t.integer "transfer_to_token_feature_balance_after_transfer"
    t.string "transfer_type"
    t.float "exchange_stakings"
  end

   create_table "user_balances" 
    t.integer "useraddress" 
    t.string "token_id" 
    t.decimal "balance" 
    t.float "balance_value" 
  end

   create_table "type_balances" 
    t.string "currency", default: "", null: false
    t.string "token_type" 
    t.string "token_id" 
    t.decimal "balance" 
  end

