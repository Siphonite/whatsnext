import React from "react";
import Sidebar from "../components/Sidebar";
import TopBar from "../components/Topbar";
import AssetCard from "../components/AssetCard";
import "../styles/dashboard.css";

// Live price feed (BTCUSDT only)
import { usePriceFeed } from "../hooks/usePriceFeed";

const Dashboard: React.FC = () => {
  // activate single-asset live price feed
  usePriceFeed();

  return (
    <div className="dashboard-container">

      {/* LEFT SIDEBAR */}
      <Sidebar />

      {/* MAIN AREA */}
      <main className="dashboard-main">

        {/* TOP BAR */}
        <TopBar />

        {/* Single Asset Card */}
        <div className="dashboard-single-wrapper">
          <AssetCard /> 
        </div>

      </main>

    </div>
  );
};

export default Dashboard;
