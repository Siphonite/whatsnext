import React from "react";

const Leaderboard: React.FC = () => {
  return (
    <div className="flex flex-col items-center justify-center h-full w-full text-center text-white px-6">
      <h1 className="text-4xl font-bold text-cyan-400 mb-4 drop-shadow-lg">
        Leaderboard
      </h1>

      <p className="text-gray-400 text-lg max-w-xl">
        This feature is currently under development.
      </p>

      <p className="text-cyan-300 mt-2 text-sm uppercase tracking-widest opacity-80">
        Coming Soon
      </p>

      {/* Optional glowing animation */}
      <div className="mt-8 w-20 h-20 border-2 border-cyan-400 rounded-full animate-ping opacity-40"></div>
    </div>
  );
};

export default Leaderboard;
