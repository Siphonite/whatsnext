import { useEffect } from "react";
import { useMarketStore } from "../store/useMarketStore";

export const usePriceFeed = () => {
  const { markets, setPrice } = useMarketStore();

  useEffect(() => {
    const fetchPrices = async () => {
      for (const m of markets) {
        try {
          let price = null;

          // CRYPTO: Binance
          if (m.symbol.includes("/USDT")) {
            const pair = m.symbol.replace("/", "");
            const res = await fetch(
              `https://api.binance.com/api/v3/ticker/price?symbol=${pair}`
            );
            const data = await res.json();
            price = parseFloat(data.price);
          }

          // GOLD & SILVER using goldprice
          else if (m.symbol === "GOLD" || m.symbol === "SILVER") {
            const res = await fetch("https://data-asg.goldprice.org/dbXRates/USD");
            const data = await res.json();
            if (m.symbol === "GOLD") price = data.items[0].xauPrice;
            if (m.symbol === "SILVER") price = data.items[0].xagPrice;
          }

          // OIL using Yahoo Finance
          else if (m.symbol === "OIL") {
            const res = await fetch(
              "https://query1.finance.yahoo.com/v8/finance/chart/CL=F"
            );
            const data = await res.json();
            price = data.chart.result[0].meta.regularMarketPrice;
          }

          // FOREX using open.er-api.com (reliable)
          else {
            const [base, quote] = m.symbol.split("/");
            const res = await fetch(`https://open.er-api.com/v6/latest/${quote}`);
            const data = await res.json();
            if (data && data.rates) {
              price = data.rates[base] ? 1 / data.rates[base] : null;
            }
          }

          if (price) setPrice(m.symbol, price);

        } catch (err) {
          console.error("Price Fetch Failed for", m.symbol, err);
        }
      }
    };

    fetchPrices();
    const interval = setInterval(fetchPrices, 7000);

    return () => clearInterval(interval);
  }, [markets, setPrice]);
};
