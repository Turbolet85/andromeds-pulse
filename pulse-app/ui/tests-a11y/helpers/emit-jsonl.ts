import { appendFile, mkdir } from "node:fs/promises";
import { homedir, platform } from "node:os";
import { dirname, join } from "node:path";

export interface AxeViolationShape {
  id: string;
  impact?: string | null;
  description?: string;
  help?: string;
  helpUrl?: string;
  nodes?: Array<{
    html?: string;
    target?: ReadonlyArray<string | ReadonlyArray<string>>;
    failureSummary?: string;
  }>;
  tags?: string[];
}

export interface EmitOptions {
  surface: string;
  violations: AxeViolationShape[];
  tool?: string;
}

function resolveLogDir(): string {
  const override = process.env.ANDROMEDA_PULSE_DATA_DIR;
  if (override) return join(override, "logs");
  if (platform() === "win32") {
    const appdata = process.env.APPDATA;
    if (appdata) return join(appdata, "andromeda-pulse", "logs");
  } else if (platform() === "darwin") {
    return join(
      homedir(),
      "Library",
      "Application Support",
      "com.andromeda.pulse",
      "logs",
    );
  } else {
    const xdg = process.env.XDG_CONFIG_HOME;
    if (xdg) return join(xdg, "andromeda-pulse", "logs");
  }
  return join(homedir(), ".andromeda-pulse", "logs");
}

function severityForImpact(impact: string | null | undefined): string {
  if (impact === "critical") return "critical";
  if (impact === "serious") return "serious";
  if (impact === "moderate") return "moderate";
  return "minor";
}

function wcagCriterionForTags(tags: string[]): string {
  for (const tag of tags) {
    const match = /^wcag(\d)(\d+)(\d*)$/.exec(tag);
    if (match) {
      const [, principle, guideline, criterion] = match;
      if (criterion) return `SC ${principle}.${guideline}.${criterion}`;
    }
  }
  return "unknown";
}

function selectorString(target: ReadonlyArray<string | ReadonlyArray<string>> | undefined): string {
  if (!target) return "";
  return target
    .map((segment) => (Array.isArray(segment) ? segment.join(" ") : String(segment)))
    .join(" >> ");
}

export async function emitViolations(opts: EmitOptions): Promise<void> {
  const dir = resolveLogDir();
  await mkdir(dir, { recursive: true });
  const filePath = join(dir, "a11y-axe-core-results.jsonl");
  const tool = opts.tool ?? "axe-core";
  const lines: string[] = [];
  for (const v of opts.violations) {
    for (const node of v.nodes ?? [{}]) {
      const record = {
        timestamp: new Date().toISOString(),
        level: severityForImpact(v.impact) === "critical" || severityForImpact(v.impact) === "serious"
          ? "ERROR"
          : "WARN",
        target: "a11y::axe-core",
        message: v.help ?? v.description ?? v.id,
        fields: {
          service: { name: "com.andromeda.pulse" },
          deployment: { environment: process.env.DEPLOYMENT_ENVIRONMENT ?? "test" },
          wcag_criterion: wcagCriterionForTags(v.tags ?? []),
          violation_type: v.id,
          severity: severityForImpact(v.impact),
          surface: opts.surface,
          selector: selectorString(node.target),
          remediation: v.helpUrl ?? null,
          tool,
          tool_result_id: `${v.id}@${opts.surface}`,
        },
      };
      lines.push(JSON.stringify(record));
    }
  }
  if (lines.length === 0) {
    const record = {
      timestamp: new Date().toISOString(),
      level: "INFO",
      target: "a11y::axe-core",
      message: `${opts.surface}: zero violations`,
      fields: {
        service: { name: "com.andromeda.pulse" },
        deployment: { environment: process.env.DEPLOYMENT_ENVIRONMENT ?? "test" },
        wcag_criterion: "n/a",
        violation_type: "none",
        severity: "info",
        surface: opts.surface,
        selector: "",
        remediation: null,
        tool,
        tool_result_id: `pass@${opts.surface}`,
      },
    };
    lines.push(JSON.stringify(record));
  }
  await ensureParent(filePath);
  await appendFile(filePath, lines.join("\n") + "\n", { encoding: "utf8" });
}

async function ensureParent(path: string): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
}
