use candid::{CandidType, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    pub new_owner: Option<Principal>,
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
pub struct UpdateAuctionItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub starting_price: Option<u64>,
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
    AuctionHasBids,
    SecurityViolation,
    InputTooLong,
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