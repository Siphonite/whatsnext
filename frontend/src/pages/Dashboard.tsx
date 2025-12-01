import React from "react";
import Sidebar from "../components/Sidebar";
import TopBar from "../components/Topbar";
import AssetCard from "../components/AssetCard";
import { SUPPORTED_ASSETS } from "../data/assets";
import "../styles/dashboard.css";

const Dashboard: React.FC = () => {
  return (
    <div className="dashboard-container">
      
      <Sidebar />

      <main className="dashboard-main">
        
        <TopBar />

        {/* WRAP GRID INSIDE A MAX-WIDTH CENTERED CONTAINER */}
        <div className="dashboard-grid-wrapper">
          <div className="dashboard-grid">
            {SUPPORTED_ASSETS.map((asset, index) => (
              <AssetCard key={index} asset={asset} />
            ))}
          </div>
        </div>

      </main>
    </div>
  );
};

export default Dashboard;
