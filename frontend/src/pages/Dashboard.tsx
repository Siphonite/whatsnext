import React from "react";
import Sidebar from "../components/Sidebar";
import TopBar from "../components/Topbar";
import AssetCard from "../components/AssetCard";
import "../styles/dashboard.css";

// Zustand store
import { useMarketStore } from "../store/useMarketStore";

// Live price feed hook
import { usePriceFeed } from "../hooks/usePriceFeed";

// Full asset metadata
import { SUPPORTED_ASSETS } from "../data/assets";

const Dashboard: React.FC = () => {

  const { markets, activeAsset, setActiveAsset } = useMarketStore();

  // 🔥 Activate live price feed (updates every 5 seconds)
  usePriceFeed();

  return (
    <div className="dashboard-container">

      <Sidebar />

      <main className="dashboard-main">

        <TopBar />

        <div className="dashboard-grid-wrapper">
          <div className="dashboard-grid">

            {markets.map((market, index) => {
              const asset = SUPPORTED_ASSETS.find(
                (a) => a.pair === market.symbol
              );

              return (
                <div
                  key={index}
                  onClick={() => setActiveAsset(market.symbol)}
                  className={`asset-card-wrapper ${
                    activeAsset === market.symbol ? "active-asset" : ""
                  }`}
                >
                  {asset && <AssetCard asset={asset} />}
                </div>
              );
            })}

          </div>
        </div>

      </main>

    </div>
  );
};

export default Dashboard;
