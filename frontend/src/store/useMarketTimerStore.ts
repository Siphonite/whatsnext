import { create } from "zustand";
import { persist } from "zustand/middleware";
import { get4HWindow } from "../utils/get4HWindow";

interface MarketTimerState {
  startTime: number;      // epoch from backend (milliseconds)
  endTime: number;        // epoch from backend (milliseconds)
  serverTime: number;     // current backend time (milliseconds)
  timeLeft: number;       // derived: endTime - serverTime (milliseconds)
  lastSynced: number;     // local timestamp of last server sync (milliseconds)
  
  setMarketTimes: (data: {
    startTime?: number;
    endTime?: number;
    serverTime?: number;
  }) => void;
  
  tick: () => void;       // update timeLeft every second
}

export const useMarketTimerStore = create<MarketTimerState>()(
  persist(
    (set, get) => ({
      startTime: 0,
      endTime: 0,
      serverTime: 0,
      timeLeft: 0,
      lastSynced: 0,

      setMarketTimes: (data) => {
        const state = get();
        const now = Date.now();
        
        // If backend provides timestamps, use them
        let newStartTime = data.startTime ?? state.startTime;
        let newEndTime = data.endTime ?? state.endTime;
        let newServerTime = data.serverTime ?? now;
        
        // If timestamps are missing, infer from TradingView 4H window
        if (!newStartTime || !newEndTime) {
          const window = get4HWindow(now);
          newStartTime = window.start;
          newEndTime = window.end;
        }
        
        // Auto-correct if backend time mismatch > 10s
        const timeDiff = Math.abs(newServerTime - now);
        if (timeDiff > 10000) {
          console.warn(`Server time drift detected: ${timeDiff}ms, using local time`);
          newServerTime = now;
        }
        
        // Verify we're in the correct 4H window
        const expectedWindow = get4HWindow(now);
        const backendWindow = get4HWindow(newEndTime);
        
        // If backend window doesn't match current window, auto-correct
        if (Math.abs(expectedWindow.start - backendWindow.start) > 1000) {
          console.warn("Backend window mismatch, using TradingView 4H window");
          newStartTime = expectedWindow.start;
          newEndTime = expectedWindow.end;
        }
        
        // Calculate initial timeLeft
        const initialTimeLeft = Math.max(0, newEndTime - newServerTime);
        
        set({
          startTime: newStartTime,
          endTime: newEndTime,
          serverTime: newServerTime,
          timeLeft: initialTimeLeft,
          lastSynced: now,
        });
      },

      tick: () => {
        const state = get();
        
        // If no sync has happened, try to hydrate from stored state
        if (state.lastSynced === 0) {
          if (state.endTime > 0) {
            // We have persisted data, recalculate
            const now = Date.now();
            const window = get4HWindow(now);
            
            // Check if we're still in the same window
            const storedWindow = get4HWindow(state.endTime);
            if (window.start !== storedWindow.start) {
              // New window, update
              set({
                startTime: window.start,
                endTime: window.end,
                serverTime: now,
                timeLeft: Math.max(0, window.end - now),
                lastSynced: now,
              });
            } else {
              // Same window, recalculate based on elapsed time
              const elapsed = state.lastSynced > 0 ? now - state.lastSynced : 0;
              const corrected = state.serverTime + elapsed;
              const timeLeft = Math.max(0, state.endTime - corrected);
              set({
                timeLeft,
                serverTime: corrected,
                lastSynced: now,
              });
            }
          } else {
            // No data, use current window
            const now = Date.now();
            const window = get4HWindow(now);
            set({
              startTime: window.start,
              endTime: window.end,
              serverTime: now,
              timeLeft: Math.max(0, window.end - now),
              lastSynced: now,
            });
          }
          return;
        }
        
        // Time drift correction: use elapsed time since last sync
        const now = Date.now();
        const elapsed = now - state.lastSynced;
        const corrected = state.serverTime + elapsed;
        const timeLeft = Math.max(0, state.endTime - corrected);
        
        set({
          timeLeft,
          serverTime: corrected,
          lastSynced: now,  // Update lastSynced to prevent accumulation
        });
      },
    }),
    {
      name: "market-timer-store",
      onRehydrateStorage: () => (state) => {
        // After rehydration, immediately recalculate timeLeft
        if (state) {
          const now = Date.now();
          if (state.endTime > 0) {
            const window = get4HWindow(now);
            const storedWindow = get4HWindow(state.endTime);
            
            if (window.start !== storedWindow.start) {
              // New window - update to current window
              state.setMarketTimes({
                startTime: window.start,
                endTime: window.end,
                serverTime: now,
              });
            } else {
              // Same window - recalculate timeLeft based on elapsed time
              const elapsed = state.lastSynced > 0 ? now - state.lastSynced : 0;
              const corrected = state.serverTime + elapsed;
              const timeLeft = Math.max(0, state.endTime - corrected);
              
              // Update state directly (bypassing set to avoid re-triggering persist)
              Object.assign(state, {
                timeLeft,
                serverTime: corrected,
                lastSynced: now,
              });
            }
          } else {
            // No persisted data - initialize with current window
            const window = get4HWindow(now);
            state.setMarketTimes({
              startTime: window.start,
              endTime: window.end,
              serverTime: now,
            });
          }
        }
      },
    }
  )
);

