import { useState } from 'react';
import { LayoutDashboard, Wallet, Trophy, ChevronLeft, ChevronRight } from 'lucide-react';

export const Sidebar = () => {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside 
      className={`h-full glass-panel border-r border-gray-800 flex flex-col justify-between relative z-20 transition-all duration-300 ${
        collapsed ? 'w-20' : 'w-1/5 min-w-[250px]'
      }`}
    >
      {/* Header */}
      <div className="p-6 flex items-center justify-between">
        {!collapsed && (
          <h2 className="font-cyber text-xl text-cyan-400 tracking-wider whitespace-nowrap">
            WN_PROTOCOL
          </h2>
        )}
        <button 
          onClick={() => setCollapsed(!collapsed)} 
          className="text-gray-400 hover:text-white transition p-1"
        >
          {collapsed ? <ChevronRight size={24} /> : <ChevronLeft size={24} />}
        </button>
      </div>

      {/* Nav Links */}
      <nav className="flex-1 px-4 space-y-4 mt-4">
        {[
          { name: 'Live Markets', icon: <LayoutDashboard size={24} /> },
          { name: 'Wallet', icon: <Wallet size={24} /> },
          { name: 'Leaderboard', icon: <Trophy size={24} /> },
        ].map((item) => (
          <a key={item.name} href="#" className="flex items-center p-3 text-gray-300 rounded-lg hover:bg-cyan-900/20 hover:text-cyan-400 transition group">
            {item.icon}
            {!collapsed && <span className="ml-3">{item.name}</span>}
          </a>
        ))}
      </nav>

      {/* User Balance */}
      <div className="p-4 border-t border-gray-800 bg-black/20 overflow-hidden">
        <div className="flex items-center">
          <div className="w-10 h-10 min-w-[2.5rem] rounded-full bg-gradient-to-tr from-cyan-500 to-blue-600 flex items-center justify-center font-bold text-white">
            U
          </div>
          {!collapsed && (
            <div className="ml-3">
              <p className="text-xs text-gray-400">Balance</p>
              <p className="text-sm font-mono text-white font-bold">$12,450.00</p>
            </div>
          )}
        </div>
      </div>
    </aside>
  );
};