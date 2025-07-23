import React, { createContext, useContext, useState, useEffect } from 'react';
import { Ed25519KeyIdentity } from '@dfinity/identity';

const AuthContext = createContext();

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

export const AuthProvider = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [principal, setPrincipal] = useState(null);
  const [loading, setLoading] = useState(true);

  // Generate consistent Ed25519 identity for local development
  const generateLocalIdentity = () => {
    try {
      let seedData = localStorage.getItem('local_auth_seed');
      if (!seedData) {
        // Create a new seed and store it
        const seed = new Uint8Array(32);
        crypto.getRandomValues(seed);
        seedData = Array.from(seed).join(',');
        localStorage.setItem('local_auth_seed', seedData);
      }
      
      const seed = new Uint8Array(seedData.split(',').map(x => parseInt(x)));
      const identity = Ed25519KeyIdentity.fromSecretKey(seed);
      const principal = identity.getPrincipal().toString();
      
      return principal;
    } catch (error) {
      console.error('Error generating identity:', error);
      return '2vxsx-fae'; // fallback anonymous principal
    }
  };

  useEffect(() => {
    checkAuthStatus();
  }, []);

  const checkAuthStatus = async () => {
    try {
      // Check if user is "logged in" locally
      const localAuth = localStorage.getItem('local_auth_status');
      if (localAuth === 'authenticated') {
        const userPrincipal = generateLocalIdentity();
        setPrincipal(userPrincipal);
        setIsAuthenticated(true);
        console.log('🔐 Local auth: Already authenticated as', userPrincipal);
      } else {
        console.log('🔐 Local auth: Not authenticated');
      }
    } catch (error) {
      console.error('Error checking auth status:', error);
    } finally {
      setLoading(false);
    }
  };

  const login = async () => {
    try {
      setLoading(true);
      
      // Simple local authentication
      const userPrincipal = generateLocalIdentity();
      localStorage.setItem('local_auth_status', 'authenticated');
      
      setPrincipal(userPrincipal);
      setIsAuthenticated(true);
      
      console.log('✅ Local auth: Logged in as', userPrincipal);
    } catch (error) {
      console.error('Login failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const logout = async () => {
    try {
      localStorage.removeItem('local_auth_status');
      setPrincipal(null);
      setIsAuthenticated(false);
      console.log('🔓 Local auth: Logged out');
    } catch (error) {
      console.error('Logout failed:', error);
    }
  };

  const value = {
    isAuthenticated,
    principal,
    loading,
    login,
    logout,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}; 