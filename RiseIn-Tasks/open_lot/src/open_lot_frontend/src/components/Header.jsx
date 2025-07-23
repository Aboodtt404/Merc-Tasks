import React from 'react';
import { motion } from 'framer-motion';
import { Plus, User, Search } from 'lucide-react';
import logoSvg from '../assets/images/OpenLot.svg';

export default function Header({ currentView, setCurrentView, userPrincipal }) {
  const navItems = [
    { id: 'marketplace', label: 'Marketplace', icon: Search },
    { id: 'create', label: 'Create Auction', icon: Plus },
    { id: 'profile', label: 'My Auctions', icon: User },
  ];

  return (
    <motion.header
      initial={{ y: -100, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      className="fixed top-0 left-0 right-0 z-50 bg-black/60 backdrop-blur-md border-b border-white/10"
    >
      <div className="max-w-7xl mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          <motion.div
            whileHover={{ scale: 1.05 }}
            className="flex items-center space-x-3"
          >
            <img 
              src={logoSvg} 
              alt="OpenLot Logo" 
              className="w-12 h-12 object-contain filter brightness-0 invert"
            />
            <p className="text-sm text-white/70">Decentralized Auctions</p>
          </motion.div>

          <nav className="flex items-center space-x-2">
            {navItems.map((item) => {
              const Icon = item.icon;
              const isActive = currentView === item.id;
              
              return (
                <motion.button
                  key={item.id}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  onClick={() => setCurrentView(item.id)}
                  className={`flex items-center space-x-2 px-4 py-2 rounded-lg transition-all duration-200 ${
                    isActive
                      ? 'text-white shadow-lg text-shadow-lg'
                      : 'text-white/80 hover:text-white hover:bg-black/40 border border-transparent hover:border-white/20'
                  }`}
                  style={{
                    textShadow: isActive ? '0 0 10px rgba(255, 255, 255, 0.8), 0 0 20px rgba(255, 255, 255, 0.6), 0 0 30px rgba(255, 255, 255, 0.4)' : undefined
                  }}
                >
                  <Icon className="w-4 h-4" />
                  <span className="hidden sm:block font-medium">{item.label}</span>
                </motion.button>
              );
            })}
          </nav>

          <div className="flex items-center space-x-4">
            {userPrincipal ? (
              <div className="bg-black/40 border border-white/20 px-3 py-2 rounded-lg backdrop-blur-sm">
                <p className="text-xs text-white/60">Connected</p>
                <p className="text-sm font-mono text-white">
                  {userPrincipal.slice(0, 8)}...{userPrincipal.slice(-4)}
                </p>
              </div>
            ) : (
              <motion.button
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                className="btn-primary"
              >
                Connect Wallet
              </motion.button>
            )}
          </div>
        </div>
      </div>
    </motion.header>
  );
} 