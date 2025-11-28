// This file contains constant values used throughout the backend application.
// We use 'pub' so other modules (like the scheduler) can see this.
// We use 'const' because this list never changes while the app is running.
// [&str; 9] means an "Array" of "string slices" with a size of 9.

pub const SUPPORTED_ASSETS: [&str; 9] = [
    // --- Crypto ---
    "SOL/USDT", 
    "BTC/USDT", 
    "ETH/USDT",
    
    // --- Forex ---
    "EUR/USD", 
    "GBP/USD", 
    "USD/JPY",
    
    // --- Commodities ---
    "GOLD", 
    "SILVER", 
    "OIL"
];