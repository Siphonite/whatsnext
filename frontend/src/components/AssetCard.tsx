import React, { useState, useEffect, useMemo } from "react";
import { useMarketStore } from "../store/useMarketStore";
import { useMarketTimerStore } from "../store/useMarketTimerStore";
import BackendChart from "./BackendChart";

const AssetCard: React.FC = () => {
  const { asset, price } = useMarketStore();
  const { timeLeft } = useMarketTimerStore();
  const [amount, setAmount] = useState<string>("");
  const [chartKey, setChartKey] = useState(0);

  // Check if betting is locked (timeLeft <= 0)
  const isLocked = useMemo(() => {
    return timeLeft <= 0;
  }, [timeLeft]);

  // Reset amount and refresh chart when timer hits 0
  useEffect(() => {
    if (timeLeft <= 0) {
      setAmount("");
      // Refresh chart by changing key (forces remount)
      setChartKey((prev) => prev + 1);
    }
  }, [timeLeft]);

  const handleBet = (side: "GREEN" | "RED") => {
    if (isLocked) {
      alert("Betting is locked. Please wait for the next candle.");
      return;
    }

    if (!amount || Number(amount) <= 0) {
      alert("Enter a valid amount");
      return;
    }

    console.log(`Bet ${side} with $${amount}`);
    // placeBet() integration comes later
  };

  return (
    <div
      className="
        asset-card 
        w-full 
        bg-[#0f0f0f] 
        shadow-lg 
        border 
        border-[#1f1f1f]
        rounded-xl
        p-6
      "
    >
      {/* HEADER */}
      <div className="asset-header flex items-center justify-between mb-4">
        <span className="asset-title text-xl font-semibold text-white">
          {asset}
        </span>

        <span className="asset-price text-green-400 text-lg font-medium">
          {price ? `$${price.toFixed(2)}` : "$0.00"}
        </span>
      </div>

      {/* BACKEND-DRIVEN CHART */}
      <div className="asset-chart w-full h-[550px] mb-6 rounded-lg overflow-hidden">
        <BackendChart 
          key={chartKey}
          containerId={`backend-chart-${asset.replace("/", "")}`} 
        />
      </div>

      {/* BETTING SECTION */}
      <div className="asset-actions w-full">

        {/* AMOUNT INPUT */}
        <div className="amount-input flex items-center bg-black/40 px-3 py-3 rounded-lg border border-gray-700 mb-4">
          <span className="mr-2 text-gray-400">$</span>
          <input
            type="number"
            placeholder="0.00"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            disabled={isLocked}
            className="bg-transparent w-full outline-none text-white text-lg disabled:opacity-50 disabled:cursor-not-allowed"
          />
        </div>

        {/* BET BUTTONS */}
        <div className="bet-buttons flex gap-4">
          <button
            onClick={() => handleBet("GREEN")}
            disabled={isLocked}
            className={`
              flex-1 
              py-3 
              rounded-lg 
              transition 
              text-white 
              font-semibold
              ${isLocked 
                ? "bg-gray-600 cursor-not-allowed opacity-50" 
                : "bg-green-600 hover:bg-green-500"
              }
            `}
          >
            GREEN
          </button>

          <button
            onClick={() => handleBet("RED")}
            disabled={isLocked}
            className={`
              flex-1 
              py-3 
              rounded-lg 
              transition 
              text-white 
              font-semibold
              ${isLocked 
                ? "bg-gray-600 cursor-not-allowed opacity-50" 
                : "bg-red-600 hover:bg-red-500"
              }
            `}
          >
            RED
          </button>
        </div>
      </div>
    </div>
  );
};

export default AssetCard;
