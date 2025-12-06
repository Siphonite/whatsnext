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

    // Get current container dimensions
    const { clientWidth, clientHeight } = containerRef.current;

    const chart = createChart(containerRef.current, {
      layout: {
        background: { type: ColorType.Solid, color: "#0a0a0a" },
        textColor: "#ffffff",
      },
      grid: {
        vertLines: { color: "#1f1f1f" },
        horzLines: { color: "#1f1f1f" },
      },
      width: clientWidth,
      height: clientHeight, // Dynamically use available height
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

        // NO FALLBACK - Backend is the only source of truth
        // If backend fails, chart will remain empty until backend is available
        console.warn("Backend OHLC endpoint unavailable. Chart will remain empty.");
      } catch (err) {
        console.error("Historical candle load failed:", err);
      }
    };

    fetchHistorical();

    // Responsive Resize
    // This ensures the chart resizes BOTH width AND height
    const resizeObserver = new ResizeObserver((entries) => {
      if (!entries || entries.length === 0) return;
      const { width, height } = entries[0].contentRect;
      chart.applyOptions({ width, height });
    });
    
    resizeObserver.observe(containerRef.current);

    return () => {
      resizeObserver.disconnect();
      chart.remove();
    };
  }, [asset, tvSymbol]);

  // NOTE: Tick-based candle updates removed.
  // Candles must come exclusively from backend OHLC feed.
  // Price ticks are only for spot price display, not chart data.

  return (
    <div
      ref={containerRef}
      className="real-chart-container"
      style={{
        width: "100%",
        height: "100%", // Fill the flex parent
        position: "relative",
        overflow: "hidden"
      }}
    />
  );
};

export default RealChart;