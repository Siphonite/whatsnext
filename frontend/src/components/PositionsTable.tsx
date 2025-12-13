import React, { useState } from "react";
import type { BetPosition } from "../store/usePositionsStore";
import { useWallet } from "@solana/wallet-adapter-react";
import { useAnchorProgram } from "../hooks/useAnchorProgram";

interface PositionsTableProps {
  positions: BetPosition[];
  showPayout?: boolean;
  isLoading?: boolean;
}

const PositionsTable: React.FC<PositionsTableProps> = ({
  positions,
  showPayout = false,
  isLoading = false,
}) => {
  const { publicKey } = useWallet();
  const {
    program,
    deriveMarketPda,
    deriveUserBetPda,
    deriveTreasuryPda,
  } = useAnchorProgram();

  const [claiming, setClaiming] = useState<number | null>(null);
  const [claimedMarkets, setClaimedMarkets] = useState<Set<number>>(new Set());

  // ------------------------
  // Time formatting
  // ------------------------
  const formatTime = (timestamp: number) => {
    const diff = Date.now() / 1000 - timestamp;
    if (diff < 60) return `${Math.floor(diff)}s ago`;
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    return new Date(timestamp * 1000).toLocaleDateString();
  };

  // ------------------------
  // Claim reward
  // ------------------------
  const handleClaim = async (position: BetPosition) => {
    if (!program || !publicKey) return;

    try {
      setClaiming(position.marketId);

      const marketPda = deriveMarketPda(position.marketId);
      const userBetPda = deriveUserBetPda(publicKey, marketPda!);
      const treasuryPda = deriveTreasuryPda();

      if (!marketPda || !userBetPda || !treasuryPda) {
        throw new Error("Failed to derive PDAs");
      }

      // 1️⃣ On-chain claim
      const txSig = await program.methods
        .claimReward()
        .accounts({
          market: marketPda,
          userBet: userBetPda,
          treasury: treasuryPda,
          user: publicKey,
        })
        .rpc();

      // 2️⃣ Backend record
      await fetch("/api/claim/record", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          market_id: position.marketId,
          wallet: publicKey.toString(),
          tx_sig: txSig,
        }),
      });

      // 3️⃣ Optimistic UI lock
      setClaimedMarkets((prev) => new Set(prev).add(position.marketId));

      alert("Reward claimed successfully");
    } catch (err: any) {
      console.error("Claim failed:", err);
      alert(err.message || "Claim failed");
    } finally {
      setClaiming(null);
    }
  };

  // ------------------------
  // Loading / Empty
  // ------------------------
  if (isLoading) {
    return (
      <div className="animate-pulse space-y-4">
        {[1, 2, 3].map((i) => (
          <div key={i} className="h-16 bg-gray-800 rounded-lg"></div>
        ))}
      </div>
    );
  }

  if (positions.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        No positions found
      </div>
    );
  }

  // ------------------------
  // Table
  // ------------------------
  return (
    <div className="w-full overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b border-gray-700">
            <th className="px-4 py-3 text-left text-gray-400">Side</th>
            <th className="px-4 py-3 text-left text-gray-400">Amount</th>
            <th className="px-4 py-3 text-left text-gray-400">Weight</th>
            <th className="px-4 py-3 text-left text-gray-400">Effective</th>
            <th className="px-4 py-3 text-left text-gray-400">
              {showPayout ? "Payout / Action" : "Time"}
            </th>
          </tr>
        </thead>

        <tbody>
          {positions.map((p) => {
            const isGreen = p.side === "GREEN";
            const alreadyClaimed = claimedMarkets.has(p.marketId);

            const canClaim =
              showPayout &&
              p.status === "SETTLED" &&
              p.payout !== undefined &&
              p.payout > 0 &&
              !alreadyClaimed;

            return (
              <tr
                key={p.marketId}
                className="border-b border-gray-800 hover:bg-gray-800/30"
              >
                <td className="px-4 py-4">
                  <span
                    className={`px-3 py-1 rounded text-xs font-bold ${
                      isGreen
                        ? "bg-green-500/20 text-green-400"
                        : "bg-red-500/20 text-red-400"
                    }`}
                  >
                    {p.side}
                  </span>
                </td>

                <td className="px-4 py-4 text-white font-mono">
                  ${p.amount.toFixed(2)}
                </td>

                <td className="px-4 py-4 text-gray-300">
                  {p.weight.toFixed(2)}x
                </td>

                <td className="px-4 py-4 text-white font-mono">
                  ${p.effectiveStake.toFixed(2)}
                </td>

                <td className="px-4 py-4">
                  {showPayout ? (
                    <div className="flex items-center gap-3">
                      <span className="font-mono text-white">
                        {p.payout !== undefined
                          ? `$${p.payout.toFixed(2)}`
                          : "—"}
                      </span>

                      {canClaim && (
                        <button
                          onClick={() => handleClaim(p)}
                          disabled={claiming === p.marketId}
                          className="px-3 py-1 text-xs rounded bg-cyan-600 hover:bg-cyan-500 disabled:opacity-50"
                        >
                          {claiming === p.marketId ? "Claiming…" : "Claim"}
                        </button>
                      )}

                      {alreadyClaimed && (
                        <span className="text-xs text-green-400">Claimed</span>
                      )}
                    </div>
                  ) : (
                    <span className="text-gray-400">
                      {formatTime(p.timestamp)}
                    </span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default PositionsTable;
