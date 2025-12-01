import React, { useEffect, useState } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

const TopBar: React.FC = () => {
  const [time, setTime] = useState("00:00:00");

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date().toLocaleTimeString());
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <header className="dashboard-topbar">
      <div>
        <h1 className="topbar-title">MARKET PREDICTION</h1>
        <p className="topbar-subtitle">LIVE FEED // SELECT 9 ASSETS</p>
      </div>

      <div className="topbar-right">
        <div className="system-status">● SYSTEM OPTIMAL</div>
        <div className="system-clock">{time}</div>

        {/* REAL WALLET CONNECT BUTTON */}
        <WalletMultiButton className="wallet-button !rounded-lg !px-4 !py-2 !bg-blue-600 !text-white hover:!bg-blue-700" />
      </div>
    </header>
  );
};

export default TopBar;
