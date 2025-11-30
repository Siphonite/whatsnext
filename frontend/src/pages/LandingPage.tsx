import { Link } from "react-router-dom";
import "../styles/landing.css";
import React from "react";

const LandingPage: React.FC = () => {
  return (
    <div className="landing-container">

      {/* Grid Background */}
      <div className="landing-grid-bg" />

      {/* Main Content */}
      <div className="landing-content">
        <p className="landing-protocol">PROTOCOL LOADED</p>

        <h1 className="landing-title line1">WHAT'S</h1>
        <h1 className="landing-title line2">NEXT</h1>

        <p className="landing-tagline">PREDICT THE TICK. OWN THE FUTURE.</p>

        {/* CTA Button */}
        <Link to="/dashboard" className="landing-button">
          INITIALIZE SYSTEM
        </Link>
      </div>

      {/* Footer */}
      <div className="landing-footer">
        SYSTEM STATUS: <span className="online">ONLINE</span> // LATENCY: 12ms
      </div>
    </div>
  );
};

export default LandingPage;
