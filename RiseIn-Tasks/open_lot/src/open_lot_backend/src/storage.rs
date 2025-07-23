use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

use crate::types::{AuctionItem, BidList};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = ic_stable_structures::Cell<u64, Memory>;

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

pub struct StorageManager;

impl StorageManager {
    pub fn next_id() -> u64 {
        ID_COUNTER.with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1).unwrap();
            current_value + 1
        })
    }

    pub fn init_counter() {
        ID_COUNTER.with(|counter| {
            counter.borrow_mut().set(0).unwrap();
        });
    }

    pub fn get_auction_count() -> u64 {
        ID_COUNTER.with(|counter| *counter.borrow().get())
    }

    pub fn insert_auction_item(id: u64, item: AuctionItem) {
        AUCTION_ITEMS.with(|items| {
            items.borrow_mut().insert(id, item);
        });
    }

    pub fn get_auction_item(id: u64) -> Option<AuctionItem> {
        AUCTION_ITEMS.with(|items| {
            items.borrow().get(&id)
        })
    }

    pub fn update_auction_item(id: u64, item: AuctionItem) {
        AUCTION_ITEMS.with(|items| {
            items.borrow_mut().insert(id, item);
        });
    }

    pub fn get_all_auction_items() -> Vec<AuctionItem> {
        AUCTION_ITEMS.with(|items| {
            items.borrow().iter().map(|(_, item)| item).collect()
        })
    }

    pub fn init_bid_list(item_id: u64) {
        ITEM_BIDS.with(|bids| {
            bids.borrow_mut().insert(item_id, BidList::default());
        });
    }

    pub fn get_bid_list(item_id: u64) -> BidList {
        ITEM_BIDS.with(|bids| {
            bids.borrow().get(&item_id).unwrap_or_default()
        })
    }

    pub fn update_bid_list(item_id: u64, bid_list: BidList) {
        ITEM_BIDS.with(|bids| {
            bids.borrow_mut().insert(item_id, bid_list);
        });
    }

    pub fn get_items_with_bid_counts() -> Vec<(AuctionItem, usize)> {
        AUCTION_ITEMS.with(|items| {
            ITEM_BIDS.with(|bids| {
                let auction_items = items.borrow();
                let item_bids = bids.borrow();
                
                auction_items
                    .iter()
                    .map(|(item_id, item)| {
                        let bid_count = item_bids
                            .get(&item_id)
                            .map(|bid_list| bid_list.bids.len())
                            .unwrap_or(0);
                        (item, bid_count)
                    })
                    .collect()
            })
        })
    }
} 