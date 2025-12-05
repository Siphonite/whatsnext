import React, { useState } from "react";
import { useMarketStore } from "../store/useMarketStore";
import RealChart from "./RealChart";

const AssetCard: React.FC = () => {
  const { asset, price } = useMarketStore();

  const [amount, setAmount] = useState<string>("");

  const handleBet = (side: "GREEN" | "RED") => {
    if (!amount || Number(amount) <= 0) {
      alert("Enter a valid amount");
      return;
    }

    // This is where you will later call:
    // placeBet(market_id, side, amount)
    console.log(`Bet ${side} with $${amount}`);
  };

  return (
    <div className="asset-card p-4 rounded-xl bg-[#0f0f0f] shadow-lg border border-[#1f1f1f]">
      
      {/* HEADER */}
      <div className="asset-header flex items-center justify-between mb-2">
        <span className="asset-title text-lg font-semibold text-white">
          {asset}
        </span>

        <span className="asset-price text-green-400 text-md font-medium">
          {price ? `$${price.toFixed(2)}` : "$0.00"}
        </span>
      </div>

      {/* REAL CHART */}
      <div className="asset-chart mb-4">
        <RealChart />
      </div>

      {/* BETTING SECTION */}
      <div className="asset-actions">
        {/* Amount Input */}
        <div className="amount-input flex items-center bg-black/40 px-3 py-2 rounded-lg border border-gray-700 mb-3">
          <span className="mr-2 text-gray-400">$</span>
          <input
            type="number"
            placeholder="0.00"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="bg-transparent w-full outline-none text-white"
          />
        </div>

        {/* Bet Buttons */}
        <div className="bet-buttons flex gap-3">
          <button
            onClick={() => handleBet("GREEN")}
            className="bet-green flex-1 py-2 rounded-lg bg-green-600 hover:bg-green-500 text-white font-semibold"
          >
            GREEN
          </button>

          <button
            onClick={() => handleBet("RED")}
            className="bet-red flex-1 py-2 rounded-lg bg-red-600 hover:bg-red-500 text-white font-semibold"
          >
            RED
          </button>
        </div>
      </div>
    </div>
  );
};

export default AssetCard;
