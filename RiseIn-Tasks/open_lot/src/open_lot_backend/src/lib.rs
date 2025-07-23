use candid::Principal;
use ic_cdk::{export_candid, init, post_upgrade, pre_upgrade, query, update};

mod types;
mod storage;
mod validation;
mod handlers;

pub use types::*;

use handlers::{AuctionHandlers, QueryHandlers};
use storage::StorageManager;

#[update]
fn create_auction_item(item_data: CreateAuctionItem) -> Result<AuctionItem, AuctionError> {
    AuctionHandlers::create_auction_item(item_data)
}

#[update]
fn place_bid(item_id: u64, bid_amount: u64) -> Result<(), AuctionError> {
    AuctionHandlers::place_bid(item_id, bid_amount)
}

#[update]
fn edit_auction_item(item_id: u64, updates: UpdateAuctionItem) -> Result<AuctionItem, AuctionError> {
    AuctionHandlers::edit_auction_item(item_id, updates)
}

#[update]
fn stop_auction(item_id: u64) -> Result<AuctionItem, AuctionError> {
    AuctionHandlers::stop_auction(item_id)
}

#[update]
fn update_auction_status(item_id: u64) -> Result<AuctionItem, AuctionError> {
    AuctionHandlers::update_auction_status(item_id)
}

#[update]
fn clear_all_auctions() -> u64 {
    AuctionHandlers::clear_all_auctions()
}

#[query]
fn get_auction_item(item_id: u64) -> Option<AuctionItem> {
    QueryHandlers::get_auction_item(item_id)
}

#[query]
fn get_all_auction_items() -> Vec<AuctionItem> {
    QueryHandlers::get_all_auction_items()
}

#[query]
fn get_active_auction_items() -> Vec<AuctionItem> {
    QueryHandlers::get_active_auction_items()
}

#[query]
fn get_item_bids(item_id: u64) -> Vec<Bid> {
    QueryHandlers::get_item_bids(item_id)
}

#[query]
fn get_user_items(user: Principal) -> Vec<AuctionItem> {
    QueryHandlers::get_user_items(user)
}

#[query]
fn get_auction_count() -> u64 {
    QueryHandlers::get_auction_count()
}

#[query]
fn get_item_sold_for_most() -> Option<AuctionItem> {
    QueryHandlers::get_item_sold_for_most()
}

#[query]
fn get_most_bid_on_item() -> Option<AuctionItem> {
    QueryHandlers::get_most_bid_on_item()
}

#[init]
fn init() {
    StorageManager::init_counter();
}

#[pre_upgrade]
fn pre_upgrade() {
}

#[post_upgrade]
fn post_upgrade() {
}

export_candid!();
