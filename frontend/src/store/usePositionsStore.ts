import { create } from "zustand";

export interface BetPosition {
  marketId: number;
  side: "GREEN" | "RED";
  amount: number;
  weight: number;
  effectiveStake: number;
  payout?: number;
  timestamp: number;
  status: "OPEN" | "SETTLED";
}

interface PnlSummary {
  totalPnl: number;
  winRate: number;
  streak: number;
}

interface PositionsState {
  openPositions: BetPosition[];
  settledPositions: BetPosition[];
  pnlSummary: PnlSummary | null;

  loading: boolean;
  error: string | null;

  // NEW CLAIM STATE
  claimable: Record<number, number>;     // marketId → lamports
  claimLoading: boolean;
  claimError: string | null;

  // Actions
  fetchPositions: (wallet: string) => Promise<void>;
  fetchPnl: (wallet: string) => Promise<void>;

  fetchClaimable: (wallet: string, marketId: number) => Promise<number>;
  claimReward: (
    wallet: string,
    marketId: number,
    sendTx: () => Promise<string>
  ) => Promise<void>;

  reset: () => void;
}

export const usePositionsStore = create<PositionsState>((set, get) => ({
  openPositions: [],
  settledPositions: [],
  pnlSummary: null,
  loading: false,
  error: null,

  claimable: {},
  claimLoading: false,
  claimError: null,

  // ---------------------------------------------
  // FETCH POSITIONS
  // ---------------------------------------------
  fetchPositions: async (wallet) => {
    set({ loading: true, error: null });

    try {
      const response = await fetch(`/api/positions/${wallet}`);
      const data = await response.json();

      if (data.error) throw new Error(data.error);

      const transform = (pos: any): BetPosition => ({
        marketId: pos.marketId || pos.market_id,
        side: pos.side === "GREEN" ? "GREEN" : "RED",
        amount: Number(pos.amount),
        weight: Number(pos.weight),
        effectiveStake: Number(
          pos.effectiveStake ?? pos.effective_stake ?? 0
        ),
        payout: pos.payout ? Number(pos.payout) : undefined,
        timestamp:
          pos.timestamp ||
          (pos.created_at
            ? new Date(pos.created_at).getTime() / 1000
            : Date.now() / 1000),
        status: pos.status || (pos.settled ? "SETTLED" : "OPEN"),
      });

      set({
        openPositions: (data.open || []).map(transform),
        settledPositions: (data.settled || []).map(transform),
        loading: false,
      });
    } catch (e: any) {
      console.error(e);
      set({ loading: false, error: e.message });
    }
  },

  // ---------------------------------------------
  // FETCH PNL SUMMARY
  // ---------------------------------------------
  fetchPnl: async (wallet: string) => {
    try {
      const response = await fetch(`/api/pnl/${wallet}`);
      const data = await response.json();

      if (data.error) throw new Error(data.error);

      set({
        pnlSummary: {
          totalPnl: Number(data.totalPnl ?? 0),
          winRate: Number(data.winRate ?? 0),
          streak: Number(data.streak ?? 0),
        },
      });
    } catch (e) {
      console.error("Failed to fetch PnL");
      set({
        pnlSummary: { totalPnl: 0, winRate: 0, streak: 0 },
      });
    }
  },

  // ---------------------------------------------
  // FETCH CLAIMABLE AMOUNT
  // ---------------------------------------------
  fetchClaimable: async (wallet, marketId) => {
    try {
      const res = await fetch(`/api/claimable/${marketId}/${wallet}`);
      const data = await res.json();

      if (!data.ok) throw new Error(data.error || "Claimable fetch failed");

      const payout = Number(data.payout || 0);

      set((s) => ({
        claimable: { ...s.claimable, [marketId]: payout },
      }));

      return payout;
    } catch (e: any) {
      console.error("Claimable fetch error:", e);
      return 0;
    }
  },

  // ---------------------------------------------
  // CLAIM REWARD
  // ---------------------------------------------
  claimReward: async (wallet, marketId, sendTx) => {
    set({ claimLoading: true, claimError: null });

    try {
      // A) Send on-chain transaction
      const txSig = await sendTx();

      // B) Notify backend
      await fetch(`/api/claim/record`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          wallet,
          market_id: marketId,
          tx_sig: txSig,
        }),
      });

      // C) Refresh local UI
      await get().fetchPositions(wallet);
      await get().fetchPnl(wallet);

      // Mark claimable removed
      set((s) => ({
        claimable: { ...s.claimable, [marketId]: 0 },
        claimLoading: false,
      }));
    } catch (e: any) {
      console.error("Claim reward failed:", e);
      set({
        claimError: e.message || "Claim failed",
        claimLoading: false,
      });
    }
  },

  reset: () =>
    set({
      openPositions: [],
      settledPositions: [],
      pnlSummary: null,
      loading: false,
      error: null,
      claimable: {},
      claimLoading: false,
      claimError: null,
    }),
}));
