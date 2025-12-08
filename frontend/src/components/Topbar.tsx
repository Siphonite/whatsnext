import React, { useMemo } from "react";
import { useMarketStore } from "../store/useMarketStore";
import { useMarketTimerStore } from "../store/useMarketTimerStore";
import { useMarketTimer } from "../hooks/useMarketTimer";
import "../styles/topbar.css";

const TopBar: React.FC = () => {
  const { asset, price } = useMarketStore();
  const { timeLeft } = useMarketTimerStore();
  
  // Initialize timer sync
  useMarketTimer();

  // Format helper
  const format = (num: number) => num.toString().padStart(2, "0");

  // Convert timeLeft (milliseconds) to HH:MM:SS
  const timeDisplay = useMemo(() => {
    const totalSeconds = Math.floor(timeLeft / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return `${format(hours)}:${format(minutes)}:${format(seconds)}`;
  }, [timeLeft]);

  // Check if timer is locked (timeLeft <= 0 or < 5 minutes)
  const isLocked = useMemo(() => {
    return timeLeft <= 0;
  }, [timeLeft]);

  // Check if timer should flash red (< 5 minutes)
  const shouldFlashRed = useMemo(() => {
    return timeLeft > 0 && timeLeft < 5 * 60 * 1000; // 5 minutes in milliseconds
  }, [timeLeft]);

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
          <h3 className={`countdown-time ${shouldFlashRed ? "flash-red" : ""}`}>
            {timeDisplay}
          </h3>
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
