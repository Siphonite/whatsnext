import React, { useState } from "react";
import "../styles/dashboard.css";
import { useWallet } from "@solana/wallet-adapter-react";
import { useBalance } from "../hooks/useBalance";

const Sidebar: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);

  const { publicKey } = useWallet();
  const { balance, loading } = useBalance();

  const shorten = (str: string) =>
    str.slice(0, 4) + "..." + str.slice(-4);

  const avatarLetter = publicKey
    ? shorten(publicKey.toString())[0]
    : "U";

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
            <p className="user-label">WALLET BALANCE</p>

            {/* Balance UI */}
            <p className="user-balance">
              {!publicKey
                ? "Not Connected"
                : loading
                ? "Loading..."
                : `${balance?.toFixed(2)} SOL`}
            </p>

            {/* Wallet address */}
            <p className="user-address">
              {publicKey ? shorten(publicKey.toString()) : ""}
            </p>
          </div>
        )}
      </div>

    </aside>
  );
};

export default Sidebar;
