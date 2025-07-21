use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller, export_candid, init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, BoundedStorable, Storable,
};
use serde::Serialize;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = ic_stable_structures::Cell<u64, Memory>;

const MAX_VALUE_SIZE: u32 = 5000;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AuctionItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub starting_price: u64,
    pub current_highest_bid: u64,
    pub highest_bidder: Option<Principal>,
    pub owner: Principal,
    pub is_active: bool,
    pub created_at: u64,
    pub end_time: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Bid {
    pub bidder: Principal,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct BidList {
    pub bids: Vec<Bid>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CreateAuctionItem {
    pub title: String,
    pub description: String,
    pub starting_price: u64,
    pub duration_hours: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum AuctionError {
    ItemNotFound,
    NotOwner,
    AuctionEnded,
    BidTooLow,
    ItemNotActive,
    InvalidInput,
    Unauthorized,
}

impl Storable for AuctionItem {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for AuctionItem {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for BidList {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for BidList {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static AUCTION_ITEMS: RefCell<StableBTreeMap<u64, AuctionItem, Memory>> =
        RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            )
        );

    static ITEM_BIDS: RefCell<StableBTreeMap<u64, BidList, Memory>> =
        RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            )
        );
}

fn next_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value + 1
    })
}

fn is_auction_expired(item: &AuctionItem) -> bool {
    if let Some(end_time) = item.end_time {
        time() >= end_time
    } else {
        false
    }
}

#[update]
fn create_auction_item(item_data: CreateAuctionItem) -> Result<AuctionItem, AuctionError> {
    if item_data.title.trim().is_empty() || item_data.description.trim().is_empty() {
        return Err(AuctionError::InvalidInput);
    }

    let caller = caller();
    let id = next_id();
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
        is_active: true,
        created_at: current_time,
        end_time,
    };

    AUCTION_ITEMS.with(|items| {
        items.borrow_mut().insert(id, auction_item.clone());
    });

    ITEM_BIDS.with(|bids| {
        bids.borrow_mut().insert(id, BidList::default());
    });

    Ok(auction_item)
}

#[update]
fn place_bid(item_id: u64, bid_amount: u64) -> Result<(), AuctionError> {
    let caller = caller();
    
    let mut item = AUCTION_ITEMS.with(|items| {
        items.borrow().get(&item_id)
    }).ok_or(AuctionError::ItemNotFound)?;

    if !item.is_active {
        return Err(AuctionError::ItemNotActive);
    }

    if is_auction_expired(&item) {
        item.is_active = false;
        AUCTION_ITEMS.with(|items| {
            items.borrow_mut().insert(item_id, item);
        });
        return Err(AuctionError::AuctionEnded);
    }

    if bid_amount <= item.current_highest_bid {
        return Err(AuctionError::BidTooLow);
    }

    if caller == item.owner {
        return Err(AuctionError::Unauthorized);
    }

    item.current_highest_bid = bid_amount;
    item.highest_bidder = Some(caller);

    AUCTION_ITEMS.with(|items| {
        items.borrow_mut().insert(item_id, item);
    });

    let new_bid = Bid {
        bidder: caller,
        amount: bid_amount,
        timestamp: time(),
    };

    ITEM_BIDS.with(|bids| {
        let mut bid_list = bids.borrow().get(&item_id).unwrap_or_default();
        bid_list.bids.push(new_bid);
        bids.borrow_mut().insert(item_id, bid_list);
    });

    Ok(())
}

#[query]
fn get_auction_item(item_id: u64) -> Option<AuctionItem> {
    AUCTION_ITEMS.with(|items| {
        items.borrow().get(&item_id)
    })
}

#[query]
fn get_all_auction_items() -> Vec<AuctionItem> {
    AUCTION_ITEMS.with(|items| {
        items.borrow().iter().map(|(_, item)| item).collect()
    })
}

#[query]
fn get_active_auction_items() -> Vec<AuctionItem> {
    AUCTION_ITEMS.with(|items| {
        items.borrow()
            .iter()
            .map(|(_, item)| item)
            .filter(|item| item.is_active && !is_auction_expired(item))
            .collect()
    })
}

#[query]
fn get_item_bids(item_id: u64) -> Vec<Bid> {
    ITEM_BIDS.with(|bids| {
        bids.borrow().get(&item_id).unwrap_or_default().bids
    })
}

#[query]
fn get_user_items(user: Principal) -> Vec<AuctionItem> {
    AUCTION_ITEMS.with(|items| {
        items.borrow()
            .iter()
            .map(|(_, item)| item)
            .filter(|item| item.owner == user)
            .collect()
    })
}

#[query]
fn get_auction_count() -> u64 {
    ID_COUNTER.with(|counter| *counter.borrow().get())
}

#[update]
fn update_auction_status(item_id: u64) -> Result<AuctionItem, AuctionError> {
    let mut item = AUCTION_ITEMS.with(|items| {
        items.borrow().get(&item_id)
    }).ok_or(AuctionError::ItemNotFound)?;

    if is_auction_expired(&item) && item.is_active {
        item.is_active = false;
        AUCTION_ITEMS.with(|items| {
            items.borrow_mut().insert(item_id, item.clone());
        });
    }

    Ok(item)
}

#[init]
fn init() {
    ID_COUNTER.with(|counter| {
        counter.borrow_mut().set(0).unwrap();
    });
}

#[pre_upgrade]
fn pre_upgrade() {
}

#[post_upgrade]
fn post_upgrade() {
}

export_candid!();
