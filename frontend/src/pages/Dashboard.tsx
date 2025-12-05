import React from "react";
import Sidebar from "../components/Sidebar";
import TopBar from "../components/Topbar";
import AssetCard from "../components/AssetCard";
import "../styles/dashboard.css";

// Live price feed (BTCUSDT only)
import { usePriceFeed } from "../hooks/usePriceFeed";

const Dashboard: React.FC = () => {
  usePriceFeed(); // Activate single-asset price feed

  return (
    <div className="dashboard-container">
      {/* LEFT SIDEBAR */}
      <Sidebar />

      {/* MAIN AREA */}
      <main className="dashboard-main">
        {/* TOP BAR */}
        <TopBar />

        {/* FULL WIDTH CHART WRAPPER */}
        {/* Removed 'justify-center' to allow full expansion naturally */}
        <div className="dashboard-single-wrapper w-full px-6 mt-4">
          <AssetCard />
        </div>
      </main>
    </div>
  );
};

export default Dashboard;