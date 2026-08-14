# obs extract

## No domain coverage
Frontend-only CSS/layout positioning fix for the Findings dropdown popover (zero backend, zero new telemetry, no obs instrumentation in the touched `pulse-app/ui/src/widget/**` files) — no span/metric/log/error/PII/harness/SLO surface applies; P5's `ui.layout.transition` is a backend layout-MODE span (per obs-plan.md §4), not this frontend popover.
