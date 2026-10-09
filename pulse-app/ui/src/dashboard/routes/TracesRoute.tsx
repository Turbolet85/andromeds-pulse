// Traces route — primary screen for the full dashboard. Wireframe stack
// per layout-templates.md §Wireframe — Full dashboard (Traces primary
// screen): per-service constellation map (hero region) → sortable trace
// data table (main region). Both consume the existing `traces.*` query
// router (chunk #25) + the existing `streams.subscribe_spans` Channel
// (chunk #23). Chunk #42 adds the Investigate trigger button in the
// header action row + per-row right-click context-menu + inline icon
// button at the end of each row.

import { ConstellationCanvas } from "./traces/ConstellationCanvas";
import { TraceTable } from "./traces/TraceTable";
import { useServiceConstellation } from "../../hooks/use-service-constellation";
import { useTraces } from "./traces/use-traces";
import { InvestigateButton } from "../../components/InvestigateButton";
import { useInvestigation } from "../../hooks/use-investigation";

const QUERY_WINDOW_SECONDS = 60;
const QUERY_LIMIT = 100;

export function TracesRoute() {
  const { rows, isLoading } = useTraces({
    timeWindowSeconds: QUERY_WINDOW_SECONDS,
    limit: QUERY_LIMIT,
  });
  const items = useServiceConstellation();
  const { openInvestigation } = useInvestigation();
  return (
    <section
      id="tabpanel-traces"
      aria-labelledby="route-heading-traces"
      data-testid="route-traces"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
        // Fill exactly the height the shell allots main (titlebar 32 + tabnav 40
        // + footer var(--spacing-lg)) so the route never grows the page; the
        // trace table flex-fills the remainder and scrolls internally.
        height: "calc(100vh - 32px - var(--spacing-lg) - 40px)",
        overflow: "hidden",
        boxSizing: "border-box",
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          gap: "var(--spacing-md)",
          flexShrink: 0,
        }}
        data-testid="traces-header"
      >
        <h1
          id="route-heading-traces"
          style={{
            fontFamily: "var(--font-display)",
            fontSize: "20px",
            fontWeight: 600,
            color: "var(--color-text-primary)",
            margin: 0,
          }}
        >
          Traces
        </h1>
        <InvestigateButton
          variant="main"
          onClick={(event) => openInvestigation(event.currentTarget)}
          data-testid="traces-investigate"
        />
      </div>
      <div style={{ flexShrink: 0 }}>
        <ConstellationCanvas items={items} />
      </div>
      <TraceTable rows={rows} isLoading={isLoading} />
    </section>
  );
}
