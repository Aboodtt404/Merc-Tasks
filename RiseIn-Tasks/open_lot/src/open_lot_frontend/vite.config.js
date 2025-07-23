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