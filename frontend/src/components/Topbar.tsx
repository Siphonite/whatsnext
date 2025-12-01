import React, { useEffect, useState } from "react";

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

        <button className="wallet-button">CONNECT WALLET</button>
      </div>
    </header>
  );
};

export default TopBar;
