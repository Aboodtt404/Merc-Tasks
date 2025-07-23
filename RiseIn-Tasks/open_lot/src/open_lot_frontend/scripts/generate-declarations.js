#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

console.log('🔧 Generating declarations...');

// Ensure declarations directory exists
const declarationsDir = path.join(__dirname, '..', 'src', 'declarations');
const backendDir = path.join(declarationsDir, 'open_lot_backend');

if (!fs.existsSync(declarationsDir)) {
  fs.mkdirSync(declarationsDir, { recursive: true });
}

if (!fs.existsSync(backendDir)) {
  fs.mkdirSync(backendDir, { recursive: true });
}

// Try to get the canister ID from dfx
let canisterId = 'rrkah-fqaaa-aaaaa-aaaaq-cai'; // fallback

try {
  // Check if we're in a dfx project and can get the canister ID
  const result = execSync('dfx canister id open_lot_backend 2>/dev/null', { encoding: 'utf8' });
  canisterId = result.trim();
  console.log(`📋 Using canister ID: ${canisterId}`);
} catch (error) {
  console.log(`⚠️  Could not get canister ID from dfx, using fallback: ${canisterId}`);
}

// Generate the IDL factory (simplified version for demo)
const idlContent = `export const idlFactory = ({ IDL }) => {
  const AuctionError = IDL.Variant({
    'ItemNotFound' : IDL.Null,
    'NotOwner' : IDL.Null,
    'AuctionEnded' : IDL.Null,
    'BidTooLow' : IDL.Null,
    'ItemNotActive' : IDL.Null,
    'InvalidInput' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'AuctionHasBids' : IDL.Null,
    'SecurityViolation' : IDL.Null,
    'InputTooLong' : IDL.Null,
  });
  
  const AuctionItem = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'description' : IDL.Text,
    'starting_price' : IDL.Nat64,
    'current_highest_bid' : IDL.Nat64,
    'highest_bidder' : IDL.Opt(IDL.Principal),
    'owner' : IDL.Principal,
    'new_owner' : IDL.Opt(IDL.Principal),
    'is_active' : IDL.Bool,
    'created_at' : IDL.Nat64,
    'end_time' : IDL.Opt(IDL.Nat64),
  });
  
  const CreateAuctionItem = IDL.Record({
    'title' : IDL.Text,
    'description' : IDL.Text,
    'starting_price' : IDL.Nat64,
    'duration_hours' : IDL.Opt(IDL.Nat64),
  });
  
  const UpdateAuctionItem = IDL.Record({
    'title' : IDL.Opt(IDL.Text),
    'description' : IDL.Opt(IDL.Text),
    'starting_price' : IDL.Opt(IDL.Nat64),
    'duration_hours' : IDL.Opt(IDL.Nat64),
  });
  
  const Bid = IDL.Record({
    'bidder' : IDL.Principal,
    'amount' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
  });
  
  return IDL.Service({
    'create_auction_item' : IDL.Func([CreateAuctionItem], [IDL.Variant({ 'Ok' : AuctionItem, 'Err' : AuctionError })], []),
    'place_bid' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : AuctionError })], []),
    'edit_auction_item' : IDL.Func([IDL.Nat64, UpdateAuctionItem], [IDL.Variant({ 'Ok' : AuctionItem, 'Err' : AuctionError })], []),
    'stop_auction' : IDL.Func([IDL.Nat64], [IDL.Variant({ 'Ok' : AuctionItem, 'Err' : AuctionError })], []),
    'get_auction_item' : IDL.Func([IDL.Nat64], [IDL.Opt(AuctionItem)], ['query']),
    'get_all_auction_items' : IDL.Func([], [IDL.Vec(AuctionItem)], ['query']),
    'get_active_auction_items' : IDL.Func([], [IDL.Vec(AuctionItem)], ['query']),
    'get_item_bids' : IDL.Func([IDL.Nat64], [IDL.Vec(Bid)], ['query']),
    'get_user_items' : IDL.Func([IDL.Principal], [IDL.Vec(AuctionItem)], ['query']),
    'get_auction_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_item_sold_for_most' : IDL.Func([], [IDL.Opt(AuctionItem)], ['query']),
    'get_most_bid_on_item' : IDL.Func([], [IDL.Opt(AuctionItem)], ['query']),
    'update_auction_status' : IDL.Func([IDL.Nat64], [IDL.Variant({ 'Ok' : AuctionItem, 'Err' : AuctionError })], []),
  });
};

export const canisterId = '${canisterId}';
`;

// Write the declarations
fs.writeFileSync(path.join(backendDir, 'index.js'), idlContent);

console.log('✅ Declarations generated successfully!'); 