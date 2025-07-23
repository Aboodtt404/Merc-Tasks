import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { idlFactory } from '../declarations/open_lot_backend';

const canisterId = process.env.CANISTER_ID_OPEN_LOT_BACKEND || 
                   process.env.VITE_CANISTER_ID_OPEN_LOT_BACKEND ||
                   import.meta.env.VITE_CANISTER_ID_OPEN_LOT_BACKEND ||
                   'uxrrr-q7777-77774-qaaaq-cai';

const isLocal = process.env.DFX_NETWORK === 'local' || 
                import.meta.env.VITE_DFX_NETWORK === 'local' ||
                window.location.hostname === 'localhost' ||
                window.location.hostname.includes('127.0.0.1');

const host = isLocal ? 'http://localhost:4943' : 'https://ic0.app';

// Create anonymous actor for queries (read-only operations)
async function createAnonymousActor() {
  const agent = new HttpAgent({ host });
  
  if (isLocal) {
    await agent.fetchRootKey().catch(err => {
      console.warn("Unable to fetch root key");
      console.error(err);
    });
  }

  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
}

// Create authenticated actor for updates (write operations)
async function createAuthenticatedActor() {
  try {
    // Check if user is authenticated locally
    const localAuth = localStorage.getItem('local_auth_status');
    
    if (localAuth !== 'authenticated') {
      throw new Error('User not authenticated');
    }
    
    // Create a simple identity from the stored principal
    // For local development, we'll use the same identity for consistency
    let identity;
    
    try {
      // Try to get or create a consistent Ed25519 identity for this user
      let seedData = localStorage.getItem('local_auth_seed');
      if (!seedData) {
        // Create a new seed and store it
        const seed = new Uint8Array(32);
        crypto.getRandomValues(seed);
        seedData = Array.from(seed).join(',');
        localStorage.setItem('local_auth_seed', seedData);
      }
      
      const seed = new Uint8Array(seedData.split(',').map(x => parseInt(x)));
      identity = Ed25519KeyIdentity.fromSecretKey(seed);
      
      console.log('🔑 Using Ed25519 identity with principal:', identity.getPrincipal().toString());
    } catch (error) {
      console.warn('Failed to create Ed25519 identity, using anonymous:', error);
      // Fallback to anonymous for queries
      return createAnonymousActor();
    }
    
    const agent = new HttpAgent({ 
      host,
      identity
    });

    if (isLocal) {
      await agent.fetchRootKey().catch(err => {
        console.warn("Unable to fetch root key");
        console.error(err);
      });
    }

    return Actor.createActor(idlFactory, {
      agent,
      canisterId,
    });
  } catch (error) {
    console.error('❌ Error creating authenticated actor:', error);
    throw error;
  }
}

export class AuctionService {
  // === UPDATE FUNCTIONS (WRITE OPERATIONS) - TEMPORARILY ANONYMOUS FOR TESTING ===

  static async createAuctionItem(itemData) {
    try {
      console.log('🔧 Creating auction with data:', itemData);
      
      // Use authenticated actor for proper ownership
      const actor = await createAuthenticatedActor();
      const result = await actor.create_auction_item(itemData);
      
      if (result.Ok) {
        console.log('✅ Auction created successfully:', result.Ok);
        return { success: true, data: result.Ok };
      } else {
        console.error('❌ Failed to create auction:', result.Err);
        return { success: false, error: result.Err };
      }
    } catch (error) {
      console.error('❌ Error creating auction:', error);
      return { success: false, error: error.message };
    }
  }

  static async placeBid(itemId, bidAmount) {
    try {
      // Temporarily use anonymous actor for testing
      const actor = await createAnonymousActor();
      const result = await actor.place_bid(BigInt(itemId), BigInt(bidAmount));
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
      // Temporarily use anonymous actor for testing
      const actor = await createAnonymousActor();
      const result = await actor.edit_auction_item(BigInt(itemId), updates);
      if (result.Ok) {
        return { success: true, data: result.Ok };
      } else {
        return { success: false, error: result.Err };
      }
    } catch (error) {
      console.error('Error editing auction:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async stopAuction(itemId) {
    try {
      // Temporarily use anonymous actor for testing
      const actor = await createAnonymousActor();
      const result = await actor.stop_auction(BigInt(itemId));
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

  static async clearAllAuctions() {
    try {
      console.log('🗑️ Clearing all auctions...');
      // Temporarily use anonymous actor for testing
      const actor = await createAnonymousActor();
      const count = await actor.clear_all_auctions();
      console.log(`✅ Cleared ${count} auctions`);
      return { success: true, data: count };
    } catch (error) {
      console.error('❌ Error clearing auctions:', error);
      return { success: false, error: error.message };
    }
  }

  // === QUERY FUNCTIONS (READ-ONLY) - USE ANONYMOUS ACTOR ===

  static async getAllAuctionItems() {
    try {
      const actor = await createAnonymousActor();
      const items = await actor.get_all_auction_items();
      return { success: true, data: items };
    } catch (error) {
      console.error('Error fetching all auctions:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getActiveAuctionItems() {
    try {
      const actor = await createAnonymousActor();
      const items = await actor.get_active_auction_items();
      return { success: true, data: items };
    } catch (error) {
      console.error('Error fetching active auctions:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getAuctionItem(itemId) {
    try {
      const actor = await createAnonymousActor();
      const item = await actor.get_auction_item(BigInt(itemId));
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
      const actor = await createAnonymousActor();
      const bids = await actor.get_item_bids(BigInt(itemId));
      return { success: true, data: bids };
    } catch (error) {
      console.error('Error fetching item bids:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getUserItems(principal) {
    try {
      const actor = await createAnonymousActor();
      const items = await actor.get_user_items(principal);
      console.log('📊 User items response:', items);
      return { success: true, data: items };
    } catch (error) {
      console.error('❌ Error fetching user items:', error);
      return { success: false, error: error.message };
    }
  }

  static async getAuctionCount() {
    try {
      const actor = await createAnonymousActor();
      const count = await actor.get_auction_count();
      return { success: true, data: count };
    } catch (error) {
      console.error('Error fetching auction count:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getItemSoldForMost() {
    try {
      const actor = await createAnonymousActor();
      const item = await actor.get_item_sold_for_most();
      if (item.length > 0) {
        return { success: true, data: item[0] };
      } else {
        return { success: false, error: 'No items found' };
      }
    } catch (error) {
      console.error('Error fetching most expensive item:', error);
      return { success: false, error: 'Network error' };
    }
  }

  static async getMostBidOnItem() {
    try {
      const actor = await createAnonymousActor();
      const item = await actor.get_most_bid_on_item();
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
    
    console.log('🔍 Formatting error:', error, typeof error);
    
    if (typeof error === 'string') {
      return errorMap[error] || `Unknown error: ${error}`;
    }
    
    if (typeof error === 'object' && error !== null) {
      const errorKey = Object.keys(error)[0];
      return errorMap[errorKey] || `Unknown error object: ${JSON.stringify(error)}`;
    }
    
    return `Unknown error type: ${error}`;
  }
} 