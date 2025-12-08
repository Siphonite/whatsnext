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
  
  // Actions
  fetchPositions: (wallet: string) => Promise<void>;
  fetchPnl: (wallet: string) => Promise<void>;
  reset: () => void;
}

export const usePositionsStore = create<PositionsState>((set, get) => ({
  openPositions: [],
  settledPositions: [],
  pnlSummary: null,
  loading: false,
  error: null,

  fetchPositions: async (wallet) => {
    set({ loading: true, error: null });
    
    try {
      // Primary: Fetch from backend API
      const response = await fetch(`/api/positions/${wallet}`);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch positions: ${response.statusText}`);
      }
      
      const data = await response.json();
      
      if (data.error) {
        throw new Error(data.error);
      }
      
      // Transform backend response to BetPosition format
      const transformPosition = (pos: any): BetPosition => {
        return {
          marketId: pos.marketId || pos.market_id,
          side: pos.side === "GREEN" || pos.side === "Green" ? "GREEN" : "RED",
          amount: typeof pos.amount === "number" ? pos.amount : parseFloat(pos.amount || "0"),
          weight: typeof pos.weight === "number" ? pos.weight : parseFloat(pos.weight || "0"),
          effectiveStake: typeof pos.effectiveStake === "number" 
            ? pos.effectiveStake 
            : parseFloat(pos.effectiveStake || pos.effective_stake || "0"),
          payout: pos.payout ? (typeof pos.payout === "number" ? pos.payout : parseFloat(pos.payout)) : undefined,
          timestamp: pos.timestamp || (pos.created_at ? new Date(pos.created_at).getTime() / 1000 : Date.now() / 1000),
          status: pos.status || (pos.settled === false ? "OPEN" : "SETTLED"),
        };
      };
      
      const open = (data.open || []).map(transformPosition);
      const settled = (data.settled || []).map(transformPosition);
      
      set({
        openPositions: open,
        settledPositions: settled,
        loading: false,
        error: null,
      });
    } catch (error: any) {
      console.error("Failed to fetch positions:", error);
      set({
        loading: false,
        error: error.message || "Failed to fetch positions",
      });
    }
  },

  fetchPnl: async (wallet: string) => {
    try {
      const response = await fetch(`/api/pnl/${wallet}`);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch PnL: ${response.statusText}`);
      }
      
      const data = await response.json();
      
      if (data.error) {
        throw new Error(data.error);
      }
      
      set({
        pnlSummary: {
          totalPnl: typeof data.totalPnl === "number" ? data.totalPnl : parseFloat(data.totalPnl || "0"),
          winRate: typeof data.winRate === "number" ? data.winRate : parseFloat(data.winRate || "0"),
          streak: typeof data.streak === "number" ? data.streak : parseInt(data.streak || "0", 10),
        },
      });
    } catch (error: any) {
      console.error("Failed to fetch PnL:", error);
      set({
        pnlSummary: {
          totalPnl: 0,
          winRate: 0,
          streak: 0,
        },
      });
    }
  },

  reset: () => {
    set({
      openPositions: [],
      settledPositions: [],
      pnlSummary: null,
      loading: false,
      error: null,
    });
  },
}));

