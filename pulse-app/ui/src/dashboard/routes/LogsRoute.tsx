import { LogFilter } from "./logs/LogFilter";
import { LogTable } from "./logs/LogTable";
import { useLogFilter } from "./logs/use-log-filter";
import { useLogs } from "./logs/use-logs";

const QUERY_WINDOW_SECONDS = 60;
const QUERY_LIMIT = 100;

export function LogsRoute() {
  const { rows, isLoading } = useLogs({
    timeWindowSeconds: QUERY_WINDOW_SECONDS,
    limit: QUERY_LIMIT,
  });
  const filter = useLogFilter(rows);
  return (
    <section
      id="tabpanel-logs"
      aria-labelledby="route-heading-logs"
      data-testid="route-logs"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
      }}
    >
      <h1
        id="route-heading-logs"
        style={{
          fontFamily: "var(--font-display)",
          fontSize: "20px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        Logs
      </h1>
      <LogFilter
        state={filter.state}
        onSearchChange={filter.setSearchQuery}
        onToggleTier={filter.toggleTier}
      />
      <LogTable rows={filter.filteredRows} isLoading={isLoading} />
    </section>
  );
}
