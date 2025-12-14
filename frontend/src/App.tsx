import React, { Suspense } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

// Pages
import LandingPage from "./pages/LandingPage";
import Dashboard from "./pages/Dashboard";
import MyPositions from "./pages/MyPositions";
import Leaderboard from "./pages/Leaderboard";
import Wallet from "./pages/Wallet";

const App: React.FC = () => {
  return (
    <BrowserRouter>
      {/* Optional suspense fallback */}
      <Suspense fallback={<div style={{ color: "white" }}>Loading...</div>}>
        <Routes>
          <Route path="/" element={<LandingPage />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/positions" element={<MyPositions />} />
          <Route path="/leaderboard" element={<Leaderboard />} />
          <Route path="/wallet" element={<Wallet />} />
        </Routes>
      </Suspense>
    </BrowserRouter>
  );
};

export default App;