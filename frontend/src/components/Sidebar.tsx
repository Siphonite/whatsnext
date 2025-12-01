import React from "react";
import "../styles/dashboard.css";

const Sidebar: React.FC = () => {
  return (
    <aside className="dashboard-sidebar">

      {/* Sidebar Header */}
      <div className="sidebar-header">
        <h2 className="sidebar-title">WN_PROTOCOL</h2>
        <button className="sidebar-toggle">☰</button>
      </div>

      {/* Nav */}
      <nav className="sidebar-nav">
        <a className="sidebar-link" href="#">Live Markets</a>
        <a className="sidebar-link" href="#">Wallet</a>
        <a className="sidebar-link" href="#">Leaderboard</a>
      </nav>

      {/* Balance */}
      <div className="sidebar-balance">
        <p className="balance-label">BALANCE</p>
        <p className="balance-value">$12,450.00</p>
      </div>

    </aside>
  );
};

export default Sidebar;
