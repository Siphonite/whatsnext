import React, { useEffect, useRef } from "react";
import {
  createChart,
  ColorType,
  type UTCTimestamp,
} from "lightweight-charts";
import { mapToTVSymbol } from "../utils/marketSymbols";
import { useMarketStore } from "../store/useMarketStore";

interface Props {
  symbol: string;
}

const RealChart: React.FC<Props> = ({ symbol }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const candleSeriesRef = useRef<any>(null);

  const { prices } = useMarketStore();
  const livePrice = prices[symbol];

  useEffect(() => {
    if (!containerRef.current) return;

    // Create Chart
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
      height: 120,
      crosshair: { mode: 0 },
    });

    // Add Candle Series
    const candleSeries = (chart as any).addCandlestickSeries({
      upColor: "#22c55e",
      downColor: "#ef4444",
      borderUpColor: "#22c55e",
      borderDownColor: "#ef4444",
      wickUpColor: "#22c55e",
      wickDownColor: "#ef4444",
    });

    candleSeriesRef.current = candleSeries;

    // Load historical data (Binance)
    const fetchHistorical = async () => {
      const tvSymbol = mapToTVSymbol(symbol);

      const url = `https://api.binance.com/api/v3/klines?symbol=${tvSymbol}&interval=4h&limit=200`;

      const response = await fetch(url);
      const data = await response.json();

      const formatted = data.map((c: any) => ({
        time: c[0] / 1000 as UTCTimestamp,
        open: parseFloat(c[1]),
        high: parseFloat(c[2]),
        low: parseFloat(c[3]),
        close: parseFloat(c[4]),
      }));

      candleSeries.setData(formatted);
    };

    fetchHistorical();

    // Resize Handler
    const resizeObserver = new ResizeObserver(() => {
      chart.applyOptions({
        width: containerRef.current!.clientWidth,
      });
    });

    resizeObserver.observe(containerRef.current);

    return () => {
      resizeObserver.disconnect();
      chart.remove();
    };
  }, [symbol]);

  // Live Updating Candle
  useEffect(() => {
    if (!candleSeriesRef.current || !livePrice) return;

    candleSeriesRef.current.update({
      time: Math.floor(Date.now() / 1000) as UTCTimestamp,
      open: livePrice,
      high: livePrice,
      low: livePrice,
      close: livePrice,
    });
  }, [livePrice]);

  return (
    <div
      ref={containerRef}
      className="real-chart-container"
      style={{ width: "100%", height: "120px", position: "relative" }}
    ></div>
  );
};

export default RealChart;
