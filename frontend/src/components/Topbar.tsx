import React, { useEffect, useState } from "react";
import { useMarketStore } from "../store/useMarketStore";
import "../styles/topbar.css";

const TopBar: React.FC = () => {

  // Zustand global state
  const { activeAsset } = useMarketStore();

  // Mock countdown timer
  const [timeLeft, setTimeLeft] = useState("03:59:22");

  useEffect(() => {
    const interval = setInterval(() => {
      const now = new Date();
      const seconds = 59 - now.getSeconds();
      const minutes = 59 - now.getMinutes();
      const hours = 3 - (now.getHours() % 4);

      const formatted = `${hours.toString().padStart(2, "0")}:${minutes
        .toString()
        .padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;

      setTimeLeft(formatted);
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="topbar-container">

      {/* Left section */}
      <div className="topbar-left">
        <h2 className="topbar-title">
          Active Market:{" "}
          <span className="highlight">{activeAsset}</span>
        </h2>
      </div>

      {/* Center section */}
      <div className="topbar-center">
        <div className="countdown-box">
          <span>Next Candle Closes In:</span>
          <h3>{timeLeft}</h3>
        </div>
      </div>

      {/* Right section */}
      <div className="topbar-right">
        <div className="market-id-box">
          <span>Market ID</span>
          <h3>#001 (mock)</h3>
        </div>
      </div>

    </div>
  );
};

export default TopBar;
``
