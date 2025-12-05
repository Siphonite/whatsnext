import React, { useEffect, useState } from "react";
import { useMarketStore } from "../store/useMarketStore";
import "../styles/topbar.css";

const TopBar: React.FC = () => {
  const { asset, price } = useMarketStore();

  const [timeLeft, setTimeLeft] = useState("00:00:00");
  const [isLocked, setIsLocked] = useState(false);

  // Format helper
  const format = (num: number) => num.toString().padStart(2, "0");

  // Placeholder market data (would come from props or context in future)
  // For now, assume market runs for 4 hours
  const marketStartTime = Math.floor(Date.now() / 1000);
  const marketEndTime = marketStartTime + (4 * 3600); // 4 hours
  const marketLockTime = marketEndTime - (10 * 60); // 10 mins before close

  // ------------------------------
  // REAL COUNTDOWN TIMER
  // ------------------------------
  useEffect(() => {
    const interval = setInterval(() => {
      const now = Date.now() / 1000;

      // lock status
      setIsLocked(now >= marketLockTime);

      const remaining = Math.max(0, marketEndTime - now);

      const hours = Math.floor(remaining / 3600);
      const minutes = Math.floor((remaining % 3600) / 60);
      const seconds = Math.floor(remaining % 60);

      setTimeLeft(`${format(hours)}:${format(minutes)}:${format(seconds)}`);
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="topbar-container">

      {/* LEFT SECTION */}
      <div className="topbar-left">
        <h2 className="topbar-title">
          Market: <span className="highlight">{asset}</span>
        </h2>

        <div className={`lock-status ${isLocked ? "locked" : "open"}`}>
          {isLocked ? "BETTING LOCKED" : "BETTING OPEN"}
        </div>
      </div>

      {/* CENTER SECTION: COUNTDOWN */}
      <div className="topbar-center">
        <div className="countdown-box">
          <span>Candle Closes In:</span>
          <h3 className="countdown-time">{timeLeft}</h3>
        </div>
      </div>

      {/* RIGHT SECTION */}
      <div className="topbar-right">
        <div className="market-id-box">
          <span>Price</span>
          <h3>
            {price ? `$${price.toFixed(2)}` : "Loading..."}
          </h3>
        </div>
      </div>

    </div>
  );
};

export default TopBar;
