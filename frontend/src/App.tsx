import React, { Suspense } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

// Pages
import LandingPage from "./pages/LandingPage";
import Dashboard from "./pages/Dashboard";

// Solana Wallet Adapter
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";

// CSS for wallet adapter
import "@solana/wallet-adapter-react-ui/styles.css";

const App: React.FC = () => {
  // You should eventually store this in an ENV
  const endpoint = "https://api.devnet.solana.com";

  // Supported wallets
  const wallets = [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter(),
  ];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <BrowserRouter>

            {/* Optional suspense fallback */}
            <Suspense fallback={<div style={{ color: "white" }}>Loading...</div>}>

              <Routes>
                <Route path="/" element={<LandingPage />} />
                <Route path="/dashboard" element={<Dashboard />} />
              </Routes>

            </Suspense>

          </BrowserRouter>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

export default App;
