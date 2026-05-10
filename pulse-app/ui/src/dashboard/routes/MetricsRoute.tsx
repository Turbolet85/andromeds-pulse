import { MetricsChart } from "./metrics/MetricsChart";
import { useMetrics } from "./metrics/use-metrics";

const QUERY_WINDOW_SECONDS = 60;
const QUERY_LIMIT = 100;

export function MetricsRoute() {
  const { rows } = useMetrics({
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
      <MetricsChart rows={rows} windowSeconds={QUERY_WINDOW_SECONDS} />
    </section>
  );
}
