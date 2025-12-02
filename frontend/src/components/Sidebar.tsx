import React, { useState } from "react";
import "../styles/dashboard.css";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { useEffect } from "react";

const Sidebar: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);

  const { publicKey } = useWallet();
  const { connection } = useConnection();
  const [balance, setBalance] = useState<number | null>(null);

  // Fetch wallet balance dynamically
  useEffect(() => {
    const fetchBalance = async () => {
      if (!publicKey) {
        setBalance(null);
        return;
      }

      try {
        const lamports = await connection.getBalance(publicKey);
        setBalance(lamports / 1_000_000_000); // Lamports → SOL
      } catch (err) {
        console.error("Failed to fetch balance:", err);
      }
    };

    fetchBalance();

    // Auto-refresh balance every 10 seconds
    const interval = setInterval(fetchBalance, 10000);
    return () => clearInterval(interval);
  }, [publicKey, connection]);

  return (
    <aside className={`dashboard-sidebar ${collapsed ? "collapsed" : ""}`}>
      
      {/* Header */}
      <div className="sidebar-header">
        {!collapsed && <h2 className="sidebar-title">What's Next</h2>}
        <button
          className="sidebar-toggle"
          onClick={() => setCollapsed(!collapsed)}
        >
          ☰
        </button>
      </div>

      {/* Navigation */}
      <nav className="sidebar-nav">
        <a className="sidebar-link" href="#">
          <span className="icon">📈</span>
          {!collapsed && <span>Live Markets</span>}
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

      {/* User Info */}
      <div className="sidebar-user">
        <div className="user-avatar">U</div>

        {!collapsed && (
          <div className="user-info">
            <p className="user-label">WALLET BALANCE</p>

            {/* REAL SOL balance */}
            <p className="user-balance">
              {publicKey
                ? balance !== null
                  ? `${balance.toFixed(2)} SOL`
                  : "Loading..."
                : "Not Connected"}
            </p>
          </div>
        )}
      </div>

    </aside>
  );
};

export default Sidebar;
