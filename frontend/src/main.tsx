import { Buffer } from "buffer";

(window as any).Buffer = Buffer;

import "@solana/wallet-adapter-react-ui/styles.css";
import { WalletContext } from "./contexts/WalletContext.tsx";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <WalletContext>
      <App />
    </WalletContext>
  </React.StrictMode>
);
