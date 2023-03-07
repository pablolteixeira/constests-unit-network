# note that asset/token is taken from the asset pallet

# module methods
# contest_new
# update_contest
# create_contest
# create_contest_entry
# assign_contest_winner
# close_contest

# supplementary methods - these are methods to be accessed also by other pallets
# dcxgetindivtokenbalance
# check_and_update_user_balance
# dcxupdatefeaturebalance


class ContestsController < ApplicationController

  def contest_new
      # get user wallet balance for specific asset 
      @user_token_balance = dcxgetindivtokenbalance(session[:walletaddress], params[:token_id])  
  end


  # contests - create contest
  def create_contest   
    #checks on params 
    if params[:prize_token_amount] == nil or params[:prize_token_amount].blank?
        # redirect_to "/contests/new?error=prize_token_amount&token_id=" + params[:token_id] + "&title=" +  params[:contest_title] + "&prize_token_amount=" +  params[:prize_token_amount] + "&prize_token_winners=" +  params[:prize_token_winners] 
        return
    end 
    if params[:prize_token_winners] == nil or params[:prize_token_winners].blank?
        # redirect_to "/contests/new?error=prize_token_winners&token_id=" + params[:token_id] + "&title=" +  params[:contest_title] + "&prize_token_amount=" +  params[:prize_token_amount] + "&prize_token_winners=" +  params[:prize_token_winners]
        return
    end 
    @current_user_token_balance = check_and_update_user_balance(session[:useraddress], params[:token_id])
    if @current_user_token_balance.to_f == 0 or params[:token_amount].to_f > @current_user_token_balance.to_f
      # redirect_to "/contests/new?error=user_token_balance&token_id=" + params[:token_id] + "&title=" +  params[:contest_title] + "&prize_token_amount=" +  params[:prize_token_amount] + "&prize_token_winners=" +  params[:prize_token_winners]
      return
    end
    contestnew = Contest.create(
      title: params[:contest_title],
      prize_token_id: params[:token_id],
      prize_token_amount: params[:prize_token_amount],
      prize_token_winners: params[:prize_token_winners],
      useraddress: session[:useraddress],
      token_id: params[:token_id],
      contest_end_date: params[:contest_ends],
      statcode: "OPEN"
    )
    @sending_tokens_to_token_contest_feature_escrow =   Transfer.create(
        transfer_from_useraddress: session[:walletaddress],
        transfer_to_useraddress: 0,
        amount: params[:prize_token_amount],
        currency: params[:token_id],
        transfer_to_feature_token_id: params[:token_id],
        transfer_to_feature: "CONTEST",
        transfer_from_feature_token_id: "",
        transfer_from_feature: ""
      )
    @current_user_token_balance = check_and_update_user_balance(session[:useraddress], params[:token_id])
    dcxupdatefeaturebalance(params[:token_id], "CONTEST", params[:token_id])

    # redirecturl = "/contests/" + contestnew.id.to_s + "/edit?status=contest_created" 
    # redirect_to redirecturl
  end




  def update_contest    
      @update_contest = Contest.update(params[:contest_id],
                              :title=>params[:contest_title],
                              :description=>params[:contest_description],
                              :contest_end_date=>params[:contest_ends])
  end

  def create_contest_entry
    require 'uri'
    if  params[:submission_url] =~ URI::regexp
    else
       # redirect_to "/contests/" + params[:contest_id] + "?contest_entry=not_url&submission_url=" + params[:submission_url]
        return
    end
    @contestentry = ContestEntries.create(
      submission_url: params[:submission_url],
      winner: false,
      useraddress: session[:useraddress],
      contest_id: params[:contest_id]
    )
   #  redirect_to "/contests/" + params[:contest_id]
   # puts  @contestentry.errors.full_messages
    # redirecturl = "/contests/" + params[:contest_id] + "?contestentry=submitted"
    # redirect_to redirecturl
  end




  def assign_contest_winner
    @contest_entry = ContestEntries.find(params[:entry_id])
    @contest = Contest.find(@contest_entry.contest_id)
    @prize_per_winner = @contest.prize_token_amount/@contest.prize_token_winners
    @sending_tokens_to_token_contest_feature_escrow =   Transfer.create(
        transfer_from_user_id: 0,
        transfer_to_useraddress: @contest_entry.useraddress,
        amount: @prize_per_winner,
        currency: @contest.token_id,
        transfer_to_feature_token_id: "",
        transfer_to_feature: "",
        transfer_from_feature_token_id: @contest.token_id,
        transfer_from_feature: "CONTEST"
      )
    # dcxupdateindivbalance(@prize_per_winner, @contest.token_id)
    # dcxupdatefeaturebalance(@contest.token_id, "CONTEST", @contest.token_id)
    @current_user_token_balance = check_and_update_user_balance(@contest_entry.user_id, @contest.token_id)
    contestentry = ContestEntries.update(params[:entry_id].to_i, winner: true,:winner_transfer_id=>@sending_tokens_to_token_contest_feature_escrow.id)
    # redirecturl = "#{@basepath}/token/" + params[:token_id] + "/contests/" + params[:contest_id]
    # redirect_to "/contests/" + @contest.id.to_s
    return
  end



  def close_contest

    @contest = Contest.find(params[:contest_id])

    if  @contest.statcode == "OPEN"

           @sending_tokens_to_token_contest_feature_escrow =   Transfer.create(
            transfer_from_useraddress: 0,
            transfer_to_useraddress: session[:useraddress],
            amount: @contest.prize_token_amount,
            currency: @contest.prize_token_id,
            transfer_to_feature_token_id: "",
            transfer_to_feature: "",
            transfer_from_feature_token_id: @contest.prize_token_id,
            transfer_from_feature: "CONTEST"
          )
        @current_user_token_balance = check_and_update_user_balance(session[:useraddress], @contest.prize_token_id)
        dcxupdatefeaturebalance(@contest.prize_token_id "CONTEST", @contest.prize_token_id)

    end


    @update_contest = Contest.update(params[:contest_id],
                            :statcode=>"CLOSED")
    # redirecturl = "/contests/" + @update_contest.id.to_s + "?status=contest_updated" 
    # redirect_to redirecturl
    return
  end




  #supplementary methods
  def dcxgetindivtokenbalance(userid, token_id)
    balanceexists = UserBalance.select(:balance).where(useraddress: useraddress, token_id: token_id).first
    if balanceexists
      balanceexists.balance
    else
      0
    end
  end


  def check_and_update_user_balance(useraddress, token_id)
    token_balance = Transfer.where("currency= ? AND  (transfer_from_useraddress = ? OR transfer_to_useraddress = ?)", token_id, useraddress, useraddress)
      .sum("(case when transfer_from_useraddress = #{useraddress}  then -amount  else amount  end)")

    balanceexists = UserBalance.where("useraddress = ?  AND token_id = ? ", useraddress, token_id).present?
    if balanceexists
      balanceupdate = UserBalance.where("useraddress = ?  AND token_id = ? ", useraddress, token_id).update(balance: token_balance)
      balanceupdate.first.balance
    else
      balancenew = UserBalance.create(useraddress: useraddress, token_id: token_id, balance: token_balance)
      balancenew.balance
    end
  end

   
  def dcxupdatefeaturebalance(currency, token_type, token_id)
    if token_id != "" and !token_id.nil? and token_type != "" and !token_type.nil?
      tokenbalance = Transfer.where("currency= ? AND  (transfer_from_feature = ? OR transfer_to_feature = ?)  AND  (transfer_from_feature_token_id= ? OR transfer_to_feature_token_id = ?) ", currency, token_type, token_type, token_id, token_id).sum("case when transfer_from_feature = '#{token_type}'  then -amount  else amount  end")


      # if tokenbalance != 0
      balanceexists = TypeBalance.where("currency = ? AND token_type = ?  AND token_id = ? ", currency, token_type, token_id).present?
      if balanceexists
        balanceupdate = TypeBalance.where("currency = ? AND token_type = ?  AND token_id = ?  ", currency, token_type, token_id).update(balance: tokenbalance)
      else
        balancenew = TypeBalance.create(currency: currency, token_type: token_type, token_id: token_id, balance: tokenbalance)
      end
      # end
    end
  end

end

