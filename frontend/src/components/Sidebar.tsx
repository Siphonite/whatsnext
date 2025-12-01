import React, { useState } from "react";

const Sidebar: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside
      className={`dashboard-sidebar ${collapsed ? "sidebar-collapsed" : ""}`}
    >
      {/* Header */}
      <div className="sidebar-header">
        <h2 className="sidebar-title">WN_PROTOCOL</h2>

        <button
          className="sidebar-toggle"
          onClick={() => setCollapsed(!collapsed)}
        >
          ☰
        </button>
      </div>

      {/* Nav Links */}
      <nav className="sidebar-nav">
        <a className="sidebar-link">Live Markets</a>
        <a className="sidebar-link">Wallet</a>
        <a className="sidebar-link">Leaderboard</a>
      </nav>

      {/* Balance Section */}
      <div className="sidebar-balance">
        <p className="balance-label">BALANCE</p>
        <p className="balance-value">$12,450.00</p>
      </div>
    </aside>
  );
};

export default Sidebar;
