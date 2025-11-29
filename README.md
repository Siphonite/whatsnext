# What’s Next? — Dynamic Candle Prediction Markets (Solana)

**What’s Next?** is a Solana-based candle prediction market where users bet on whether the next **4-hour** candlestick of major assets (crypto, forex, commodities) will close **Green** or **Red**. Early bets earn higher multipliers via a Dynamic Edge Mechanism that prevents last-minute sniping. :contentReference[oaicite:1]{index=1}

---

## Key Features
- Predict the next 4H candle: Green (↑) or Red (↓)
- Dynamic Edge Mechanism (time-based reward multipliers)
- Weighted parimutuel pools + virtual liquidity bootstrap
- Fully on-chain settlement (Anchor / Solana)
- Real-time charts (TradingView) and wallet connect

---

## Supported Assets (MVP)
- Crypto: SOL/USDT, BTC/USDT, ETH/USDT  
- Forex: EUR/USD, GBP/USD, USD/JPY  
- Commodities: XAU/USD (Gold), XAG/USD (Silver), Brent Crude

---

## Quickstart (Dev)
1. Clone repo  
   `git clone https://github.com/<you>/whatsnext.git && cd whatsnext`
2. Frontend  
   `cd frontend && npm install && npm run dev`
3. Backend (Node)  
   `cd backend && npm install && npm run dev`
4. Anchor program (devnet)  
   `cd program && anchor build && anchor deploy --provider.cluster devnet`

See `frontend/`, `backend/`, and `program/` for detailed setup.

---

## Project Structure
