// src/components/TradingViewChart.tsx
import React, { useEffect, useRef } from "react";
import { mapToTVSymbol } from "../utils/marketSymbols";

type Props = {
  asset: string;           // e.g. "BTC/USDT"
  interval?: string;       // e.g. "240" for 4H (TradingView uses minutes for numeric intervals)
  theme?: "dark" | "light";
  containerId?: string;
};

const TradingViewChart: React.FC<Props> = ({
  asset,
  interval = "240", // 4H
  theme = "dark",
  containerId = "tv-chart-container",
}) => {
  const widgetRef = useRef<any>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    // prevent double-loading if component re-mounts
    let tvScript = document.getElementById("tradingview-widget-script") as HTMLScriptElement | null;

    const tvSymbol = mapToTVSymbol(asset);

    const createWidget = () => {
      try {
        // @ts-ignore - TradingView global
        if ((window as any).TradingView && containerRef.current) {
          // remove previous widget if present
          if (widgetRef.current && widgetRef.current.remove) {
            try { widgetRef.current.remove(); } catch (e) {}
            widgetRef.current = null;
          }

          // TradingView widget constructor
          // @ts-ignore
          widgetRef.current = new (window as any).TradingView.widget({
            width: "100%",
            height: "100%",
            symbol: tvSymbol,
            interval: interval,
            container_id: containerId,
            timezone: "Etc/UTC",
            theme: theme === "dark" ? "dark" : "light",
            style: "1", // 1=candlesticks default
            locale: "en",
            // saves settings to local storage (optional)
            enable_publishing: false,
            allow_symbol_change: true,
            range: "YTD",
            details: true,
            studies: [
              // example indicators:
              "MASimple@tv-basicstudies",
              "RSI@tv-basicstudies",
            ],
            withdateranges: true,
            hide_side_toolbar: false,
            hideideas: true,
            toolbar_bg: theme === "dark" ? "#0b0e14" : "#ffffff",
            // This will show the built-in countdown & status; TradingView handles history & timer itself
            overrides: {},
            studies_overrides: {},
          });
        }
      } catch (err) {
        // widget may throw if symbol not supported by TradingView
        console.error("TradingView widget init error:", err);
      }
    };

    // If script not loaded, inject it then create the widget on load
    if (!tvScript) {
      tvScript = document.createElement("script");
      tvScript.id = "tradingview-widget-script";
      tvScript.src = "https://s3.tradingview.com/tv.js";
      tvScript.async = true;
      tvScript.onload = () => {
        createWidget();
      };
      document.head.appendChild(tvScript);
    } else {
      // script present: create widget immediately (it may still be initializing; guard inside createWidget)
      createWidget();
    }

    // cleanup on unmount
    return () => {
      try {
        if (widgetRef.current && widgetRef.current.remove) {
          widgetRef.current.remove();
        }
      } catch (e) {}
      widgetRef.current = null;
    };
    // Recreate widget when asset or interval changes
  }, [asset, interval, theme, containerId]);

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

export default TradingViewChart;
