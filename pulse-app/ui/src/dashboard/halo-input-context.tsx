// React context bridge for the synthetic HaloInput stream that App.tsx
// generates via use-synthetic-halo-input. Lets TanStack Router route
// components (TracesRoute) consume the input without TanStack Router's
// per-call context machinery — keeps coupling to TanStack Router minimal so
// future consumers (e.g. compact-widget reuse, tray icon panel) can adopt
// the same context.

import { createContext, useContext, type ReactNode } from "react";
import type { HaloInput } from "../halo/halo-types";

const HaloInputContext = createContext<HaloInput | null>(null);

interface HaloInputProviderProps {
  value: HaloInput;
  children: ReactNode;
}

export function HaloInputProvider({ value, children }: HaloInputProviderProps) {
  return (
    <HaloInputContext.Provider value={value}>{children}</HaloInputContext.Provider>
  );
}

export function useHaloInput(): HaloInput {
  const value = useContext(HaloInputContext);
  if (value === null) {
    return { throughputHz: 0, errorRate: 0 };
  }
  return value;
}
