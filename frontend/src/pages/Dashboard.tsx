import React from "react";
import Sidebar from "../components/Sidebar";
import TopBar from "../components/Topbar";
import AssetCard from "../components/AssetCard";
import { SUPPORTED_ASSETS } from "../data/assets";
import "../styles/dashboard.css";

const Dashboard: React.FC = () => {
  return (
    <div className="dashboard-container">
      
      {/* Sidebar */}
      <Sidebar />

      {/* Main Section */}
      <main className="dashboard-main">
        
        {/* Top Bar */}
        <TopBar />

        {/* Asset Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 pb-10">
          {SUPPORTED_ASSETS.map((asset, index) => (
            <AssetCard key={index} asset={asset} />
          ))}
        </div>

      </main>
    </div>
  );
};

export default Dashboard;
