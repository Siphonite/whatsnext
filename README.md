What’s Next? — Dynamic Candle Prediction Markets on Solana

What’s Next? is a Solana-based prediction market where users bet on whether the next 4-hour candlestick of an asset will close Green (↑) or Red (↓).

Unlike traditional betting or AMM-based prediction markets, What’s Next? introduces a time-weighted, parimutuel design that rewards early conviction, discourages last-minute sniping, and settles markets fully on-chain.

Core Idea

For every 4-hour candle:

A new market is created automatically

Users place bets on Green or Red

Earlier bets receive higher weight

After the candle closes, the market is settled on-chain

Winners claim rewards directly from the protocol treasury

Markets are deterministic, continuous, and autonomous, making the system resilient to restarts and backend failures.

->Key Features
->Candle-Based Prediction Markets

Predict the direction of the next 4H candle

One market per candle, no manual intervention

Dynamic Edge Mechanism (Anti-Sniping)

Bets placed earlier receive higher effective stake

Prevents last-minute “free-ride” behavior

Encourages genuine market conviction

Weighted Parimutuel Pools

No fixed odds or house bias

Winners split the losing side proportionally

Includes virtual liquidity to reduce early volatility

Fully On-Chain Settlement (Solana + Anchor)

Market creation & settlement handled by a Solana program

PDA-based markets, bets, and treasury

Trust-minimized outcome resolution

Transparent Claims

Users claim winnings on-chain

Backend records payouts for analytics only

No custodial user funds

->Why This Is Different
Traditional Prediction Markets

Static odds or AMM pricing

Vulnerable to sniping

Heavy off-chain logic

Difficult to scale across time-based events

- What’s Next?

Time-aware markets (native to candles)

Weight-based fairness, not price manipulation

Deterministic market IDs (timestamp-derived)

Solana-native automation via PDAs + cron scheduler

Designed for high-frequency, repeating markets

This makes What’s Next? a strong example of a new financial primitive enabled by Solana’s speed and low fees.

Unique Solana Use Case

What’s Next? showcases how Solana can be used for:

Repeating, high-frequency financial markets

Deterministic PDA derivation for time-based assets

On-chain resolution without oracle price feeds per trade

Scalable prediction markets without AMMs

This model would be impractical on slower or more expensive chains.

Supported Assets (MVP)

BTCUSDT (Binance 4H candles)

Architecture supports easy extension to:

ETH, SOL

Forex pairs

Commodities

Indexes

Architecture Overview
┌────────────┐     ┌────────────┐     ┌──────────────┐
│  Frontend  │────▶│  Backend   │────▶│ Solana Program│
│  (React)   │     │ (Rust/Axum)│     │  (Anchor)    │
└────────────┘     └────────────┘     └──────────────┘
       ▲                   │
       └──── Wallet + RPC ─┘

Components

Frontend: React + Solana Wallet Adapter

Backend (Rust):

Market scheduler (4H creation)

Settlement automation

Oracle integration

Read-only APIs

Solana Program:

Market PDA

Bet PDA

Treasury PDA

On-chain settlement & claims

Quickstart (Development)
1️⃣ Clone Repository
git clone https://github.com/<Siphonite>/whatsnext.git
cd whatsnext

2️⃣ Anchor Program (Devnet)
cd program
anchor build
anchor deploy --provider.cluster devnet

3️⃣ Backend (Rust / Axum)
cd backend-rs
cargo run

4️⃣ Frontend
cd frontend
npm install
npm run dev

MVP Notes & Known Limitations

Oracle relies on Binance 4H candles

Backend scheduler assumes continuous uptime

UI focuses on core flows (bet, view, claim)

Advanced analytics & multi-asset expansion planned

These tradeoffs were intentional to focus on core protocol correctness.

Future Roadmap

Multi-asset expansion

Mobile-optimized UI

Permissionless market creation

DAO-governed parameters

Advanced analytics & leaderboard

Cross-chain oracle sources

Final Note

What’s Next? is not just a betting app — it’s a repeatable, time-aware financial primitive built natively for Solana.

It demonstrates how blockchain can power continuous prediction markets with fairness, transparency, and automation at scale.