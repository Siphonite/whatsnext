import React, { useEffect, useRef } from "react";
import {
  createChart,
  ColorType,
  type UTCTimestamp,
} from "lightweight-charts";
import { useMarketStore } from "../store/useMarketStore";

const RealChart: React.FC = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const candleSeriesRef = useRef<any>(null);

  // Single asset from your store
  const { asset, tvSymbol, price } = useMarketStore();

  // -----------------------------
  // 1. INIT CHART + LOAD HISTORY
  // -----------------------------
  useEffect(() => {
    if (!containerRef.current) return;

    const chart = createChart(containerRef.current, {
      layout: {
        background: { type: ColorType.Solid, color: "#0a0a0a" },
        textColor: "#ffffff",
      },
      grid: {
        vertLines: { color: "#1f1f1f" },
        horzLines: { color: "#1f1f1f" },
      },
      width: containerRef.current.clientWidth,
      height: 550, // Match the height in AssetCard css
      crosshair: { mode: 0 },
      timeScale: {
        borderColor: "#222",
        timeVisible: true,
      },
      rightPriceScale: {
        borderColor: "#222",
      },
    });

    const candleSeries = chart.addCandlestickSeries({
      upColor: "#22c55e",
      downColor: "#ef4444",
      borderUpColor: "#22c55e",
      borderDownColor: "#ef4444",
      wickUpColor: "#22c55e",
      wickDownColor: "#ef4444",
    });

    candleSeriesRef.current = candleSeries;

    // Fetch historical candles
    const fetchHistorical = async () => {
      try {
        // Prefer BACKEND candles (more accurate for WhatsNext)
        const backendURL = `/api/prices/${asset}`;
        const backendRes = await fetch(backendURL);

        if (backendRes.ok) {
          const json = await backendRes.json();

          if (json?.candles) {
            const formatted = json.candles.map((c: any) => ({
              time: c.timestamp as UTCTimestamp,
              open: c.open,
              high: c.high,
              low: c.low,
              close: c.close,
            }));

            candleSeries.setData(formatted);
            return; // done
          }
        }

        // FALLBACK → Binance historical 4H candles
        const res = await fetch(
          `https://api.binance.com/api/v3/klines?symbol=${tvSymbol}&interval=4h&limit=200`
        );
        const data = await res.json();

        const converted = data.map((c: any) => ({
          time: (c[0] / 1000) as UTCTimestamp,
          open: parseFloat(c[1]),
          high: parseFloat(c[2]),
          low: parseFloat(c[3]),
          close: parseFloat(c[4]),
        }));

        candleSeries.setData(converted);
      } catch (err) {
        console.error("Historical candle load failed:", err);
      }
    };

    fetchHistorical();

    // Responsive Resize
    // This ensures the chart resizes when the window or container resizes
    const resizeObserver = new ResizeObserver(() => {
      if (containerRef.current) {
        chart.applyOptions({
          width: containerRef.current.clientWidth,
        });
      }
    });
    resizeObserver.observe(containerRef.current);

    return () => {
      resizeObserver.disconnect();
      chart.remove();
    };
  }, [asset, tvSymbol]);

  // -----------------------------
  // 2. LIVE PRICE CANDLE UPDATE
  // -----------------------------
  useEffect(() => {
    if (!candleSeriesRef.current || !price) return;

    const timestamp = Math.floor(Date.now() / 1000) as UTCTimestamp;

    candleSeriesRef.current.update({
      time: timestamp,
      open: price,
      high: price,
      low: price,
      close: price,
    });
  }, [price]);

  return (
    <div
      ref={containerRef}
      className="real-chart-container"
      style={{
        width: "100%",
        height: "100%", // Fill the parent div
        position: "relative",
      }}
    />
  );
};

export default RealChart;