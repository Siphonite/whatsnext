import React, { useState, useMemo } from "react";
import { Link, useLocation } from "react-router-dom";
import "../styles/dashboard.css";
import { useWallet } from "@solana/wallet-adapter-react";
import { useBalance } from "../hooks/useBalance";
import { useSolPrice } from "../hooks/useSolPrice";

const Sidebar: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);
  const location = useLocation();

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

  const navItems = [
    { name: "Live Markets", path: "/dashboard", icon: "📈" },
    { name: "My Positions", path: "/positions", icon: "💰" },
    { name: "Leaderboard", path: "/leaderboard", icon: "🏆" },
  ];

  return (
    <aside 
      className={`dashboard-sidebar ${collapsed ? "collapsed" : ""} bg-[#0B101B] border-r border-gray-800`}
    >
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
      <nav className="sidebar-nav" style={{ gap: "16px" }}>
        {navItems.map((item) => {
          const isActive = location.pathname === item.path;
          return (
            <Link
              key={item.path}
              to={item.path}
              className={`sidebar-link flex flex-row items-center ${
                isActive ? "text-cyan-400" : "text-gray-300"
              } hover:text-cyan-300`}
            >
              <span className="icon">{item.icon}</span>
              {!collapsed && <span>{item.name}</span>}
            </Link>
          );
        })}
      </nav>

      {/* BOTTOM SECTION - WALLET PREVIEW + BUTTON */}
      <div className="sidebar-user border-t border-gray-700 pt-4 flex flex-col items-stretch">
        <div className="flex items-center gap-3 mb-3">
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

        {!collapsed && (
          <Link
            to="/wallet"
            className="mt-3 block w-full bg-cyan-600 text-white text-center py-2 rounded-lg hover:bg-cyan-500 hover:shadow-[0_0_10px_rgba(6,182,212,0.5)] transition-all duration-200 no-underline"
          >
            Open Wallet
          </Link>
        )}
      </div>
    </aside>
  );
};

export default Sidebar;
