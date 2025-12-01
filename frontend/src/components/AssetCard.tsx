import React from "react";
import type { Asset } from "../data/assets";

interface Props {
  asset: Asset;
}

const AssetCard: React.FC<Props> = ({ asset }) => {
  return (
    <div className="asset-card">
      
      {/* Header */}
      <div className="asset-header">
        <span className="asset-title">{asset.pair}</span>
        {asset.price && (
          <span className="asset-price">${asset.price}</span>
        )}
      </div>

      {/* Fake Chart */}
      <div className="asset-chart">
        <svg viewBox="0 0 300 100" className="chart-svg">
          <path
            d="M0,80 Q30,90 60,50 T120,50 T180,70 T240,30 T300,50"
            fill="none"
            stroke={asset.color}
            strokeWidth="2"
            className="chart-line"
          />
        </svg>
        <span className="chart-status">Waiting for API Stream...</span>
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
