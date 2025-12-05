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

    console.log(`Bet ${side} with $${amount}`);
    // placeBet() will be integrated later
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
      "
    >
      {/* HEADER */}
      <div className="asset-header flex items-center justify-between">
        <span className="asset-title text-xl font-semibold text-white">
          {asset}
        </span>

        <span className="asset-price text-green-400 text-lg font-medium">
          {price ? `$${price.toFixed(2)}` : "$0.00"}
        </span>
      </div>

      {/* REAL CHART - Wrapper needs to be flex-1 to grow */}
      <div className="asset-chart flex-1 w-full min-h-0">
        <RealChart />
      </div>

      {/* BETTING SECTION - Fixed at bottom */}
      <div className="asset-actions w-full">

        {/* AMOUNT INPUT */}
        <div className="amount-input flex items-center bg-black/40 px-3 py-3 rounded-lg border border-gray-700 mb-4">
          <span className="mr-2 text-gray-400">$</span>
          <input
            type="number"
            placeholder="0.00"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="bg-transparent w-full outline-none text-white text-lg"
          />
        </div>

        {/* BET BUTTONS */}
        <div className="bet-buttons flex gap-4">
          <button
            onClick={() => handleBet("GREEN")}
            className="
              flex-1 
              py-3 
              rounded-lg 
              bg-green-600 
              hover:bg-green-500 
              transition 
              text-white 
              font-semibold
            "
          >
            GREEN
          </button>

          <button
            onClick={() => handleBet("RED")}
            className="
              flex-1 
              py-3 
              rounded-lg 
              bg-red-600 
              hover:bg-red-500 
              transition 
              text-white 
              font-semibold
            "
          >
            RED
          </button>
        </div>
      </div>
    </div>
  );
};

export default AssetCard;