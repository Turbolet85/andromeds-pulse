#!/usr/bin/env bash
# xtask/ci/l4-latency-p99.sh — obs-plan §10 SLO gate (L4 inference latency)
#
# Parses metric.pipeline.l4.inference_latency_p99_milliseconds records from
# the JSON log, groups by hardware_profile label, computes p99 per profile by
# nearest rank (the ceil(0.99*n)-th smallest, obs-plan §10's one rule), and
# asserts each is at or below the dist-arch v3 §L4 hardware profile matrix
# budget:
#
#   gpu-primary:  10000ms   (Tier-1 SLO)
#   gpu-fallback:  3000ms   (Tier-1 SLO)
#   cpu-primary:  30000ms   (Tier-1 SLO; degraded mode)
#   cpu-fallback: 15000ms   (Tier-1 SLO)
#
# Per dist-arch v3 §L4 Hardware Profile Matrix: cpu-primary in degraded
# mode is intentionally lenient (10-30s primary inference latency); gate
# trips only on extreme regression beyond the documented degraded mode
# expectation.
#
# A target record without a hardware_profile or a numeric duration_ms cannot
# be graded, so any such record fails the run instead of being dropped.
#
# INACTIVE state (chunks #82-#83 substrate): when no L4 inference latency
# records are present (mistralrs runtime not yet bound; LlmInferenceRunner
# stub returns ModelNotConfigured), exits 0. Activates organically when
# the first real mistralrs binding chunk lands (chunk #83 follow-up OR
# chunk #84+ fallback tier).
#
# Usage: l4-latency-p99.sh [<log-path>]
#   Default log-path: $ANDROMEDA_PULSE_DATA_DIR/logs/agent-latest.jsonl* glob
#                     (tracing-appender daily-rolling appends YYYY-MM-DD suffix)

set -euo pipefail

LOG_DIR="${ANDROMEDA_PULSE_DATA_DIR:-}/logs"
LOG_ARG="${1:-}"
TARGET_METRIC="metric.pipeline.l4.inference_latency_p99_milliseconds"

# Per-profile budgets (override individual values via env vars).
GPU_PRIMARY_BUDGET_MS="${L4_GPU_PRIMARY_BUDGET_MS:-10000}"
GPU_FALLBACK_BUDGET_MS="${L4_GPU_FALLBACK_BUDGET_MS:-3000}"
CPU_PRIMARY_BUDGET_MS="${L4_CPU_PRIMARY_BUDGET_MS:-30000}"
CPU_FALLBACK_BUDGET_MS="${L4_CPU_FALLBACK_BUDGET_MS:-15000}"

if [ -n "$LOG_ARG" ] && [ -f "$LOG_ARG" ]; then
  LOG_FILES=("$LOG_ARG")
elif [ -d "$LOG_DIR" ]; then
  mapfile -t LOG_FILES < <(find "$LOG_DIR" -maxdepth 1 -type f -name 'agent-latest.jsonl*' 2>/dev/null | sort)
else
  LOG_FILES=()
fi

if [ "${#LOG_FILES[@]}" -eq 0 ]; then
  echo "l4-latency-p99: no log file present (INACTIVE state — pre-mistralrs-binding chunk; gate trivially passes)"
  exit 0
fi

# Extract (profile, duration_ms) tuples for the target metric.
samples_per_profile() {
  for f in "${LOG_FILES[@]}"; do
    while IFS= read -r line; do
      [ -z "$line" ] && continue
      target=$(printf '%s' "$line" | sed -nE 's/.*"target":"([^"]+)".*/\1/p')
      if [ "$target" != "$TARGET_METRIC" ]; then
        continue
      fi
      profile=$(printf '%s' "$line" | sed -nE 's/.*"hardware_profile":"([^"]+)".*/\1/p')
      ms=$(printf '%s' "$line" | sed -nE 's/.*"duration_ms":([0-9]+).*/\1/p')
      if [ -n "$profile" ] && [ -n "$ms" ]; then
        printf '%s\t%s\n' "$profile" "$ms"
      else
        printf 'UNLABELED\n'
      fi
    done < "$f"
  done
}

mapfile -t EXTRACTED < <(samples_per_profile)

SAMPLE_LINES=()
unlabeled=0
for entry in "${EXTRACTED[@]}"; do
  if [ "$entry" = "UNLABELED" ]; then
    unlabeled=$((unlabeled + 1))
  else
    SAMPLE_LINES+=("$entry")
  fi
done

if [ "$unlabeled" -gt 0 ]; then
  echo "::error::l4-latency-p99: $unlabeled sample(s) carry no hardware_profile or duration_ms" >&2
  exit 1
fi

if [ "${#SAMPLE_LINES[@]}" -eq 0 ]; then
  echo "l4-latency-p99: no $TARGET_METRIC records found (INACTIVE state — pre-mistralrs-binding chunk; gate trivially passes)"
  exit 0
fi

# Compute p99 per profile via awk: sort, then the nearest-rank index
# ceil(0.99 * n), in integer arithmetic.
p99_per_profile=$(printf '%s\n' "${SAMPLE_LINES[@]}" | awk '
  {
    profile = $1
    ms = $2 + 0
    samples[profile] = samples[profile] " " ms
    counts[profile]++
  }
  END {
    for (p in samples) {
      n = counts[p]
      split(samples[p], arr, " ")
      # arr[1] is the empty leading split; values start at arr[2].
      # Sort the array.
      asort(arr, sorted)
      # Drop the leading empty element.
      shifted_n = 0
      for (i = 1; i <= length(sorted); i++) {
        if (sorted[i] != "") {
          shifted[++shifted_n] = sorted[i]
        }
      }
      idx = int((shifted_n * 99 + 99) / 100)
      if (idx < 1) idx = 1
      if (idx > shifted_n) idx = shifted_n
      p99 = shifted[idx]
      printf "%s\t%d\t%d\n", p, p99, shifted_n
    }
  }
')

exit_code=0
while IFS=$'\t' read -r profile p99 sample_count; do
  [ -z "$profile" ] && continue
  case "$profile" in
    gpu_primary)  budget="$GPU_PRIMARY_BUDGET_MS" ;;
    gpu_fallback) budget="$GPU_FALLBACK_BUDGET_MS" ;;
    cpu_primary)  budget="$CPU_PRIMARY_BUDGET_MS" ;;
    cpu_fallback) budget="$CPU_FALLBACK_BUDGET_MS" ;;
    *)
      echo "l4-latency-p99: unknown profile label '$profile' (sample_count=$sample_count) — skipping"
      continue
      ;;
  esac
  if [ "$p99" -gt "$budget" ]; then
    echo "::error::l4-latency-p99: profile=$profile p99=${p99}ms exceeds budget ${budget}ms (sample_count=$sample_count)" >&2
    exit_code=1
  else
    echo "l4-latency-p99: profile=$profile p99=${p99}ms ≤ ${budget}ms (sample_count=$sample_count) PASS"
  fi
done <<< "$p99_per_profile"

exit $exit_code
