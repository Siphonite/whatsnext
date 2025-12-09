import React, { useState, useEffect, useMemo } from "react";
import { useMarketStore } from "../store/useMarketStore";
import { useMarketTimerStore } from "../store/useMarketTimerStore";
import BackendChart from "./BackendChart";

import axios from "axios";
import { SystemProgram } from "@solana/web3.js";
import { useAnchorProgram } from "../hooks/useAnchorProgram";
import { deriveMarketPDA, deriveUserBetPDA } from "../utils/pda";

const AssetCard: React.FC = () => {
  const { asset, price } = useMarketStore();
  const { timeLeft } = useMarketTimerStore();
  const [amount, setAmount] = useState<string>("");
  const [chartKey, setChartKey] = useState(0);
  const [loading, setLoading] = useState(false);

  const { program, walletPubkey, provider } = useAnchorProgram();

  // Betting lock check
  const isLocked = useMemo(() => timeLeft <= 0, [timeLeft]);

  // Reset on new candle
  useEffect(() => {
    if (timeLeft <= 0) {
      setAmount("");
      setChartKey((prev) => prev + 1);
    }
  }, [timeLeft]);

  // -------------------------------
  //       PLACE BET HANDLER
  // -------------------------------
  const handleBet = async (side: "GREEN" | "RED") => {
    try {
      if (isLocked) {
        alert("Betting is locked. Please wait for the next candle.");
        return;
      }

      const amountFloat = Number(amount);
      if (isNaN(amountFloat) || amountFloat <= 0) {
        alert("Invalid amount");
        return;
      }

      if (!program || !walletPubkey || !provider) {
        alert("Connect your wallet first");
        return;
      }

      setLoading(true);

      // 1) Fetch active markets from backend
      const res = await axios.get(
        `${import.meta.env.VITE_BACKEND_URL}/market/active`
      );

      const markets = res.data;
      const currentMarket = Array.isArray(markets) ? markets[0] : markets;

      if (!currentMarket || !currentMarket.market_id) {
        alert("No active market found for this asset.");
        return;
      }

      const market_id = currentMarket.market_id;

      // 2) Derive PDAs
      const programId = program.programId;
      const [marketPDA] = deriveMarketPDA(programId, market_id);
      const [userBetPDA] = deriveUserBetPDA(
        programId,
        walletPubkey,
        marketPDA
      );

      // 3) Prepare instruction data
      const amountU64 = BigInt(Math.round(amountFloat * 1_000_000));

      // FIX: Correct Anchor enum format based on IDL
      const sideEnum =
        side === "GREEN" ? { Green: {} } : { Red: {} };

      console.log("♟ placeBet args:", {
        sideEnum,
        amountU64: amountU64.toString(),
        marketPDA: marketPDA.toBase58(),
        userBetPDA: userBetPDA.toBase58(),
      });

      // 4) Send Transaction
      const txSig = await program.methods
        .placeBet(sideEnum, amountU64)
        .accounts({
          market: marketPDA,
          userBet: userBetPDA,
          user: walletPubkey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      alert(`Bet placed successfully!\nTransaction: ${txSig}`);
    } catch (err: any) {
      console.error("Bet error:", err);
      alert("Bet failed: " + (err.message || err.toString()));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="asset-card w-full bg-[#0f0f0f] shadow-lg border border-[#1f1f1f] rounded-xl p-6">
      {/* HEADER */}
      <div className="asset-header flex items-center justify-between mb-4">
        <span className="asset-title text-xl font-semibold text-white">
          {asset}
        </span>
        <span className="asset-price text-green-400 text-lg font-medium">
          {price ? `$${price.toFixed(2)}` : "$0.00"}
        </span>
      </div>

      {/* CHART */}
      <div className="asset-chart w-full h-[550px] mb-6 rounded-lg overflow-hidden">
        <BackendChart
          key={chartKey}
          containerId={`backend-chart-${asset.replace("/", "")}`}
        />
      </div>

      {/* BETTING SECTION */}
      <div className="asset-actions w-full">
        {/* Amount Input */}
        <div className="amount-input flex items-center bg-black/40 px-3 py-3 rounded-lg border border-gray-700 mb-4">
          <span className="mr-2 text-gray-400">$</span>
          <input
            type="number"
            placeholder="0.00"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            disabled={isLocked || loading}
            className="bg-transparent w-full outline-none text-white text-lg disabled:opacity-50 disabled:cursor-not-allowed"
          />
        </div>

        {/* Bet Buttons */}
        <div className="bet-buttons flex gap-4">
          <button
            onClick={() => handleBet("GREEN")}
            disabled={isLocked || loading}
            className={`flex-1 py-3 rounded-lg transition text-white font-semibold ${
              isLocked || loading
                ? "bg-gray-600 cursor-not-allowed opacity-50"
                : "bg-green-600 hover:bg-green-500"
            }`}
          >
            {loading ? "Processing..." : "GREEN"}
          </button>

          <button
            onClick={() => handleBet("RED")}
            disabled={isLocked || loading}
            className={`flex-1 py-3 rounded-lg transition text-white font-semibold ${
              isLocked || loading
                ? "bg-gray-600 cursor-not-allowed opacity-50"
                : "bg-red-600 hover:bg-red-500"
            }`}
          >
            {loading ? "Processing..." : "RED"}
          </button>
        </div>
      </div>
    </div>
  );
};

export default AssetCard;
