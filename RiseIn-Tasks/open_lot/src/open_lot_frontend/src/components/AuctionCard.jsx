import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Clock, User, TrendingUp, Gavel, AlertCircle, Edit, Square } from 'lucide-react';
import { useAuth } from '../contexts/AuthContext';
import { AuctionService } from '../services/auctionService';
import { formatPrincipal, principalsEqual } from '../utils/principal';

export default function AuctionCard({ auction, onBid, currentUser, onUpdate }) {
  const { isAuthenticated, principal } = useAuth();
  const [bidAmount, setBidAmount] = useState('');
  const [showBidInput, setShowBidInput] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);
  const [showEditForm, setShowEditForm] = useState(false);
  const [editData, setEditData] = useState({
    title: auction.title,
    description: auction.description,
    starting_price: Number(auction.starting_price) / 100000000,
    duration_hours: null
  });

  const isOwner = currentUser && principalsEqual(auction.owner, currentUser);
  const timeLeft = auction.end_time ? 
    Math.max(0, Number(auction.end_time) - Date.now() * 1000000) : null;
  
  const formatTime = (nanoseconds) => {
    if (!nanoseconds) return 'No time limit';
    const hours = Math.floor(nanoseconds / (1000000000 * 60 * 60));
    const minutes = Math.floor((nanoseconds % (1000000000 * 60 * 60)) / (1000000000 * 60));
    return `${hours}h ${minutes}m`;
  };

  const formatTimeLeft = (endTimeNanos) => {
    if (!endTimeNanos) return 'No limit';
    
    const currentTime = Date.now() * 1000000; // Convert to nanoseconds
    const timeLeft = Number(endTimeNanos) - currentTime;
    
    if (timeLeft <= 0) return 'Ended';
    
    const hours = Math.floor(timeLeft / (1000000000 * 60 * 60));
    const minutes = Math.floor((timeLeft % (1000000000 * 60 * 60)) / (1000000000 * 60));
    return `${hours}h ${minutes}m`;
  };

  const handleBid = async () => {
    // Check authentication first
    if (!isAuthenticated || !principal) {
      setError('Please connect to place a bid');
      return;
    }

    const currentBidInICP = Number(auction.current_highest_bid) / 100000000;
    if (!bidAmount || Number(bidAmount) <= currentBidInICP) {
      setError('Bid must be higher than current highest bid');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Convert ICP amount to backend scale and place bid directly
      const bidAmountInBackendScale = Math.floor(Number(bidAmount) * 100000000);
      const result = await AuctionService.placeBid(auction.id, bidAmountInBackendScale);
      
      if (result.success) {
        setBidAmount('');
        setShowBidInput(false);
        if (onUpdate) onUpdate(); // Refresh auction list
        // Don't call onBid to avoid double processing
      } else {
        setError(AuctionService.formatError(result.error));
      }
    } catch (error) {
      setError('Failed to place bid. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleEdit = async () => {
    if (!isAuthenticated || !principal) {
      setError('Please connect to edit your auction');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const updates = {
        title: editData.title !== auction.title ? editData.title : null,
        description: editData.description !== auction.description ? editData.description : null,
        starting_price: editData.starting_price !== Number(auction.starting_price) / 100000000 
          ? Math.floor(editData.starting_price * 100000000) : null,
        duration_hours: editData.duration_hours
      };

      // Remove null values
      Object.keys(updates).forEach(key => updates[key] === null && delete updates[key]);

      if (Object.keys(updates).length === 0) {
        setError('No changes made');
        setIsLoading(false);
        return;
      }

      const result = await AuctionService.editAuctionItem(auction.id, updates);
      
      if (result.success) {
        setShowEditForm(false);
        if (onUpdate) onUpdate(); // Refresh auction list
      } else {
        setError(AuctionService.formatError(result.error));
      }
    } catch (error) {
      setError('Failed to edit auction. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleStop = async () => {
    if (!isAuthenticated || !principal) {
      setError('Please connect to stop your auction');
      return;
    }

    if (!window.confirm('Are you sure you want to stop this auction? This action cannot be undone.')) {
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const result = await AuctionService.stopAuction(auction.id);
      
      if (result.success) {
        if (onUpdate) onUpdate(); // Refresh auction list
      } else {
        setError(AuctionService.formatError(result.error));
      }
    } catch (error) {
      setError('Failed to stop auction. Please try again.');
    } finally {
      setIsLoading(false);
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
        </div>

        <div className="mb-4">
          <p className="text-sm text-white/80 mb-3">
            {auction.description}
          </p>
          
          {/* Auction Status */}
          {!auction.is_active && (
            <div className="mb-3 p-2 bg-gray-600/20 border border-gray-500/30 rounded-lg">
              <div className="text-xs text-gray-400 font-medium">
                🔒 Auction Ended
              </div>
              {auction.new_owner && (
                <div className="text-xs text-green-400 mt-1">
                  Winner: {formatPrincipal(auction.new_owner)}
                </div>
              )}
            </div>
          )}

          {/* Auction Details */}
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <div className="text-white/60 text-xs mb-1">Current Bid</div>
              <div className="text-yellow-400 font-semibold text-lg">
                💰 {(Number(auction.current_highest_bid) / 100000000).toFixed(8)} ICP
              </div>
              {auction.highest_bidder && (
                <div className="text-white/60 text-xs mt-1">
                  Highest bidder: {formatPrincipal(auction.highest_bidder)}
                </div>
              )}
            </div>
            <div>
              <div className="text-white/60 text-xs mb-1">Time Left</div>
              <div className="text-blue-400 font-medium">
                <Clock className="w-4 h-4 inline mr-1" />
                {auction.end_time ? formatTimeLeft(auction.end_time) : 'No limit'}
              </div>
            </div>
          </div>
        </div>

        <div className="flex items-center justify-between text-sm">
          <div className="flex items-center space-x-1 text-white/60">
            <User className="w-4 h-4" />
            <span>Owner: {formatPrincipal(auction.owner)}</span>
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

        {auction.is_active && !isOwner && (
          <div className="space-y-2">
            {error && (
              <div className="flex items-center space-x-2 text-red-400 text-sm">
                <AlertCircle className="w-4 h-4" />
                <span>{error}</span>
              </div>
            )}
            
            {!showBidInput ? (
              <motion.button
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
                onClick={() => {
                  setShowBidInput(true);
                  setError(null);
                }}
                className="w-full btn-primary"
                disabled={isLoading}
              >
                {isLoading ? 'Processing...' : 'Place Bid'}
              </motion.button>
            ) : (
              <div className="space-y-2">
                <input
                  type="number"
                  value={bidAmount}
                  onChange={(e) => {
                    setBidAmount(e.target.value);
                    setError(null);
                  }}
                  placeholder={`Min: ${Number(auction.current_highest_bid) / 100000000 + 0.01} ICP`}
                  className="input-field"
                  min={(Number(auction.current_highest_bid) / 100000000 + 0.01).toFixed(8)}
                  step="0.01"
                  disabled={isLoading}
                />
                <div className="flex space-x-2">
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={handleBid}
                    className="flex-1 btn-primary"
                    disabled={isLoading || !bidAmount || Number(bidAmount) <= Number(auction.current_highest_bid) / 100000000}
                  >
                    {isLoading ? 'Confirming...' : 'Confirm Bid'}
                  </motion.button>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      setShowBidInput(false);
                      setBidAmount('');
                      setError(null);
                    }}
                    className="px-4 btn-secondary"
                    disabled={isLoading}
                  >
                    Cancel
                  </motion.button>
                </div>
              </div>
            )}
          </div>
        )}

        {isOwner && (
          <div className="mt-4 p-3 bg-yellow-500/10 border border-yellow-500/20 rounded-lg">
            <div className="text-xs text-yellow-400 font-medium mb-3">
              You own this auction
            </div>
            
            {showEditForm ? (
              <div className="space-y-3">
                <input
                  type="text"
                  value={editData.title}
                  onChange={(e) => setEditData({...editData, title: e.target.value})}
                  placeholder="Auction title"
                  className="input-field text-sm"
                  disabled={isLoading}
                />
                <textarea
                  value={editData.description}
                  onChange={(e) => setEditData({...editData, description: e.target.value})}
                  placeholder="Description"
                  className="input-field text-sm min-h-[60px] resize-none"
                  disabled={isLoading}
                />
                <input
                  type="number"
                  value={editData.starting_price}
                  onChange={(e) => setEditData({...editData, starting_price: Number(e.target.value)})}
                  placeholder="Starting price (ICP)"
                  className="input-field text-sm"
                  min="0.01"
                  step="0.01"
                  disabled={isLoading}
                />
                <input
                  type="number"
                  value={editData.duration_hours || ''}
                  onChange={(e) => setEditData({...editData, duration_hours: e.target.value ? Number(e.target.value) : null})}
                  placeholder="Duration (hours, optional)"
                  className="input-field text-sm"
                  min="1"
                  disabled={isLoading}
                />
                <div className="flex space-x-2">
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={handleEdit}
                    className="flex-1 btn-primary text-sm py-2"
                    disabled={isLoading}
                  >
                    {isLoading ? 'Saving...' : 'Save Changes'}
                  </motion.button>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      setShowEditForm(false);
                      setEditData({
                        title: auction.title,
                        description: auction.description,
                        starting_price: Number(auction.starting_price) / 100000000,
                        duration_hours: null
                      });
                      setError(null);
                    }}
                    className="px-4 btn-secondary text-sm py-2"
                    disabled={isLoading}
                  >
                    Cancel
                  </motion.button>
                </div>
              </div>
            ) : (
              <div className="flex space-x-2">
                <motion.button
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  onClick={() => setShowEditForm(true)}
                  className="flex-1 bg-blue-600/20 hover:bg-blue-600/30 border border-blue-500/30 text-blue-400 text-sm py-2 px-3 rounded-lg transition-colors"
                  disabled={isLoading || !auction.is_active}
                >
                  <Edit className="w-3 h-3 inline mr-1" />
                  Edit
                </motion.button>
                <motion.button
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  onClick={handleStop}
                  className="flex-1 bg-red-600/20 hover:bg-red-600/30 border border-red-500/30 text-red-400 text-sm py-2 px-3 rounded-lg transition-colors"
                  disabled={isLoading || !auction.is_active}
                >
                  <Square className="w-3 h-3 inline mr-1" />
                  Stop
                </motion.button>
              </div>
            )}
          </div>
        )}
      </div>
    </motion.div>
  );
} 