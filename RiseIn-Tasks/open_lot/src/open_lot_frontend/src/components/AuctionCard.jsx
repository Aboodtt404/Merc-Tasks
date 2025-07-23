import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Clock, User, TrendingUp, Gavel } from 'lucide-react';

export default function AuctionCard({ auction, onBid, currentUser }) {
  const [bidAmount, setBidAmount] = useState('');
  const [showBidInput, setShowBidInput] = useState(false);

  const isOwner = currentUser && auction.owner === currentUser;
  const timeLeft = auction.end_time ? 
    Math.max(0, Number(auction.end_time) - Date.now() * 1000000) : null;
  
  const formatTime = (nanoseconds) => {
    if (!nanoseconds) return 'No time limit';
    const hours = Math.floor(nanoseconds / (1000000000 * 60 * 60));
    const minutes = Math.floor((nanoseconds % (1000000000 * 60 * 60)) / (1000000000 * 60));
    return `${hours}h ${minutes}m`;
  };

  const handleBid = () => {
    if (bidAmount && Number(bidAmount) > Number(auction.current_highest_bid)) {
      onBid(auction.id, Number(bidAmount));
      setBidAmount('');
      setShowBidInput(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      whileHover={{ y: -5 }}
      className="auction-card group"
    >
      <div className="aspect-video bg-gradient-to-br from-purple-600/20 to-blue-600/20 rounded-lg mb-4 flex items-center justify-center">
        <Gavel className="w-12 h-12 text-white/60 group-hover:text-white/80 transition-colors" />
      </div>

      <div className="space-y-3">
        <div>
          <h3 className="text-lg font-semibold text-white group-hover:text-blue-300 transition-colors">
            {auction.title}
          </h3>
          <p className="text-white/70 text-sm line-clamp-2">
            {auction.description}
          </p>
        </div>

        <div className="flex items-center justify-between text-sm">
          <div className="flex items-center space-x-1 text-white/60">
            <User className="w-4 h-4" />
            <span>Owner: {auction.owner.slice(0, 8)}...</span>
          </div>
          {auction.is_active ? (
            <span className="px-2 py-1 bg-green-500/20 text-green-400 rounded-full text-xs">
              Active
            </span>
          ) : (
            <span className="px-2 py-1 bg-red-500/20 text-red-400 rounded-full text-xs">
              Ended
            </span>
          )}
        </div>

        <div className="space-y-2">
          <div className="flex justify-between items-center">
            <span className="text-white/60 text-sm">Current Bid</span>
            <div className="flex items-center space-x-1">
              <TrendingUp className="w-4 h-4 text-accent-500" />
              <span className="text-lg font-bold text-accent-500">
                {auction.current_highest_bid.toString()} ICP
              </span>
            </div>
          </div>

          {auction.highest_bidder && (
            <div className="text-xs text-white/50">
              Highest bidder: {auction.highest_bidder.slice(0, 8)}...
            </div>
          )}

          {timeLeft !== null && (
            <div className="flex items-center space-x-1 text-sm text-white/60">
              <Clock className="w-4 h-4" />
              <span>{formatTime(timeLeft)}</span>
            </div>
          )}
        </div>

        {auction.is_active && !isOwner && (
          <div className="space-y-2">
            {!showBidInput ? (
              <motion.button
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
                onClick={() => setShowBidInput(true)}
                className="w-full btn-primary"
              >
                Place Bid
              </motion.button>
            ) : (
              <div className="space-y-2">
                <input
                  type="number"
                  value={bidAmount}
                  onChange={(e) => setBidAmount(e.target.value)}
                  placeholder={`Min: ${Number(auction.current_highest_bid) + 1} ICP`}
                  className="input-field"
                  min={Number(auction.current_highest_bid) + 1}
                />
                <div className="flex space-x-2">
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={handleBid}
                    className="flex-1 btn-primary"
                    disabled={!bidAmount || Number(bidAmount) <= Number(auction.current_highest_bid)}
                  >
                    Confirm Bid
                  </motion.button>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      setShowBidInput(false);
                      setBidAmount('');
                    }}
                    className="px-4 btn-secondary"
                  >
                    Cancel
                  </motion.button>
                </div>
              </div>
            )}
          </div>
        )}

        {isOwner && (
          <div className="text-xs text-yellow-400 font-medium">
            You own this auction
          </div>
        )}
      </div>
    </motion.div>
  );
} 