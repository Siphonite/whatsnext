import React, { useState } from "react";
import "../styles/dashboard.css";

const Sidebar: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);

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

      {/* User Widget */}
      <div className="sidebar-user">
        <div className="user-avatar">U</div>
        {!collapsed && (
          <div className="user-info">
            <p className="user-label">WALLET BALANCE</p>
            <p className="user-balance">$12,450.00</p>
          </div>
        )}
      </div>

    </aside>
  );
};

export default Sidebar;
