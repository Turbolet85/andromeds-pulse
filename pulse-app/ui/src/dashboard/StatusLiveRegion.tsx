// Polite aria-live region for dashboard status announcements (WCAG SC 4.1.3).
// Single mounted region exposed via React Context; route-change side-effects
// call announce() to update the region without stealing focus. Visually hidden
// per design plan §Component Patterns + a11y plan §7 Live regions.
//
// Announcements MUST stay non-blocking: aria-live="polite" + role="status";
// never aria-live="assertive" here (would steal focus).

import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactNode,
} from "react";

interface StatusLiveRegionContextValue {
  announce: (message: string) => void;
}

const StatusLiveRegionContext = createContext<StatusLiveRegionContextValue | null>(null);

export function useStatusAnnouncer(): (message: string) => void {
  const context = useContext(StatusLiveRegionContext);
  if (context === null) {
    return () => {
      // No-op when used outside provider; tests that don't mount the provider
      // shouldn't crash. Production always renders the provider in DashboardShell.
    };
  }
  return context.announce;
}

interface StatusLiveRegionProviderProps {
  children: ReactNode;
}

export function StatusLiveRegionProvider({ children }: StatusLiveRegionProviderProps) {
  const [message, setMessage] = useState<string>("");

  const announce = useCallback((value: string) => {
    setMessage(value);
  }, []);

  const value = useMemo<StatusLiveRegionContextValue>(() => ({ announce }), [announce]);

  return (
    <StatusLiveRegionContext.Provider value={value}>
      {children}
      <div
        role="status"
        aria-live="polite"
        id="shell-status"
        data-testid="status-live-region"
        style={{
          position: "absolute",
          width: 1,
          height: 1,
          margin: -1,
          padding: 0,
          overflow: "hidden",
          clip: "rect(0, 0, 0, 0)",
          whiteSpace: "nowrap",
          border: 0,
        }}
      >
        {message}
      </div>
    </StatusLiveRegionContext.Provider>
  );
}
