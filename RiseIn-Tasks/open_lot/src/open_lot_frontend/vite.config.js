import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import dfxJson from '../../dfx.json';

const aliases = Object.entries(dfxJson.canisters).reduce(
  (acc, [name, _value]) => ({
    ...acc,
    ['declarations/' + name]: path.resolve(__dirname, '.dfx/local/canisters/' + name),
  }),
  {}
);

export default defineConfig({
  plugins: [react()],
  define: {
    global: 'globalThis',
  },
  build: {
    target: 'es2020',
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          dfinity: ['@dfinity/agent', '@dfinity/auth-client', '@dfinity/candid'],
          three: ['three', '@react-three/fiber', '@react-three/drei'],
        },
      },
    },
    sourcemap: false,
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
  },
  resolve: {
    alias: {
      ...aliases,
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:4943',
        changeOrigin: true,
      },
    },
  },
}); 