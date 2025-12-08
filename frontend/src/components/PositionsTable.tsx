import React from "react";
import type { BetPosition } from "../store/usePositionsStore";

interface PositionsTableProps {
  positions: BetPosition[];
  showPayout?: boolean;
  isLoading?: boolean;
}

const PositionsTable: React.FC<PositionsTableProps> = ({ 
  positions, 
  showPayout = false,
  isLoading = false 
}) => {
  // Format timestamp to readable time
  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const now = Date.now() / 1000;
    const diff = now - timestamp;
    
    if (diff < 60) return `${Math.floor(diff)}s ago`;
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    return date.toLocaleDateString();
  };

  // Calculate time left for open positions
  const getTimeLeft = (timestamp: number) => {
    // For open positions, we'd need market end_time
    // For now, just show when the bet was placed
    return formatTime(timestamp);
  };

  if (isLoading) {
    return (
      <div className="w-full">
        <div className="animate-pulse space-y-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="h-16 bg-gray-800 rounded-lg"></div>
          ))}
        </div>
      </div>
    );
  }

  if (positions.length === 0) {
    return (
      <div className="w-full text-center py-12 text-gray-500">
        <p className="text-lg">No positions found</p>
        <p className="text-sm mt-2">Your {showPayout ? "settled" : "open"} positions will appear here</p>
      </div>
    );
  }

  return (
    <div className="w-full overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b border-gray-700">
            <th className="text-left py-3 px-4 text-gray-400 font-semibold text-sm">Side</th>
            <th className="text-left py-3 px-4 text-gray-400 font-semibold text-sm">Amount</th>
            <th className="text-left py-3 px-4 text-gray-400 font-semibold text-sm">Weight</th>
            <th className="text-left py-3 px-4 text-gray-400 font-semibold text-sm">Effective Stake</th>
            <th className="text-left py-3 px-4 text-gray-400 font-semibold text-sm">
              {showPayout ? "Payout" : "Time Left"}
            </th>
          </tr>
        </thead>
        <tbody>
          {positions.map((position, index) => {
            const isGreen = position.side === "GREEN";
            return (
              <tr
                key={`${position.marketId}-${index}`}
                className="border-b border-gray-800 hover:bg-gray-800/30 transition-colors cursor-pointer"
                style={{
                  boxShadow: "0 0 0 rgba(6, 182, 212, 0)",
                  transition: "box-shadow 0.2s ease",
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.boxShadow = "0 0 15px rgba(6, 182, 212, 0.3)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.boxShadow = "0 0 0 rgba(6, 182, 212, 0)";
                }}
              >
                <td className="py-4 px-4">
                  <span
                    className={`
                      inline-block px-3 py-1 rounded text-xs font-bold
                      ${isGreen 
                        ? "bg-green-500/20 text-green-400 border border-green-500/30" 
                        : "bg-red-500/20 text-red-400 border border-red-500/30"
                      }
                    `}
                    style={{
                      textShadow: isGreen 
                        ? "0 0 8px rgba(34, 197, 94, 0.5)" 
                        : "0 0 8px rgba(239, 68, 68, 0.5)",
                    }}
                  >
                    {position.side}
                  </span>
                </td>
                <td className="py-4 px-4 text-white font-mono">
                  ${position.amount.toFixed(2)}
                </td>
                <td className="py-4 px-4 text-gray-300">
                  {position.weight.toFixed(2)}x
                </td>
                <td className="py-4 px-4 text-white font-mono">
                  ${position.effectiveStake.toFixed(2)}
                </td>
                <td className="py-4 px-4">
                  {showPayout ? (
                    <span className={`font-mono font-semibold ${
                      position.payout && position.payout > position.effectiveStake
                        ? "text-green-400"
                        : position.payout && position.payout < position.effectiveStake
                        ? "text-red-400"
                        : "text-gray-400"
                    }`}>
                      {position.payout ? `$${position.payout.toFixed(2)}` : "N/A"}
                    </span>
                  ) : (
                    <span className="text-gray-400 text-sm">
                      {getTimeLeft(position.timestamp)}
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

