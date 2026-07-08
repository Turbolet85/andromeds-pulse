import { EmptyState } from "../../components/EmptyState";
import { MetricsChart } from "./metrics/MetricsChart";
import { useMetrics } from "./metrics/use-metrics";

const QUERY_WINDOW_SECONDS = 60;
const QUERY_LIMIT = 100;

export function MetricsRoute() {
  const { rows, isLoading, error } = useMetrics({
    timeWindowSeconds: QUERY_WINDOW_SECONDS,
    limit: QUERY_LIMIT,
  });
  return (
    <section
      id="tabpanel-metrics"
      aria-labelledby="route-heading-metrics"
      data-testid="route-metrics"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
      }}
    >
      <h1
        id="route-heading-metrics"
        style={{
          fontFamily: "var(--font-display)",
          fontSize: "20px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        Metrics
      </h1>
      {error !== null ? (
        <EmptyState message="Couldn't load metrics" testId="metrics-error-state" />
      ) : !isLoading && rows.length === 0 ? (
        <EmptyState
          message="No metrics received yet"
          hint={
            <>
              Point an OTLP metrics exporter at{" "}
              <code style={{ fontFamily: "var(--font-code)" }}>:4318</code> /{" "}
              <code style={{ fontFamily: "var(--font-code)" }}>:4317</code>
            </>
          }
          testId="metrics-empty-state"
        />
      ) : (
        <MetricsChart rows={rows} windowSeconds={QUERY_WINDOW_SECONDS} />
      )}
    </section>
  );
}
