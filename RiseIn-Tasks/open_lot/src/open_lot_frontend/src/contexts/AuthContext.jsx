import React, { createContext, useContext, useState, useEffect } from 'react';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent, Actor } from '@dfinity/agent';

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
  const [authClient, setAuthClient] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    initAuth();
  }, []);

  const initAuth = async () => {
    try {
      const client = await AuthClient.create();
      setAuthClient(client);

      const isAuthenticated = await client.isAuthenticated();
      setIsAuthenticated(isAuthenticated);

      if (isAuthenticated) {
        const identity = client.getIdentity();
        const principal = identity.getPrincipal();
        setPrincipal(principal.toString());
      } else {
        setPrincipal('2vxsx-fae');
      }
    } catch (error) {
      console.error('Auth initialization failed:', error);
      setPrincipal('2vxsx-fae');
    } finally {
      setLoading(false);
    }
  };

  const login = async () => {
    if (!authClient) return false;

    try {
      const days = BigInt(7);
      const hours = BigInt(24);
      const nanoseconds = days * hours * BigInt(3600000000000);

      const isLocal = process.env.DFX_NETWORK === 'local' || 
                      import.meta.env.VITE_DFX_NETWORK === 'local' ||
                      window.location.hostname === 'localhost' ||
                      window.location.hostname.includes('127.0.0.1');

      // Get Internet Identity canister ID
      let iiCanisterId = 'rdmx6-jaaaa-aaaaa-aaadq-cai'; // standard local II canister ID
      
      try {
        // Try to get the actual II canister ID from dfx
        const envIIId = process.env.CANISTER_ID_INTERNET_IDENTITY || 
                       import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY;
        if (envIIId) {
          iiCanisterId = envIIId;
          console.log('Using Internet Identity canister ID from env:', iiCanisterId);
        } else {
          console.log('Using hardcoded Internet Identity canister ID:', iiCanisterId);
        }
      } catch (error) {
        console.warn('Using default Internet Identity canister ID:', iiCanisterId);
      }

      // Use production II frontend even for local development to avoid white page
      const internetIdentityUrl = 'https://identity.ic0.app';

      console.log('🔐 Attempting login with Internet Identity URL:', internetIdentityUrl);
      console.log('🔑 Using canister ID:', iiCanisterId);

      await authClient.login({
        identityProvider: internetIdentityUrl,
        maxTimeToLive: nanoseconds,
        onSuccess: () => {
          const identity = authClient.getIdentity();
          const principal = identity.getPrincipal();
          setPrincipal(principal.toString());
          setIsAuthenticated(true);
        },
        onError: (error) => {
          console.error('Login failed:', error);
        }
      });

      return true;
    } catch (error) {
      console.error('Login error:', error);
      return false;
    }
  };

  const logout = async () => {
    if (!authClient) return;

    try {
      await authClient.logout();
      setIsAuthenticated(false);
      setPrincipal('2vxsx-fae');
    } catch (error) {
      console.error('Logout error:', error);
    }
  };

  const value = {
    isAuthenticated,
    principal,
    login,
    logout,
    loading
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}; 