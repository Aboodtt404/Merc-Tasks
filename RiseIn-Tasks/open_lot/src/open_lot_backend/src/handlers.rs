use candid::Principal;
use ic_cdk::{api::time, caller};

use crate::storage::StorageManager;
use crate::types::{AuctionError, AuctionItem, Bid, CreateAuctionItem, UpdateAuctionItem};
use crate::validation::Validator;

pub struct AuctionHandlers;

impl AuctionHandlers {
    pub fn create_auction_item(item_data: CreateAuctionItem) -> Result<AuctionItem, AuctionError> {
        Validator::validate_auction_input(&item_data.title, &item_data.description, item_data.starting_price)?;

        let caller = caller();
        let id = StorageManager::next_id();
        let current_time = time();
        
        let end_time = item_data.duration_hours.map(|hours| {
            current_time + (hours * 60 * 60 * 1_000_000_000)
        });

        let auction_item = AuctionItem {
            id,
            title: item_data.title,
            description: item_data.description,
            starting_price: item_data.starting_price,
            current_highest_bid: item_data.starting_price,
            highest_bidder: None,
            owner: caller,
            new_owner: None,
            is_active: true,
            created_at: current_time,
            end_time,
        };

        StorageManager::insert_auction_item(id, auction_item.clone());
        StorageManager::init_bid_list(id);

        Ok(auction_item)
    }

    pub fn place_bid(item_id: u64, bid_amount: u64) -> Result<(), AuctionError> {
        let caller = caller();
        
        let mut item = StorageManager::get_auction_item(item_id)
            .ok_or(AuctionError::ItemNotFound)?;

        Validator::validate_bid(caller, &item, bid_amount)?;

        if Validator::is_auction_expired(&item) {
            item.is_active = false;
            StorageManager::update_auction_item(item_id, item);
            return Err(AuctionError::AuctionEnded);
        }

        item.current_highest_bid = bid_amount;
        item.highest_bidder = Some(caller);
        StorageManager::update_auction_item(item_id, item);

        let new_bid = Bid {
            bidder: caller,
            amount: bid_amount,
            timestamp: time(),
        };

        let mut bid_list = StorageManager::get_bid_list(item_id);
        bid_list.bids.push(new_bid);
        StorageManager::update_bid_list(item_id, bid_list);

        Ok(())
    }

    pub fn edit_auction_item(item_id: u64, updates: UpdateAuctionItem) -> Result<AuctionItem, AuctionError> {
        let caller = caller();
        
        let mut item = StorageManager::get_auction_item(item_id)
            .ok_or(AuctionError::ItemNotFound)?;

        Validator::validate_auction_operation(caller, &item, true)?;

        Validator::validate_update_fields(&updates.title, &updates.description, &updates.starting_price)?;

        let bid_list = StorageManager::get_bid_list(item_id);

        if !bid_list.bids.is_empty() && updates.starting_price.is_some() {
            return Err(AuctionError::AuctionHasBids);
        }

        if let Some(title) = updates.title {
            item.title = title;
        }

        if let Some(description) = updates.description {
            item.description = description;
        }

        if let Some(starting_price) = updates.starting_price {
            if bid_list.bids.is_empty() {
                item.starting_price = starting_price;
                item.current_highest_bid = starting_price;
            }
        }

        if let Some(duration_hours) = updates.duration_hours {
            let current_time = time();
            item.end_time = Some(current_time + (duration_hours * 60 * 60 * 1_000_000_000));
        }

        StorageManager::update_auction_item(item_id, item.clone());
        Ok(item)
    }

    pub fn stop_auction(item_id: u64) -> Result<AuctionItem, AuctionError> {
        let caller = caller();
        
        let mut item = StorageManager::get_auction_item(item_id)
            .ok_or(AuctionError::ItemNotFound)?;

        Validator::validate_ownership(caller, &item)?;
        Validator::validate_stop_auction(&item)?;

        item.is_active = false;

        if let Some(highest_bidder) = item.highest_bidder {
            item.new_owner = Some(highest_bidder);
        }

        StorageManager::update_auction_item(item_id, item.clone());
        Ok(item)
    }

    pub fn update_auction_status(item_id: u64) -> Result<AuctionItem, AuctionError> {
        let mut item = StorageManager::get_auction_item(item_id)
            .ok_or(AuctionError::ItemNotFound)?;

        if Validator::is_auction_expired(&item) && item.is_active {
            item.is_active = false;
            StorageManager::insert_auction_item(item.id, item.clone());
        }

        Ok(item)
    }

    pub fn clear_all_auctions() -> u64 {
        let count = StorageManager::get_auction_count();
        StorageManager::clear_all_data();
        count
    }
}

pub struct QueryHandlers;

impl QueryHandlers {
    pub fn get_auction_item(item_id: u64) -> Option<AuctionItem> {
        StorageManager::get_auction_item(item_id)
    }

    pub fn get_all_auction_items() -> Vec<AuctionItem> {
        StorageManager::get_all_auction_items()
    }

    pub fn get_active_auction_items() -> Vec<AuctionItem> {
        StorageManager::get_all_auction_items()
            .into_iter()
            .filter(|item| item.is_active && !Validator::is_auction_expired(item))
            .collect()
    }

    pub fn get_item_bids(item_id: u64) -> Vec<Bid> {
        StorageManager::get_bid_list(item_id).bids
    }

    pub fn get_user_items(user: Principal) -> Vec<AuctionItem> {
        StorageManager::get_all_auction_items()
            .into_iter()
            .filter(|item| item.owner == user)
            .collect()
    }

    pub fn get_auction_count() -> u64 {
        StorageManager::get_auction_count()
    }

    pub fn get_item_sold_for_most() -> Option<AuctionItem> {
        StorageManager::get_all_auction_items()
            .into_iter()
            .filter(|item| !item.is_active && item.highest_bidder.is_some())
            .max_by_key(|item| item.current_highest_bid)
    }

    pub fn get_most_bid_on_item() -> Option<AuctionItem> {
        StorageManager::get_items_with_bid_counts()
            .into_iter()
            .filter(|(_, bid_count)| *bid_count > 0)
            .max_by_key(|(_, bid_count)| *bid_count)
            .map(|(item, _)| item)
    }
} 