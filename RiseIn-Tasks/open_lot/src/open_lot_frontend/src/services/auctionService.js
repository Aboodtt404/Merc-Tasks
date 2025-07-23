import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from '../declarations/open_lot_backend';

const canisterId = process.env.CANISTER_ID_OPEN_LOT_BACKEND || 
                   process.env.VITE_CANISTER_ID_OPEN_LOT_BACKEND ||
                   import.meta.env.VITE_CANISTER_ID_OPEN_LOT_BACKEND ||
                   'rrkah-fqaaa-aaaaa-aaaaq-cai';

const isLocal = process.env.DFX_NETWORK === 'local' || 
                import.meta.env.VITE_DFX_NETWORK === 'local' ||
                window.location.hostname === 'localhost' ||
                window.location.hostname.includes('127.0.0.1');

const host = isLocal ? 'http://localhost:4943' : 'https://ic0.app';

const agent = new HttpAgent({ host });

if (isLocal) {
  agent.fetchRootKey().catch(err => {
    console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
    console.error(err);
  });
}

const auctionActor = Actor.createActor(idlFactory, {
  agent,
  canisterId,
});

export class AuctionService {
  static async createAuctionItem(itemData) {
    try {
      const result = await auctionActor.create_auction_item(itemData);
      if (result.Ok) {
        return { success: true, data: result.Ok };
      } else {
        return { success: false, error: result.Err };
      }
    } catch (error) {
      console.error('Error creating auction item:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async placeBid(itemId, bidAmount) {
    try {
      const result = await auctionActor.place_bid(BigInt(itemId), bidAmount);
      if (result.Ok !== undefined) {
        return { success: true };
      } else {
        return { success: false, error: result.Err };
      }
    } catch (error) {
      console.error('Error placing bid:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async editAuctionItem(itemId, updates) {
    try {
      const result = await auctionActor.edit_auction_item(BigInt(itemId), updates);
      if (result.Ok) {
        return { success: true, data: result.Ok };
      } else {
        return { success: false, error: result.Err };
      }
    } catch (error) {
      console.error('Error editing auction item:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async stopAuction(itemId) {
    try {
      const result = await auctionActor.stop_auction(BigInt(itemId));
      if (result.Ok) {
        return { success: true, data: result.Ok };
      } else {
        return { success: false, error: result.Err };
      }
    } catch (error) {
      console.error('Error stopping auction:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getAllAuctionItems() {
    try {
      const items = await auctionActor.get_all_auction_items();
      return { success: true, data: items };
    } catch (error) {
      console.error('Error fetching auction items:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getActiveAuctionItems() {
    try {
      const items = await auctionActor.get_active_auction_items();
      return { success: true, data: items };
    } catch (error) {
      console.error('Error fetching active auction items:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getAuctionItem(itemId) {
    try {
      const item = await auctionActor.get_auction_item(BigInt(itemId));
      if (item.length > 0) {
        return { success: true, data: item[0] };
      } else {
        return { success: false, error: 'Item not found' };
      }
    } catch (error) {
      console.error('Error fetching auction item:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getItemBids(itemId) {
    try {
      const bids = await auctionActor.get_item_bids(BigInt(itemId));
      return { success: true, data: bids };
    } catch (error) {
      console.error('Error fetching item bids:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getUserItems(userPrincipal) {
    try {
      const items = await auctionActor.get_user_items(userPrincipal);
      return { success: true, data: items };
    } catch (error) {
      console.error('Error fetching user items:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getAuctionCount() {
    try {
      const count = await auctionActor.get_auction_count();
      return { success: true, data: count };
    } catch (error) {
      console.error('Error fetching auction count:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getItemSoldForMost() {
    try {
      const item = await auctionActor.get_item_sold_for_most();
      if (item.length > 0) {
        return { success: true, data: item[0] };
      } else {
        return { success: false, error: 'No items found' };
      }
    } catch (error) {
      console.error('Error fetching highest selling item:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getMostBidOnItem() {
    try {
      const item = await auctionActor.get_most_bid_on_item();
      if (item.length > 0) {
        return { success: true, data: item[0] };
      } else {
        return { success: false, error: 'No items found' };
      }
    } catch (error) {
      console.error('Error fetching most bid on item:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static formatError(error) {
    const errorMap = {
      ItemNotFound: 'Auction item not found',
      NotOwner: 'You are not the owner of this item',
      AuctionEnded: 'This auction has ended',
      BidTooLow: 'Your bid is too low',
      ItemNotActive: 'This auction is not active',
      InvalidInput: 'Invalid input provided',
      Unauthorized: 'You are not authorized to perform this action',
      AuctionHasBids: 'Cannot modify auction with existing bids',
      SecurityViolation: 'Input contains invalid characters',
      InputTooLong: 'Input text is too long',
    };
    
    return errorMap[error] || 'An unknown error occurred';
  }
} 