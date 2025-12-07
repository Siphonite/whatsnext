import React, { useState, useMemo } from "react";
import "../styles/dashboard.css";
import { useWallet } from "@solana/wallet-adapter-react";
import { useBalance } from "../hooks/useBalance";
import { useSolPrice } from "../hooks/useSolPrice";

const Sidebar: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);

  const { publicKey } = useWallet();
  const { balance, loading } = useBalance();
  const { solPrice } = useSolPrice();

  const shorten = (str: string) =>
    str.slice(0, 4) + "..." + str.slice(-4);

  const avatarLetter = publicKey
    ? shorten(publicKey.toString())[0]
    : "U";

  // Calculate USDT equivalent
  const usdtValue = useMemo(() => {
    if (!balance || !solPrice) return null;
    return balance * solPrice;
  }, [balance, solPrice]);

  return (
    <aside className={`dashboard-sidebar ${collapsed ? "collapsed" : ""}`}>
      
      {/* HEADER */}
      <div className="sidebar-header">
        {!collapsed && <h2 className="sidebar-title">What's Next</h2>}

        <button
          className="sidebar-toggle"
          onClick={() => setCollapsed(!collapsed)}
        >
          ☰
        </button>
      </div>

      {/* NAVIGATION */}
      <nav className="sidebar-nav">
        <a className="sidebar-link" href="#">
          <span className="icon">📈</span>
          {!collapsed && <span>Live Market</span>}
        </a>

        <a className="sidebar-link" href="#">
          <span className="icon">💰</span>
          {!collapsed && <span>PnL</span>}
        </a>

        <a className="sidebar-link" href="#">
          <span className="icon">👛</span>
          {!collapsed && <span>Wallet</span>}
        </a>

        <a className="sidebar-link" href="#">
          <span className="icon">🏆</span>
          {!collapsed && <span>Leaderboard</span>}
        </a>

        <a className="sidebar-link" href="#">
          <span className="icon">📜</span>
          {!collapsed && <span>History</span>}
        </a>
      </nav>

      {/* USER SECTION */}
      <div className="sidebar-user">
        <div className="user-avatar">{avatarLetter}</div>

        {!collapsed && (
          <div className="user-info">
            {/* Wallet address */}
            <p className="text-sm text-gray-400 font-mono mb-1">
              {publicKey ? shorten(publicKey.toString()) : "Not Connected"}
            </p>

            {/* Balance UI: SOL (USDT) */}
            <p className="text-base text-white font-mono font-semibold">
              {!publicKey
                ? "Connect Wallet"
                : loading
                ? "Loading..."
                : balance !== null
                ? `${balance.toFixed(2)} SOL${usdtValue !== null ? ` (${usdtValue.toFixed(2)} USDT)` : ""}`
                : "0.00 SOL"}
            </p>
          </div>
        )}
      </div>

    </aside>
  );
};

export default Sidebar;
