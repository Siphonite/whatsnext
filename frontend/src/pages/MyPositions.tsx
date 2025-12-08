import React, { useEffect, useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import Sidebar from "../components/Sidebar";
import TopBar from "../components/Topbar";
import Tabs from "../components/Tabs";
import PositionsTable from "../components/PositionsTable";
import { usePositionsStore } from "../store/usePositionsStore";
import "../styles/dashboard.css";

const MyPositions: React.FC = () => {
  const { publicKey } = useWallet();
  const { 
    openPositions, 
    settledPositions, 
    pnlSummary, 
    loading, 
    fetchPositions, 
    fetchPnl 
  } = usePositionsStore();
  
  const [activeTab, setActiveTab] = useState<string>("open");

  // Fetch data on mount and when wallet changes
  useEffect(() => {
    if (!publicKey) {
      return;
    }

    const wallet = publicKey.toString();
    
    // Initial fetch
    fetchPositions(wallet);
    fetchPnl(wallet);

    // Auto-refresh every 12 seconds
    const interval = setInterval(() => {
      fetchPositions(wallet);
      fetchPnl(wallet);
    }, 12000);

    return () => clearInterval(interval);
  }, [publicKey, fetchPositions, fetchPnl]);

  const tabs = [
    { id: "open", label: "Open Positions" },
    { id: "settled", label: "Settled Positions" },
    { id: "overview", label: "PnL Overview" },
  ];

  // Format currency
  const formatCurrency = (value: number) => {
    const sign = value >= 0 ? "+" : "";
    return `${sign}$${Math.abs(value).toFixed(2)}`;
  };

  // Format percentage
  const formatPercent = (value: number) => {
    return `${value.toFixed(1)}%`;
  };

  return (
    <div className="dashboard-container">
      {/* LEFT SIDEBAR */}
      <Sidebar />

      {/* MAIN AREA */}
      <main className="dashboard-main">
        {/* TOP BAR */}
        <TopBar />

        {/* CONTENT */}
        <div className="dashboard-content px-6 py-6 overflow-y-auto">
          {/* HEADER SECTION */}
          <div className="mb-8">
            <h1 className="text-3xl font-bold text-white mb-2 font-cyber">
              My Positions
            </h1>
            
            {/* PnL SUMMARY CARDS */}
            {publicKey && pnlSummary && (
              <div className="grid grid-cols-3 gap-4 mt-6">
                {/* Total PnL */}
                <div className="bg-[#0f0f0f] border border-gray-800 rounded-xl p-4 hover:border-cyan-500/30 transition-all">
                  <p className="text-gray-400 text-sm mb-2">Total PnL</p>
                  <p className={`text-2xl font-bold font-mono ${
                    pnlSummary.totalPnl >= 0 ? "text-green-400" : "text-red-400"
                  }`}>
                    {formatCurrency(pnlSummary.totalPnl)}
                  </p>
                </div>

                {/* Win Rate */}
                <div className="bg-[#0f0f0f] border border-gray-800 rounded-xl p-4 hover:border-cyan-500/30 transition-all">
                  <p className="text-gray-400 text-sm mb-2">Win Rate</p>
                  <p className="text-2xl font-bold font-mono text-cyan-400">
                    {formatPercent(pnlSummary.winRate)}
                  </p>
                </div>

                {/* Current Streak */}
                <div className="bg-[#0f0f0f] border border-gray-800 rounded-xl p-4 hover:border-cyan-500/30 transition-all">
                  <p className="text-gray-400 text-sm mb-2">Current Streak</p>
                  <p className="text-2xl font-bold font-mono text-cyan-400">
                    {pnlSummary.streak}
                  </p>
                </div>
              </div>
            )}

            {!publicKey && (
              <div className="mt-6 p-6 bg-[#0f0f0f] border border-gray-800 rounded-xl text-center">
                <p className="text-gray-400">Please connect your wallet to view positions</p>
              </div>
            )}
          </div>

          {/* TABS AND CONTENT */}
          {publicKey && (
            <>
              <Tabs tabs={tabs} activeTab={activeTab} onTabChange={setActiveTab} />

              {/* TAB CONTENT */}
              <div className="mt-6">
                {activeTab === "open" && (
                  <div className="bg-[#0f0f0f] border border-gray-800 rounded-xl p-6">
                    <h2 className="text-xl font-semibold text-white mb-4">Open Positions</h2>
                    <PositionsTable 
                      positions={openPositions} 
                      showPayout={false}
                      isLoading={loading}
                    />
                  </div>
                )}

                {activeTab === "settled" && (
                  <div className="bg-[#0f0f0f] border border-gray-800 rounded-xl p-6">
                    <h2 className="text-xl font-semibold text-white mb-4">Settled Positions</h2>
                    <PositionsTable 
                      positions={settledPositions} 
                      showPayout={true}
                      isLoading={loading}
                    />
                  </div>
                )}

                {activeTab === "overview" && (
                  <div className="bg-[#0f0f0f] border border-gray-800 rounded-xl p-6">
                    <h2 className="text-xl font-semibold text-white mb-4">PnL Overview</h2>
                    {pnlSummary ? (
                      <div className="space-y-4">
                        <div className="grid grid-cols-2 gap-4">
                          <div className="p-4 bg-black/40 rounded-lg border border-gray-700">
                            <p className="text-gray-400 text-sm mb-1">Total PnL</p>
                            <p className={`text-xl font-bold font-mono ${
                              pnlSummary.totalPnl >= 0 ? "text-green-400" : "text-red-400"
                            }`}>
                              {formatCurrency(pnlSummary.totalPnl)}
                            </p>
                          </div>
                          <div className="p-4 bg-black/40 rounded-lg border border-gray-700">
                            <p className="text-gray-400 text-sm mb-1">Win Rate</p>
                            <p className="text-xl font-bold font-mono text-cyan-400">
                              {formatPercent(pnlSummary.winRate)}
                            </p>
                          </div>
                        </div>
                        <div className="p-4 bg-black/40 rounded-lg border border-gray-700">
                          <p className="text-gray-400 text-sm mb-1">Current Streak</p>
                          <p className="text-2xl font-bold font-mono text-cyan-400">
                            {pnlSummary.streak} {pnlSummary.streak === 1 ? "win" : "wins"}
                          </p>
                        </div>
                        <div className="mt-6 pt-6 border-t border-gray-700">
                          <p className="text-gray-400 text-sm mb-2">Statistics</p>
                          <div className="grid grid-cols-2 gap-4 text-sm">
                            <div>
                              <span className="text-gray-500">Open Positions:</span>
                              <span className="ml-2 text-white font-semibold">{openPositions.length}</span>
                            </div>
                            <div>
                              <span className="text-gray-500">Settled Positions:</span>
                              <span className="ml-2 text-white font-semibold">{settledPositions.length}</span>
                            </div>
                          </div>
                        </div>
                      </div>
                    ) : (
                      <div className="text-center py-12 text-gray-500">
                        <p>Loading PnL data...</p>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </>
          )}
        </div>
      </main>
    </div>
  );
};

export default MyPositions;

