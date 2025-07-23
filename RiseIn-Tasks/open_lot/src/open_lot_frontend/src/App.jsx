import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Toaster, toast } from 'react-hot-toast';
import ThreeBackground from './components/ThreeBackground';
import Header from './components/Header';
import AuctionCard from './components/AuctionCard';
import { AuctionService } from './services/auctionService';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import { Plus, Search, Loader, Package } from 'lucide-react';

function CreateAuctionForm({ onSubmit, loading }) {
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    starting_price: '',
    duration_hours: ''
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!formData.title || !formData.description || !formData.starting_price) {
      toast.error('Please fill in all required fields');
      return;
    }
    
    const data = {
      title: formData.title,
      description: formData.description,
      starting_price: BigInt(Math.floor(Number(formData.starting_price) * 100000000)),
      duration_hours: formData.duration_hours ? [BigInt(formData.duration_hours)] : []
    };
    
    onSubmit(data);
  };

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      className="max-w-2xl mx-auto"
    >
      <div className="bg-black/50 backdrop-blur-md border border-white/20 rounded-2xl p-8 shadow-2xl">
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-16 h-16 bg-primary-600 rounded-full mb-4 shadow-lg">
            <Plus className="w-8 h-8 text-white" />
          </div>
          <h2 className="text-3xl font-bold text-white mb-2">Create New Auction</h2>
          <p className="text-white/70">List your item for decentralized bidding</p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className="block text-white font-medium mb-2">Title *</label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              placeholder="Enter auction title"
              className="input-field"
              required
            />
          </div>

          <div>
            <label className="block text-white font-medium mb-2">Description *</label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              placeholder="Describe your item"
              rows={4}
              className="input-field resize-none"
              required
            />
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label className="block text-white font-medium mb-2">Starting Price (ICP) *</label>
              <input
                type="number"
                value={formData.starting_price}
                onChange={(e) => setFormData({ ...formData, starting_price: e.target.value })}
                placeholder="0.00"
                min="0"
                step="0.01"
                className="input-field"
                required
              />
            </div>

            <div>
              <label className="block text-white font-medium mb-2">Duration (hours)</label>
              <input
                type="number"
                value={formData.duration_hours}
                onChange={(e) => setFormData({ ...formData, duration_hours: e.target.value })}
                placeholder="Leave empty for no limit"
                min="1"
                className="input-field"
              />
            </div>
          </div>

          <motion.button
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            type="submit"
            disabled={loading}
            className="w-full btn-primary h-12 text-lg font-semibold"
          >
            {loading ? (
              <div className="flex items-center justify-center space-x-2">
                <Loader className="w-5 h-5 animate-spin" />
                <span>Creating Auction...</span>
              </div>
            ) : (
              'Create Auction'
            )}
          </motion.button>
        </form>
      </div>
    </motion.div>
  );
}

function AuctionGrid({ auctions, onBid, currentUser, loading, fetchAuctions }) {
  if (loading) {
    return (
      <div className="flex items-center justify-center py-20">
        <div className="text-center">
          <Loader className="w-8 h-8 animate-spin text-primary-500 mx-auto mb-4" />
          <p className="text-white/70">Loading auctions...</p>
        </div>
      </div>
    );
  }

  if (auctions.length === 0) {
    return (
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="text-center py-20"
      >
        <Package className="w-16 h-16 text-white/40 mx-auto mb-4" />
        <h3 className="text-xl font-semibold text-white mb-2">No auctions found</h3>
        <p className="text-white/70">Be the first to create an auction!</p>
      </motion.div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {auctions.map((auction) => (
        <AuctionCard
          key={auction.id}
          auction={auction}
          onBid={onBid}
          currentUser={currentUser}
          onUpdate={fetchAuctions}
        />
      ))}
    </div>
  );
}

function AuctionApp() {
  const { principal, loading: authLoading } = useAuth();
  const [currentView, setCurrentView] = useState('marketplace');
  const [auctions, setAuctions] = useState([]);
  const [loading, setLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(false);

  useEffect(() => {
    fetchAuctions();
  }, []);

  const fetchAuctions = async () => {
    try {
      setLoading(true);
      const result = await AuctionService.getAllAuctionItems();
      if (result.success) {
        setAuctions(result.data);
      } else {
        toast.error('Failed to fetch auctions: ' + AuctionService.formatError(result.error));
        setAuctions([]);
      }
    } catch (error) {
      toast.error('Failed to fetch auctions');
      setAuctions([]);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateAuction = async (data) => {
    try {
      setActionLoading(true);
      const result = await AuctionService.createAuctionItem(data);
      if (result.success) {
        toast.success('Auction created successfully!');
        setCurrentView('marketplace');
        fetchAuctions();
      } else {
        toast.error('Failed to create auction: ' + AuctionService.formatError(result.error));
      }
    } catch (error) {
      toast.error('Failed to create auction');
    } finally {
      setActionLoading(false);
    }
  };

  const handlePlaceBid = async (auctionId, amount) => {
    try {
      const result = await AuctionService.placeBid(auctionId, BigInt(Math.floor(amount * 100000000)));
      if (result.success) {
        toast.success('Bid placed successfully!');
        fetchAuctions();
      } else {
        toast.error('Failed to place bid: ' + AuctionService.formatError(result.error));
      }
    } catch (error) {
      toast.error('Failed to place bid');
    }
  };

  const filteredAuctions = currentView === 'profile' 
    ? auctions.filter(auction => auction.owner === principal)
    : auctions;

  if (authLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <Loader className="w-8 h-8 animate-spin text-primary-500 mx-auto mb-4" />
          <p className="text-white/70">Initializing...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen">
      <ThreeBackground />
      <Header 
        currentView={currentView} 
        setCurrentView={setCurrentView}
      />
      
      <main className="pt-24 pb-12 px-6">
        <div className="max-w-7xl mx-auto">
          <AnimatePresence mode="wait">
            {currentView === 'create' ? (
              <CreateAuctionForm 
                key="create"
                onSubmit={handleCreateAuction}
                loading={actionLoading}
              />
            ) : (
              <motion.div
                key="marketplace"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
              >
                <div className="text-center mb-12">
                  <h1 className="text-4xl md:text-6xl font-bold text-white mb-4">
                    {currentView === 'profile' ? 'My Auctions' : 'Auction Marketplace'}
                  </h1>
                  <p className="text-xl text-white/70 max-w-2xl mx-auto">
                    {currentView === 'profile' 
                      ? 'Manage your auction listings and track their performance'
                      : 'Discover and bid on unique items in our decentralized marketplace'
                    }
                  </p>
                </div>

                <AuctionGrid
                  auctions={filteredAuctions}
                  onBid={handlePlaceBid}
                  currentUser={principal}
                  loading={loading}
                  fetchAuctions={fetchAuctions}
                />
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </main>

      <Toaster
        position="bottom-right"
        toastOptions={{
          style: {
            background: 'rgba(0, 0, 0, 0.8)',
            backdropFilter: 'blur(10px)',
            border: '1px solid rgba(255, 255, 255, 0.2)',
            color: 'white',
          },
        }}
      />
    </div>
  );
}

export default function App() {
  return (
    <AuthProvider>
      <AuctionApp />
    </AuthProvider>
  );
} 