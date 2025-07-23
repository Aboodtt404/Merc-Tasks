use candid::Principal;
use ic_cdk::api::time;

use crate::types::{AuctionError, AuctionItem};

pub struct Validator;

impl Validator {
    pub fn is_auction_expired(item: &AuctionItem) -> bool {
        if let Some(end_time) = item.end_time {
            time() >= end_time
        } else {
            false
        }
    }

    pub fn validate_ownership(caller: Principal, item: &AuctionItem) -> Result<(), AuctionError> {
        if caller != item.owner {
            return Err(AuctionError::NotOwner);
        }
        Ok(())
    }

    pub fn validate_modifiable_auction(item: &AuctionItem) -> Result<(), AuctionError> {
        if !item.is_active {
            return Err(AuctionError::ItemNotActive);
        }
        
        if Self::is_auction_expired(item) {
            return Err(AuctionError::AuctionEnded);
        }
        
        Ok(())
    }

    pub fn validate_auction_input(
        title: &str, 
        description: &str, 
        starting_price: u64
    ) -> Result<(), AuctionError> {
        if title.trim().is_empty() || description.trim().is_empty() {
            return Err(AuctionError::InvalidInput);
        }
        
        if starting_price == 0 {
            return Err(AuctionError::InvalidInput);
        }
        
        if title.len() > 100 || description.len() > 1000 {
            return Err(AuctionError::InputTooLong);
        }
        
        if title.contains(['<', '>', '"', '\'']) || description.contains(['<', '>', '"', '\'']) {
            return Err(AuctionError::SecurityViolation);
        }
        
        Ok(())
    }

    pub fn validate_auction_operation(
        caller: Principal, 
        item: &AuctionItem, 
        require_modifiable: bool
    ) -> Result<(), AuctionError> {
        Self::validate_ownership(caller, item)?;
        
        if require_modifiable {
            Self::validate_modifiable_auction(item)?;
        }
        
        Ok(())
    }

    pub fn validate_bid(
        caller: Principal,
        item: &AuctionItem,
        bid_amount: u64
    ) -> Result<(), AuctionError> {
        if !item.is_active {
            return Err(AuctionError::ItemNotActive);
        }

        if Self::is_auction_expired(item) {
            return Err(AuctionError::AuctionEnded);
        }

        if bid_amount == 0 {
            return Err(AuctionError::InvalidInput);
        }

        if bid_amount <= item.current_highest_bid {
            return Err(AuctionError::BidTooLow);
        }

        if caller == item.owner {
            return Err(AuctionError::Unauthorized);
        }

        Ok(())
    }

    pub fn validate_update_fields(
        title: &Option<String>,
        description: &Option<String>,
        starting_price: &Option<u64>
    ) -> Result<(), AuctionError> {
        if let Some(title) = title {
            if title.trim().is_empty() {
                return Err(AuctionError::InvalidInput);
            }
            if title.len() > 100 {
                return Err(AuctionError::InputTooLong);
            }
            if title.contains(['<', '>', '"', '\'']) {
                return Err(AuctionError::SecurityViolation);
            }
        }

        if let Some(description) = description {
            if description.trim().is_empty() {
                return Err(AuctionError::InvalidInput);
            }
            if description.len() > 1000 {
                return Err(AuctionError::InputTooLong);
            }
            if description.contains(['<', '>', '"', '\'']) {
                return Err(AuctionError::SecurityViolation);
            }
        }

        if let Some(price) = starting_price {
            if *price == 0 {
                return Err(AuctionError::InvalidInput);
            }
        }

        Ok(())
    }

    pub fn validate_stop_auction(item: &AuctionItem) -> Result<(), AuctionError> {
        if !item.is_active {
            return Err(AuctionError::ItemNotActive);
        }
        Ok(())
    }
} 