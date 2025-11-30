import { Sidebar } from '../components/Sidebar';

// Mock Data (We will fetch this from backend later)
const ASSETS = [
  { pair: 'BTC/USDT', price: '64,230.50', color: '#f7931a' },
  { pair: 'ETH/USDT', price: '3,450.12', color: '#627eea' },
  { pair: 'SOL/USDT', price: '145.60', color: '#00ffa3' },
  { pair: 'BNB/USDT', price: '590.20', color: '#f3ba2f' },
  { pair: 'XRP/USDT', price: '0.6230', color: '#ffffff' },
  { pair: 'ADA/USDT', price: '0.4500', color: '#0033ad' },
  { pair: 'DOGE/USDT', price: '0.1600', color: '#ba9f33' },
  { pair: 'DOT/USDT', price: '7.20', color: '#e6007a' },
  { pair: 'LINK/USDT', price: '18.40', color: '#2a5ada' },
];

export const Dashboard = () => {
  return (
    <div className="flex h-screen overflow-hidden bg-[url('https://assets.codepen.io/1462889/bg_stars.png')] text-white">
      
      {/* Sidebar Component */}
      <Sidebar />

      {/* Main Content */}
      <main className="flex-1 h-full overflow-y-auto p-6 relative z-10">
        
        {/* Header */}
        <header className="flex justify-between items-center mb-8">
          <div>
            <h1 className="font-cyber text-2xl text-white">MARKET PREDICTION</h1>
            <p className="text-gray-500 text-xs mt-1">LIVE FEED // SELECT 9 ASSETS</p>
          </div>
          <div className="flex space-x-4">
            <div className="px-4 py-2 glass-panel rounded text-xs font-mono text-green-400 border border-green-500/20">
              ● SYSTEM OPTIMAL
            </div>
            <div className="px-4 py-2 glass-panel rounded text-xs font-mono text-gray-300 border border-gray-700">
              00:00:00
            </div>
          </div>
        </header>

        {/* Chart Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 pb-10">
          {ASSETS.map((asset) => (
            <div key={asset.pair} className="glass-panel rounded-xl p-4 border border-gray-700 hover:border-cyan-500/30 transition duration-300 flex flex-col group">
              
              {/* Asset Header */}
              <div className="flex justify-between items-center mb-4">
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 rounded-full shadow-[0_0_8px]" style={{ backgroundColor: asset.color, boxShadow: `0 0 10px ${asset.color}` }}></div>
                  <span className="font-cyber font-bold text-lg tracking-wide">{asset.pair}</span>
                </div>
                <span className="font-mono text-sm text-gray-300 tracking-wider">${asset.price}</span>
              </div>

              {/* Fake Chart Placeholder (SVG) */}
              <div className="h-32 w-full bg-black/40 rounded mb-4 relative overflow-hidden flex items-center justify-center border border-gray-800">
                <svg viewBox="0 0 300 100" className="w-full h-full absolute inset-0 opacity-50">
                  <path 
                    d="M0,80 Q30,90 60,50 T120,50 T180,70 T240,30 T300,50" 
                    fill="none" 
                    stroke={asset.color} 
                    strokeWidth="2" 
                    className="chart-line" 
                  />
                </svg>
                <span className="text-xs text-gray-600 z-10 relative font-mono">Waiting for API Stream...</span>
              </div>

              {/* Betting UI */}
              <div className="mt-auto space-y-3">
                <div className="relative">
                  <span className="absolute left-3 top-2 text-gray-500">$</span>
                  <input 
                    type="number" 
                    placeholder="0.00" 
                    className="w-full bg-gray-900/80 border border-gray-700 rounded py-2 pl-6 pr-3 text-white focus:outline-none focus:border-cyan-500 text-sm font-mono placeholder-gray-600 transition"
                  />
                </div>
                <div className="grid grid-cols-2 gap-3">
                  <button className="bg-green-500/10 border border-green-500/50 hover:bg-green-500 hover:text-black text-green-400 font-bold py-2 px-4 rounded text-sm transition duration-200 flex justify-center items-center">
                    HIGH
                  </button>
                  <button className="bg-red-500/10 border border-red-500/50 hover:bg-red-500 hover:text-black text-red-400 font-bold py-2 px-4 rounded text-sm transition duration-200 flex justify-center items-center">
                    LOW
                  </button>
                </div>
              </div>

            </div>
          ))}
        </div>

      </main>
    </div>
  );
};