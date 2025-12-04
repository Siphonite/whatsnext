import React from "react";
import type { Asset } from "../data/assets";
import { useMarketStore } from "../store/useMarketStore";
import RealChart from "./RealChart";

interface Props {
  asset: Asset;
}

const AssetCard: React.FC<Props> = ({ asset }) => {
  const { prices } = useMarketStore();
  const livePrice = prices[asset.pair];

  return (
    <div className="asset-card">
      
      {/* Header */}
      <div className="asset-header">
        <span className="asset-title">{asset.pair}</span>

        <span className="asset-price">
          {livePrice ? `$${livePrice.toFixed(2)}` : "$0.00"}
        </span>
      </div>

      {/* REAL TradingView Chart */}
      <div className="asset-chart real-chart-wrapper">
        <RealChart symbol={asset.pair} />
      </div>

      {/* Betting Section */}
      <div className="asset-actions">
        <div className="amount-input">
          <span>$</span>
          <input type="number" placeholder="0.00" />
        </div>

        <div className="bet-buttons">
          <button className="bet-green">GREEN</button>
          <button className="bet-red">RED</button>
        </div>
      </div>

    </div>
  );
};

export default AssetCard;
