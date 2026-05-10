import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useRef,
  useState,
  type MutableRefObject,
  type ReactNode,
} from "react";

interface InvestigationContextValue {
  open: boolean;
  triggerRef: MutableRefObject<HTMLElement | null>;
  openInvestigation: (trigger: HTMLElement | null) => void;
  closeInvestigation: () => void;
}

const InvestigationContext = createContext<InvestigationContextValue | null>(null);

export function useInvestigation(): InvestigationContextValue {
  const value = useContext(InvestigationContext);
  if (value === null) {
    throw new Error(
      "useInvestigation called outside InvestigationProvider; wrap your tree in <InvestigationProvider>",
    );
  }
  return value;
}

export function InvestigationProvider({ children }: { children: ReactNode }) {
  const [open, setOpen] = useState(false);
  const triggerRef = useRef<HTMLElement | null>(null);

  const openInvestigation = useCallback((trigger: HTMLElement | null) => {
    triggerRef.current = trigger;
    setOpen(true);
  }, []);

  const closeInvestigation = useCallback(() => {
    setOpen(false);
  }, []);

  const value = useMemo<InvestigationContextValue>(
    () => ({
      open,
      triggerRef,
      openInvestigation,
      closeInvestigation,
    }),
    [open, openInvestigation, closeInvestigation],
  );

  return (
    <InvestigationContext.Provider value={value}>
      {children}
    </InvestigationContext.Provider>
  );
}
