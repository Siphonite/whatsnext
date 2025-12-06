import React, { useEffect, useRef } from "react";
import {
  createChart,
  ColorType,
  type IChartApi,
  type ISeriesApi,
} from "lightweight-charts";
import { useOHLCFeed } from "../hooks/useOHLCFeed";

type Props = {
  containerId?: string;
};

/**
 * Backend-driven chart component using lightweight-charts.
 * This chart ONLY displays data from the backend OHLC feed.
 * No external data sources, no tick-based updates.
 */
const BackendChart: React.FC<Props> = ({ containerId = "backend-chart" }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<IChartApi | null>(null);
  const candleSeriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);
  const { candles } = useOHLCFeed();

  // Initialize chart
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
      height: clientHeight,
      crosshair: { mode: 0 },
      timeScale: {
        borderColor: "#222",
        timeVisible: true,
        rightOffset: 12,
        barSpacing: 8,
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

    chartRef.current = chart;
    candleSeriesRef.current = candleSeries;

    // Responsive resize
    const resizeObserver = new ResizeObserver((entries) => {
      if (!entries || entries.length === 0 || !chartRef.current) return;
      const { width, height } = entries[0].contentRect;
      chartRef.current.applyOptions({ width, height });
    });

    resizeObserver.observe(containerRef.current);

    return () => {
      resizeObserver.disconnect();
      if (chartRef.current) {
        chartRef.current.remove();
        chartRef.current = null;
      }
      candleSeriesRef.current = null;
    };
  }, []);

  // Update chart data when candles change
  useEffect(() => {
    if (!candleSeriesRef.current || !candles || candles.length === 0) {
      return;
    }

    // Set all candles at once (backend is source of truth)
    candleSeriesRef.current.setData(candles);
    
    // Auto-fit time scale to show all data
    if (chartRef.current && candles.length > 0) {
      chartRef.current.timeScale().fitContent();
    }
  }, [candles]);

  return (
    <div
      id={containerId}
      ref={containerRef}
      style={{
        width: "100%",
        height: "100%",
        minHeight: 420,
      }}
    />
  );
};

export default BackendChart;

