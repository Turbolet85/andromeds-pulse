// Headful webview driver: drives the assembled product path in the live Tauri
// window and REPORTS what it observed. It asserts nothing.
//
// The boot-quit `cargo xtask self-verify` harness never clicks (it hides/quits
// programmatically), and the Playwright a11y suite drives a static `dist` in
// Chromium — so neither can catch a dead affordance. This script closes that
// gap via the test-plan §6 stack (tauri-driver + WebdriverIO).
//
// Division of labour is the whole point: this side presses and observes, and
// `xtask/src/webview_drive.rs` owns every assertion. A press that "returned
// without throwing" is not evidence — the ACL drops a webview IPC silently,
// which is exactly the dead-affordance mode this harness exists to catch. DOM
// observations therefore leave here as FACTS in a stage-report file; the Rust
// caller decides what they mean.
//
// Invoked by `cargo xtask webview-drive`. Env contract:
//   PULSE_BIN          absolute path to the pulse-app binary under test
//   PULSE_DATA_DIR     per-run data dir (also the obs-log root + report sink)
//   MSEDGEDRIVER_PATH  msedgedriver matching the installed WebView2 runtime
//   PULSE_INJECTOR     absolute path to the prebuilt inject_demo example
//                      (absent ⇒ the injector-suppressed RED arm)
import { spawn } from 'node:child_process'
import { createRequire } from 'node:module'
import { createConnection } from 'node:net'
import { readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

const require = createRequire(import.meta.url)
const { remote, Key } = require('webdriverio')

const PULSE_BIN = process.env.PULSE_BIN
const PULSE_DATA_DIR = process.env.PULSE_DATA_DIR
const MSEDGEDRIVER_PATH = process.env.MSEDGEDRIVER_PATH
const PULSE_INJECTOR = process.env.PULSE_INJECTOR ?? ''

// Traces anchors re-pointed onto the accessible names/roles the surface ships
// (2026-08-23-a11y-verification made them load-bearing): the hero landmark by
// its STABLE literal name, the trace list by its native table element (the
// implicit `table` role), rows by their semantic class. The empty state ships
// no accessible anchor (a plain <td> message cell), so its testid stays —
// test-plan §6 permits data-testid exactly where no accessible name ships.
const TRACE_REGION = 'section[aria-label="Telemetry traces chart"]'
const TRACE_TABLE = 'table'
const TRACE_TABLE_EMPTY = '[data-testid="trace-table-empty"]'
const TRACE_ROW = '.trace-row'
const TRACE_SCROLL = '[data-testid="trace-table-scroll"]'
const CONNECTION_STATUS_LINE = '[data-testid="connection-status-line"]'
const FINDINGS_COUNTER = '[data-testid="findings-counter"]'
const FINDINGS_ROW = '[data-testid="findings-window-row"]'
const MODAL_DIALOG = '[data-testid="modal-dialog"]'
const INVESTIGATE = 'button[aria-label="Investigate"]'
const CLOSE_TO_TRAY = 'button[aria-label="Close to tray"]'
const TOGGLE_DASHBOARD = 'button[aria-label="Toggle dashboard"]'
// The titlebar's flex spacer: inside `data-tauri-drag-region` and clear of every
// control, so a synthesized press here lands on the drag surface rather than on
// a button that would swallow it.
const DRAG_REGION = '.titlebar__grow'
const TAB_NAV = '[data-testid="tab-nav"]'
const PRESET_PROMPT_LIST = '[data-testid="preset-prompt-list"]'
// The titlebar Investigate button OPENS the modal; `investigate.run_action`
// only fires when one of the preset actions inside it is pressed.
const PRESET_ACTION = 'button=Diagnose latency outlier'
const TAB = (id) => `[data-testid="tab-${id}"]`
const EMPTY_STATE = (route) => `[data-testid="${route}-empty-state"]`
const EMPTY_STATE_HINT = (route) => `[data-testid="${route}-empty-state-hint"]`
const ERROR_STATE = (route) => `[data-testid="${route}-error-state"]`

const REPORT_BASENAME = 'webview-drive-stages.json'
const SESSION_TIMEOUT_MS = 60_000
const DRIVER_PORT = 4444
const POLL_INTERVAL_MS = 250
const DRIVER_READY_TIMEOUT_MS = 30_000
// Per-stage budgets. The storm stage is the long one: inject_demo warms up for
// ~5s healthy before payment-service degrades, and the detector needs ~1s of
// error spans past that to cross its threshold.
const MOUNT_GRACE_MS = 20_000
const DASHBOARD_TIMEOUT_MS = 30_000
const POPULATE_TIMEOUT_MS = 30_000
const STORM_TIMEOUT_MS = 60_000
const INVESTIGATE_TIMEOUT_MS = 30_000
// Absence is a legitimate outcome on the RED arm, so a stage that observes
// nothing necessarily spends its whole budget; keep these inside the xtask-side
// timeout rather than letting one stage eat it.
const ABSENT_TIMEOUT_MS = 15_000
// Probe displacements: large enough that a real OS move/resize is unambiguous
// against any incidental jitter, small enough to stay on-screen.
const PROBE_DRAG_DX = 120
const PROBE_DRAG_DY = 90
const PROBE_RESIZE_DW = 60
const PROBE_RESIZE_DH = 40

for (const [name, value] of Object.entries({ PULSE_BIN, PULSE_DATA_DIR, MSEDGEDRIVER_PATH })) {
  if (!value) {
    console.error(`webview-drive: missing required env ${name}`)
    process.exit(2)
  }
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms))
const stages = []
let tauriDriver
let injector

function record(stage, observed, detail = {}) {
  stages.push({ stage, observed, ...detail })
  console.log(`webview-drive: [${stage}] observed=${observed} ${JSON.stringify(detail)}`)
}

// Written even when a stage throws — a partial report is what lets the Rust
// side say WHICH stage went dark instead of just "the leg failed".
function writeReport() {
  try {
    writeFileSync(join(PULSE_DATA_DIR, REPORT_BASENAME), JSON.stringify({ stages }, null, 2))
  } catch (err) {
    console.error(`webview-drive: could not write stage report: ${err?.message ?? err}`)
  }
}

// Poll a condition to a deadline instead of sleeping a guessed interval —
// test-plan §11 E2E bans sleep(N) synchronization, and a fixed wait is also a
// race: tearing the session down before an action's IPC lands would look
// identical to a dead affordance.
async function pollUntil(label, deadlineMs, probe) {
  const deadline = Date.now() + deadlineMs
  while (Date.now() < deadline) {
    let hit
    try {
      hit = await probe()
    } catch {
      hit = false
    }
    if (hit) return true
    await sleep(POLL_INTERVAL_MS)
  }
  console.log(`webview-drive: ${label} not observed within ${deadlineMs}ms`)
  return false
}

function portAccepts(port) {
  return new Promise((resolve) => {
    const socket = createConnection({ host: '127.0.0.1', port })
    const done = (ok) => {
      socket.destroy()
      resolve(ok)
    }
    socket.once('connect', () => done(true))
    socket.once('error', () => done(false))
  })
}

// Synchronization only. The xtask caller re-reads the same family afterwards
// and owns the verdict; reading here just decides when a stage may advance.
function logRecords(dataDir) {
  const logDir = join(dataDir, 'logs')
  let names
  try {
    names = readdirSync(logDir)
  } catch {
    return []
  }
  const records = []
  for (const n of names.filter((n) => n.startsWith('agent-latest.jsonl'))) {
    let body
    try {
      body = readFileSync(join(logDir, n), 'utf8')
    } catch {
      continue
    }
    for (const line of body.split('\n').filter(Boolean)) {
      try {
        records.push(JSON.parse(line))
      } catch {
        /* partial line mid-write */
      }
    }
  }
  return records
}

function logMentions(dataDir, predicate) {
  return logRecords(dataDir).some((rec) => {
    try {
      return predicate(rec)
    } catch {
      return false
    }
  })
}

function logCount(dataDir, predicate) {
  return logRecords(dataDir).filter((rec) => {
    try {
      return predicate(rec)
    } catch {
      return false
    }
  }).length
}

const sawIncidentCreated = (dataDir) =>
  logMentions(
    dataDir,
    (rec) => rec.target === 'interpretation.incident.created' && rec.fields?.created === true,
  )

function startTauriDriver() {
  const cliJs = require.resolve('@crabnebula/tauri-driver/cli.js')
  tauriDriver = spawn(process.execPath, [cliJs, '--native-driver', MSEDGEDRIVER_PATH], {
    stdio: ['ignore', 'pipe', 'pipe'],
    env: {
      ...process.env,
      ANDROMEDA_PULSE_DATA_DIR: PULSE_DATA_DIR,
      ANDROMEDA_PULSE_LOG_LEVEL: 'debug',
      // The app under test is a grandchild (tauri-driver spawns it), so the
      // mode gate has to ride this env. Without it the L4 path attempts real
      // model inference, finds no model, and the incident never forms — which
      // looks identical to a broken storm→incident seam.
      ANDROMEDA_PULSE_L4_DETERMINISTIC: 'true',
    },
  })
  tauriDriver.stdout.on('data', (d) => process.stdout.write(`[tauri-driver] ${d}`))
  tauriDriver.stderr.on('data', (d) => process.stderr.write(`[tauri-driver] ${d}`))
}

// The injector is a separate OTLP client process (arch §Test-time telemetry
// injection: no in-process bypass). It is spawned HERE rather than by the xtask
// caller because stage ordering is the point — the empty-table stage must
// observe the table before any telemetry exists, and the populate stage is only
// honest if nothing reloads the view in between.
function startInjector() {
  if (!PULSE_INJECTOR) {
    console.log('webview-drive: PULSE_INJECTOR unset — injector-suppressed arm')
    return false
  }
  injector = spawn(PULSE_INJECTOR, [], { stdio: ['ignore', 'pipe', 'pipe'] })
  injector.stdout.on('data', (d) => process.stdout.write(`[inject] ${d}`))
  injector.stderr.on('data', (d) => process.stderr.write(`[inject] ${d}`))
  injector.on('error', (err) => console.error(`webview-drive: injector spawn failed: ${err?.message ?? err}`))
  return true
}

// Four webview windows exist (compact-widget / main / findings / report) and the
// session lands on an arbitrary one, so every stage that touches the dashboard
// resolves the handle rather than trusting a handle captured earlier.
//
// The tab nav is the discriminator, NOT the Investigate button: `Titlebar` is
// shared, so the compact widget carries an identically-labelled Investigate
// control. Keying on it matched the widget and left every dashboard stage dark.
async function switchToDashboard(browser) {
  const handles = await browser.getWindowHandles()
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    if (await browser.$(TAB_NAV).isExisting()) return handle
  }
  return null
}

// Switch by the REAL Tauri window label — the one identity URL + markers can't
// fake. NOTE (measured): the label is injected via an initialization script
// that runs on about:blank too, so label readability is NOT a navigation
// signal — a window can answer its label while still blank. Navigation is
// proven only by getUrl() leaving about:blank.
async function switchToLabel(browser, label) {
  const handles = await browser.getWindowHandles()
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    let found = null
    try {
      found = await browser.execute(
        () => globalThis.__TAURI_INTERNALS__?.metadata?.currentWindow?.label ?? null,
      )
    } catch {
      found = null
    }
    if (found === label) return handle
  }
  return null
}

// Window getters via the Tauri IPC the app itself uses cross-window
// (use-toggle-dashboard finds `main` from the widget webview) — the getters
// live in core:window:default, so no extra grant. Callers must be switched to
// a NAVIGATED tauri webview; the target window is named by label.
// Last invoke failure, surfaced into stage details so a broken transport is
// distinguishable from a genuinely-false visibility read (null vs false).
let lastInvokeError = null

async function windowInvoke(browser, cmd, label) {
  try {
    const result = await browser.executeAsync(
      (cmd, label, done) => {
        const t = globalThis.__TAURI_INTERNALS__
        if (!t || typeof t.invoke !== 'function') return done({ error: 'no-tauri-internals' })
        t.invoke(`plugin:window|${cmd}`, { label })
          .then((value) => done({ value }))
          .catch((e) => done({ error: String(e) }))
      },
      cmd,
      label,
    )
    if (result && 'value' in result) return result.value
    lastInvokeError = `${cmd}(${label}): ${result?.error ?? 'no result'}`
    return null
  } catch (e) {
    lastInvokeError = `${cmd}(${label}): ${e?.message ?? e}`
    return null
  }
}

// Position/size payloads normalize across the two serde shapes tauri versions
// have used ({x,y} plain vs {Physical:{x,y}}).
const asPoint = (v) =>
  v && typeof v.x === 'number' ? { x: v.x, y: v.y } : (v?.Physical ?? null)
const asSize = (v) =>
  v && typeof v.width === 'number'
    ? { width: v.width, height: v.height }
    : (v?.Physical ?? null)

async function isWindowVisible(browser, label) {
  return (await windowInvoke(browser, 'is_visible', label)) === true
}

// Root-cause remediation for the measured startup race: each boot under the
// automation environment loses ~ONE of the four webviews' initial navigation
// (victim ~random — main run A, findings run B, report run C), and nothing
// ever re-navigates it — which is exactly why a toggle press could never
// recover a blank main (show ≠ navigate; 0/10 measured). Re-navigate every
// about:blank victim to the app origin (copied from a healthy sibling), and
// record WHO was recovered so the race stays visible in every report. Runs at
// launch, before any stage asserts anything, so the traces no-reload invariant
// (which starts at traces-empty) is untouched.
async function recoverBlankWindows(browser) {
  const recovered = []
  let origin = null
  for (const handle of await browser.getWindowHandles()) {
    await browser.switchToWindow(handle)
    let url = null
    try {
      url = await browser.getUrl()
    } catch {
      continue
    }
    if (url && url !== 'about:blank') {
      origin = new URL(url).origin
      break
    }
  }
  if (!origin) return recovered
  for (const handle of await browser.getWindowHandles()) {
    await browser.switchToWindow(handle)
    let url = null
    try {
      url = await browser.getUrl()
    } catch {
      continue
    }
    if (url !== 'about:blank') continue
    let label = null
    try {
      label = await browser.execute(
        () => globalThis.__TAURI_INTERNALS__?.metadata?.currentWindow?.label ?? null,
      )
    } catch {
      /* label stays null */
    }
    try {
      await browser.url(`${origin}/`)
      recovered.push(label ?? 'unknown')
      console.log(`webview-drive: recovered blank webview (label=${label}) via ${origin}/`)
    } catch (err) {
      console.log(`webview-drive: FAILED to recover blank webview (label=${label}): ${err?.message ?? err}`)
    }
  }
  return recovered
}

// Mechanics probe. W3C pointer Actions and WebDriver window-rect manipulation
// have never been exercised against a WRY window on this host, so whether a drag
// or resize stage can exist AT ALL is a measurement, not an assumption. Finding
// a mechanism unsupported is a RESULT — the probe records what it found and
// never fails the leg.
//
// Input is what is under test (Actions / setWindowRect); READBACK deliberately
// uses the Tauri window getters already proven by the other stages, so a null
// delta means the input did nothing rather than that the measurement is unproven.
// Runs against `main`, whose geometry no later stage keys on (findings docks off
// the WIDGET), and restores what it moved.
async function probeWindowMechanics(browser) {
  const out = {
    geometry_readable: false,
    drag_region_found: false,
    pointer_actions_ran: false,
    pointer_actions_error: null,
    drag_delta: null,
    set_window_rect_ran: false,
    set_window_rect_error: null,
    resize_delta: null,
  }
  if ((await switchToDashboard(browser)) === null) {
    out.pointer_actions_error = 'dashboard handle not resolvable'
    return out
  }

  const posBefore = asPoint(await windowInvoke(browser, 'outer_position', 'main'))
  const sizeBefore = asSize(await windowInvoke(browser, 'outer_size', 'main'))
  out.geometry_readable = Boolean(posBefore && sizeBefore)

  const region = await browser.$(DRAG_REGION)
  out.drag_region_found = await region.isExisting()
  if (out.drag_region_found) {
    try {
      await browser
        .action('pointer', { parameters: { pointerType: 'mouse' } })
        .move({ origin: region })
        .down({ button: 0 })
        .move({ origin: 'pointer', x: PROBE_DRAG_DX, y: PROBE_DRAG_DY, duration: 200 })
        .up({ button: 0 })
        .perform()
      out.pointer_actions_ran = true
    } catch (err) {
      out.pointer_actions_error = String(err?.message ?? err)
    }
  }
  const posAfter = asPoint(await windowInvoke(browser, 'outer_position', 'main'))
  if (posBefore && posAfter) {
    out.drag_delta = { dx: posAfter.x - posBefore.x, dy: posAfter.y - posBefore.y }
  }

  if (sizeBefore) {
    try {
      await browser.setWindowRect(
        null,
        null,
        sizeBefore.width + PROBE_RESIZE_DW,
        sizeBefore.height + PROBE_RESIZE_DH,
      )
      out.set_window_rect_ran = true
    } catch (err) {
      out.set_window_rect_error = String(err?.message ?? err)
    }
    const sizeAfter = asSize(await windowInvoke(browser, 'outer_size', 'main'))
    if (sizeAfter) {
      out.resize_delta = {
        dw: sizeAfter.width - sizeBefore.width,
        dh: sizeAfter.height - sizeBefore.height,
      }
    }
  }

  // Best-effort restore so later stages see the geometry they booted with.
  try {
    await browser.setWindowRect(
      posBefore?.x ?? null,
      posBefore?.y ?? null,
      sizeBefore?.width ?? null,
      sizeBefore?.height ?? null,
    )
  } catch {
    /* the report records what was measured; a failed restore is not a stage */
  }
  return out
}

async function switchToInvestigateHost(browser) {
  const handles = await browser.getWindowHandles()
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    if (await browser.$(INVESTIGATE).isExisting()) return handle
  }
  return null
}

// One-shot ground truth about what each webview actually contains. A stage that
// never satisfies is otherwise indistinguishable between "wrong window" and
// "selector wrong", and guessing between those costs a full run each time.
// Returns a derived per-handle summary so a failing launch record carries its
// own diagnosis instead of only console scrollback.
async function dumpHandles(browser, label) {
  const handles = await browser.getWindowHandles()
  const summary = []
  console.log(`webview-drive: --- handles (${label}): ${handles.length} ---`)
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    const marks = {}
    for (const [name, sel] of Object.entries({
      tabNav: TAB_NAV,
      traceTable: TRACE_TABLE,
      investigate: INVESTIGATE,
      toggleDashboard: TOGGLE_DASHBOARD,
      closeToTray: CLOSE_TO_TRAY,
    })) {
      marks[name] = await browser.$(sel).isExisting()
    }
    let url = '<unreadable>'
    try {
      url = await browser.getUrl()
    } catch {
      /* a hidden window may refuse */
    }
    // The Tauri window label is the only unambiguous identity: App.tsx renders
    // <Dashboard /> for BOTH `main` and the `unknown` fallback, so URL plus
    // marker presence cannot tell those apart while the SPA is still mounting.
    let winLabel = '<unknown>'
    try {
      winLabel = await browser.execute(
        () => globalThis.__TAURI_INTERNALS__?.metadata?.currentWindow?.label ?? '<none>',
      )
    } catch {
      /* not a Tauri webview, or internals not exposed */
    }
    console.log(`webview-drive:   ${handle} label=${winLabel} url=${url} ${JSON.stringify(marks)}`)
    summary.push({ label: winLabel, navigated: url !== 'about:blank', ...marks })
  }
  return summary
}

// Every window boots `visible: false` (tauri.conf.json); the widget is shown at
// boot and the dashboard only when its affordance is pressed. Pressing the real
// in-widget control is also what makes this a drive rather than a URL jump.
async function openDashboard(browser) {
  if ((await switchToDashboard(browser)) !== null) return true
  const handles = await browser.getWindowHandles()
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    const toggle = await browser.$(TOGGLE_DASHBOARD)
    if (await toggle.isExisting()) {
      await toggle.click()
      return true
    }
  }
  return false
}

// The compact widget is the surface whose close is the full app-to-tray path;
// the dashboard carries its own ✕ that only collapses to the widget.
//
// The exclusion is the TAB NAV, not the trace table: the table exists only on
// the /traces route, so once an earlier stage navigates the dashboard to /logs
// the dashboard stops matching a `!traceTable` exclusion and gets picked as the
// widget — measured, and the app then correctly logged `main → hidden`.
async function switchToWidget(browser) {
  const handles = await browser.getWindowHandles()
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    const hasClose = await browser.$(CLOSE_TO_TRAY).isExisting()
    const hasTabNav = await browser.$(TAB_NAV).isExisting()
    if (hasClose && !hasTabNav) return handle
  }
  return null
}

async function countRows(browser) {
  const rows = await browser.$$(TRACE_ROW)
  return rows.length
}

async function clickTab(browser, id) {
  const tab = await browser.$(TAB(id))
  if (!(await tab.isExisting())) return false
  await tab.click()
  return true
}

async function readEmptyState(browser, route) {
  const empty = await browser.$(EMPTY_STATE(route)).isExisting()
  const hint = await browser.$(EMPTY_STATE_HINT(route)).isExisting()
  const error = await browser.$(ERROR_STATE(route)).isExisting()
  return { empty, hint, error }
}

async function main() {
  startTauriDriver()
  if (!(await pollUntil('tauri-driver listening', DRIVER_READY_TIMEOUT_MS, () => portAccepts(DRIVER_PORT)))) {
    console.error(`webview-drive: tauri-driver never bound 127.0.0.1:${DRIVER_PORT}`)
    return 4
  }

  const browser = await remote({
    hostname: '127.0.0.1',
    port: DRIVER_PORT,
    connectionRetryCount: 1,
    connectionRetryTimeout: SESSION_TIMEOUT_MS,
    logLevel: 'error',
    capabilities: { browserName: 'wry', 'tauri:options': { application: PULSE_BIN } },
  })

  try {
    // Stage 1 — launch. Wait on the POSITIVE NAVIGATION SIGNAL first: a
    // webview that left about:blank exposes __TAURI_INTERNALS__, so `main`'s
    // label becomes readable — which separates "SPA still mounting" (navigated;
    // keep waiting for the tab nav) from the measured startup race ("main"
    // never navigates; a toggle press cannot mount an unnavigated SPA — 0/10
    // across 14 runs, 50s already elapsing in failing runs).
    await dumpHandles(browser, 'before launch')
    const recoveredBlank = await recoverBlankWindows(browser)
    // Labels are readable on about:blank (initialization-script injection), so
    // the navigation signal is the URL itself leaving about:blank.
    const mainNavigated = async () => {
      if ((await switchToLabel(browser, 'main')) === null) return false
      let url = null
      try {
        url = await browser.getUrl()
      } catch {
        return false
      }
      return !!url && url !== 'about:blank'
    }
    const navigated = await pollUntil('main webview navigated', MOUNT_GRACE_MS, mainNavigated)
    // Press ONCE, never inside a poll: re-pressing each interval would toggle
    // the dashboard shut again, a ~120-press flicker whose outcome is decided
    // by which parity the deadline lands on.
    let toggled = false
    let found = false
    if (navigated) {
      found = await pollUntil(
        'dashboard mount',
        DASHBOARD_TIMEOUT_MS,
        async () => (await switchToDashboard(browser)) !== null,
      )
    } else {
      // Never-navigated: press the real affordance ONCE anyway — the press is
      // free evidence (its measured uselessness here is the race's signature),
      // and a hypothetical show-triggered late navigation would still be
      // caught by the re-check.
      toggled = await openDashboard(browser)
      const lateNavigated = await pollUntil(
        'main webview navigated (post-toggle)',
        DASHBOARD_TIMEOUT_MS,
        mainNavigated,
      )
      if (lateNavigated) {
        found = await pollUntil(
          'dashboard mount (post-toggle)',
          DASHBOARD_TIMEOUT_MS,
          async () => (await switchToDashboard(browser)) !== null,
        )
      }
    }
    const handles = await dumpHandles(browser, found ? 'launch' : 'after failed launch')
    record('launch', found, {
      windows: handles.length,
      toggle_pressed: toggled,
      main_navigated: await mainNavigated(),
      recovered_from_blank: recoveredBlank,
      handles,
    })
    if (!found) return 3
    await switchToDashboard(browser)

    // Mechanics probe — runs before any stage asserts, so its findings can gate
    // which window-mechanic stages exist. Not a stage: `observed` records only
    // that the probe completed, and the Rust side reads the detail fields.
    let mechanics = null
    try {
      mechanics = await probeWindowMechanics(browser)
      record('mechanics-probe', true, mechanics)
    } catch (err) {
      record('mechanics-probe', false, { probe_error: String(err?.message ?? err) })
    }
    await switchToDashboard(browser)

    // Stage 2 — traces-empty: the pre-state stage 3 needs. Nothing has been
    // injected yet, so an empty table here is what makes empty→populated real.
    await clickTab(browser, 'traces')
    const emptyShown = await pollUntil('empty trace table', ABSENT_TIMEOUT_MS, async () => {
      await switchToDashboard(browser)
      return browser.$(TRACE_TABLE_EMPTY).isExisting()
    })
    record('traces-empty', emptyShown, {
      rows_before: await countRows(browser),
      // The a11y anchors the leg now binds to — a regression on either
      // reddens the run's diagnosis even though `observed` keys on the cell.
      region_present: await browser.$(TRACE_REGION).isExisting(),
      table_present: await browser.$(TRACE_TABLE).isExisting(),
    })

    // Stage 3 — traces-populate: telemetry starts flowing and the table fills
    // ON ITS OWN. No reload, no re-navigation between here and the assertion.
    const injecting = startInjector()
    const populated = await pollUntil('populated trace table', POPULATE_TIMEOUT_MS, async () => {
      await switchToDashboard(browser)
      return (await countRows(browser)) > 0
    })
    await switchToDashboard(browser)
    record('traces-populate', populated, {
      injector_started: injecting,
      rows_after: await countRows(browser),
    })

    // Stage 4 — traces-scroll (P-082, default window size): the table body
    // scrolls INSIDE its own region while the page does not. Rows keep
    // arriving while the injector runs, so poll until the region overflows —
    // asserting page containment in the SAME probe, because the defect mode
    // is precisely "the outer page scrolls instead".
    const scrollProbe = () =>
      browser.execute((scrollSel) => {
        const el = globalThis.document.querySelector(scrollSel)
        const doc = globalThis.document.scrollingElement || globalThis.document.documentElement
        return {
          scroll_region_found: !!el,
          region_overflows: !!el && el.scrollHeight > el.clientHeight,
          page_overflows: doc.scrollHeight > doc.clientHeight,
        }
      }, TRACE_SCROLL)
    await pollUntil('trace table internal overflow', POPULATE_TIMEOUT_MS, async () => {
      await switchToDashboard(browser)
      const s = await scrollProbe()
      return s.scroll_region_found && s.region_overflows && !s.page_overflows
    })
    await switchToDashboard(browser)
    const scrollFacts = await scrollProbe()
    record(
      'traces-scroll',
      scrollFacts.scroll_region_found &&
        scrollFacts.region_overflows &&
        !scrollFacts.page_overflows,
      scrollFacts,
    )

    // Stage 4b — native-menu-suppressed (P-064/P-065 residual). The mechanics
    // probe measured that no synthesized OS pointer input reaches this window,
    // so a NATIVE WebView2 context menu can neither be opened nor observed by
    // the driver — the CARRY's "if unobservable, assert the suppression side"
    // branch. Dispatching a REAL contextmenu in the shipped production bundle is
    // evidence vitest cannot give: it exercises the hook in jsdom under a
    // stubbed PROD flag, never the bundle the user actually runs.
    //
    // Placed HERE, not late in the leg: the dashboard is already on /traces so
    // the canvas needed for the P-065 half is present WITHOUT navigating. An
    // earlier revision sat after `empty-states` and clicked back to /traces,
    // which left the downstream window state different from the pristine flow
    // and killed the RED arm's WebDriver session at `dashboard-close`
    // (measured: pristine 184s PASS, with the route change >1200s and no
    // completion). A read-only observer stage must not perturb what follows it.
    await switchToDashboard(browser)
    let suppression = null
    try {
      suppression = await browser.execute(() => {
        // Browser globals via `globalThis` — this callback is serialized into
        // the webview, but the file is linted with Node globals (the existing
        // `__TAURI_INTERNALS__` probes take the same route).
        const doc = globalThis.document
        const menu = new globalThis.MouseEvent('contextmenu', {
          bubbles: true,
          cancelable: true,
        })
        doc.body.dispatchEvent(menu)
        const canvas = doc.querySelector('canvas')
        let dragPrevented = null
        if (canvas) {
          const drag = new globalThis.Event('dragstart', { bubbles: true, cancelable: true })
          canvas.dispatchEvent(drag)
          dragPrevented = drag.defaultPrevented
        }
        return {
          menuPrevented: menu.defaultPrevented,
          canvasFound: Boolean(canvas),
          dragPrevented,
        }
      })
    } catch (err) {
      suppression = { error: String(err?.message ?? err) }
    }
    record('native-menu-suppressed', suppression?.menuPrevented === true, {
      context_menu_prevented: suppression?.menuPrevented ?? null,
      canvas_found: suppression?.canvasFound ?? null,
      canvas_dragstart_prevented: suppression?.dragPrevented ?? null,
      native_menu_observable: false,
      dispatch_error: suppression?.error ?? null,
    })

    // Stage 5 — connection-status (P-070): the dashboard footer's worded line
    // goes live under flowing telemetry; the compact widget must NOT carry it
    // (layout-templates keeps the widget aggregate-glance).
    const statusProbe = () =>
      browser.execute((sel) => {
        const el = globalThis.document.querySelector(sel)
        if (!el) return { line_found: false }
        const text = el.textContent ?? ''
        return {
          line_found: true,
          live_kind: !!el.querySelector('[data-status-kind="live"]'),
          matched_services: /Receiving from \d+ services?/.test(text),
          matched_spans_rate: /spans\/s/.test(text),
          matched_buffer: /buffer \d+ min \/ \d+ min/.test(text),
        }
      }, CONNECTION_STATUS_LINE)
    await pollUntil('connection status line live', POPULATE_TIMEOUT_MS, async () => {
      await switchToDashboard(browser)
      const s = await statusProbe()
      return (
        s.line_found &&
        s.live_kind &&
        s.matched_services &&
        s.matched_spans_rate &&
        s.matched_buffer
      )
    })
    await switchToDashboard(browser)
    const statusFacts = await statusProbe()
    await switchToWidget(browser)
    const lineOnWidget = await browser.$(CONNECTION_STATUS_LINE).isExisting()
    record(
      'connection-status',
      statusFacts.line_found === true &&
        statusFacts.live_kind === true &&
        statusFacts.matched_services === true &&
        statusFacts.matched_spans_rate === true &&
        statusFacts.matched_buffer === true &&
        !lineOnWidget,
      {
        line_on_dashboard: statusFacts.line_found === true,
        live_kind: statusFacts.live_kind === true,
        matched_services: statusFacts.matched_services === true,
        matched_spans_rate: statusFacts.matched_spans_rate === true,
        matched_buffer: statusFacts.matched_buffer === true,
        line_on_widget: lineOnWidget,
      },
    )

    // Stage 6 — storm-incident: wait for the app's own incident record. This is
    // synchronization; the xtask side re-reads the log and owns the verdict.
    const stormBudget = injecting ? STORM_TIMEOUT_MS : ABSENT_TIMEOUT_MS
    const incident = await pollUntil('incident created', stormBudget, async () =>
      sawIncidentCreated(PULSE_DATA_DIR),
    )
    record('storm-incident', incident, {})

    // Stage 7 — findings-window: the incident lights the widget's unread
    // badge; pressing it opens the SEPARATE findings window docked below the
    // widget (2026-07-10 disclosure). Geometry lands as derived booleans and
    // deltas only — never raw coordinates (security-plan §Input Validation).
    // Runs BEFORE investigate so no focus-trapping modal occludes the widget.
    const badgeShown = await pollUntil('findings badge', ABSENT_TIMEOUT_MS, async () => {
      await switchToWidget(browser)
      return browser.$(FINDINGS_COUNTER).isExisting()
    })
    let badgePressed = false
    if (badgeShown) {
      await switchToWidget(browser)
      await (await browser.$(FINDINGS_COUNTER)).click()
      badgePressed = true
    }
    lastInvokeError = null
    const findingsVisible = await pollUntil('findings window visible', ABSENT_TIMEOUT_MS, async () => {
      await switchToWidget(browser)
      return isWindowVisible(browser, 'findings')
    })
    // The findings webview can sit at about:blank (the same startup-race class
    // the launch stage measures — observed live on THIS window): visible but
    // never navigated means no SPA, no rows, no Esc handler. Record each layer
    // so a red stage names which one died.
    let findingsNavigated = false
    let findingsSpaMounted = false
    if ((await switchToLabel(browser, 'findings')) !== null) {
      findingsNavigated = await pollUntil('findings webview navigated', ABSENT_TIMEOUT_MS, async () => {
        if ((await switchToLabel(browser, 'findings')) === null) return false
        let url = null
        try {
          url = await browser.getUrl()
        } catch {
          return false
        }
        return !!url && url !== 'about:blank'
      })
    }
    if (findingsNavigated) {
      findingsSpaMounted = await pollUntil('findings SPA mount', ABSENT_TIMEOUT_MS, async () => {
        if ((await switchToLabel(browser, 'findings')) === null) return false
        return browser.$('[data-testid="findings-window"]').isExisting()
      })
    }
    let findingsRows = 0
    if (findingsSpaMounted) {
      await pollUntil('findings rows', ABSENT_TIMEOUT_MS, async () => {
        return (await browser.$$(FINDINGS_ROW)).length > 0
      })
      findingsRows = (await browser.$$(FINDINGS_ROW)).length
    }
    await switchToWidget(browser)
    const widgetPos = asPoint(await windowInvoke(browser, 'outer_position', 'compact-widget'))
    const widgetSize = asSize(await windowInvoke(browser, 'outer_size', 'compact-widget'))
    const findingsPos = asPoint(await windowInvoke(browser, 'outer_position', 'findings'))
    const dockGap =
      widgetPos && widgetSize && findingsPos
        ? findingsPos.y - (widgetPos.y + widgetSize.height)
        : null
    const dockedBelow = dockGap !== null && dockGap >= 0
    record('findings-window', badgePressed && findingsVisible && findingsRows > 0 && dockedBelow, {
      badge_pressed: badgePressed,
      findings_visible: findingsVisible,
      findings_navigated: findingsNavigated,
      spa_mounted: findingsSpaMounted,
      row_count: findingsRows,
      docked_below: dockedBelow,
      dock_gap_px: dockGap,
      invoke_error: lastInvokeError,
    })

    // Stage 8 — report-window: a row-select opens the report window BESIDE the
    // findings window, and the Esc chain unwinds it — report hides with focus
    // returning to findings, findings hides with DOM focus restored to the
    // widget's badge (a11y-plan §5 Focus restoration, SC 2.1.2).
    let rowPressed = false
    if ((await switchToLabel(browser, 'findings')) !== null) {
      const row = await browser.$(FINDINGS_ROW)
      if (await row.isExisting()) {
        await row.click()
        rowPressed = true
      }
    }
    const reportVisible =
      rowPressed &&
      (await pollUntil('report window visible', ABSENT_TIMEOUT_MS, async () => {
        await switchToWidget(browser)
        return isWindowVisible(browser, 'report')
      }))
    // Gate the DOM probe on visibility: the report webview renders its Report
    // with a first-active-incident FALLBACK even while hidden (measured — a
    // hidden window's dialog satisfied this probe on the first run), so an
    // ungated read is vacuously green.
    let dialogShown = false
    if (reportVisible && (await switchToLabel(browser, 'report')) !== null) {
      dialogShown = await pollUntil('report dialog', ABSENT_TIMEOUT_MS, async () =>
        browser.$(MODAL_DIALOG).isExisting(),
      )
    }
    await switchToWidget(browser)
    let positionedBeside = false
    if (reportVisible) {
      const fPos = asPoint(await windowInvoke(browser, 'outer_position', 'findings'))
      const fSize = asSize(await windowInvoke(browser, 'outer_size', 'findings'))
      const rPos = asPoint(await windowInvoke(browser, 'outer_position', 'report'))
      const rSize = asSize(await windowInvoke(browser, 'outer_size', 'report'))
      // Left of findings preferred, right fallback (computeReportWindowPosition);
      // slack absorbs the dock gap + rounding.
      const BESIDE_SLACK_PX = 16
      positionedBeside =
        !!(fPos && fSize && rPos && rSize) &&
        (rPos.x + rSize.width <= fPos.x + BESIDE_SLACK_PX ||
          rPos.x >= fPos.x + fSize.width - BESIDE_SLACK_PX)
    }
    let reportHidden = false
    if (dialogShown && (await switchToLabel(browser, 'report')) !== null) {
      await browser.keys([Key.Escape])
      reportHidden = await pollUntil('report hidden after Escape', ABSENT_TIMEOUT_MS, async () => {
        await switchToWidget(browser)
        return (await windowInvoke(browser, 'is_visible', 'report')) === false
      })
    }
    let findingsHidden = false
    if ((await switchToLabel(browser, 'findings')) !== null) {
      await browser.keys([Key.Escape])
      findingsHidden = await pollUntil('findings hidden after Escape', ABSENT_TIMEOUT_MS, async () => {
        await switchToWidget(browser)
        return (await windowInvoke(browser, 'is_visible', 'findings')) === false
      })
    }
    // Gate on the dismissal: after the badge CLICK the badge is already the
    // widget's focused element, so an ungated read cannot distinguish
    // click-focus from restored focus (measured vacuously green on run 1).
    let badgeFocusRestored = false
    if (findingsHidden) {
      badgeFocusRestored = await pollUntil('badge focus restored', ABSENT_TIMEOUT_MS, async () => {
        await switchToWidget(browser)
        const active = await browser.execute(
          () => globalThis.document.activeElement?.getAttribute('data-testid') ?? null,
        )
        return active === 'findings-counter'
      })
    }
    record(
      'report-window',
      rowPressed &&
        reportVisible &&
        dialogShown &&
        positionedBeside &&
        reportHidden &&
        findingsHidden &&
        badgeFocusRestored,
      {
        row_pressed: rowPressed,
        report_visible: reportVisible,
        dialog_shown: dialogShown,
        positioned_beside: positionedBeside,
        report_hidden_after_escape: reportHidden,
        findings_hidden_after_escape: findingsHidden,
        badge_focus_restored: badgeFocusRestored,
        invoke_error: lastInvokeError,
      },
    )

    // Stage 9 — investigate: press the REAL control by its accessible name, so
    // a name regression fails this stage.
    // Investigate lives on the WIDGET titlebar, not the dashboard: `Titlebar`
    // renders the control only when handed `onInvestigateClick`, and only the
    // compact widget passes it (measured — the dashboard handle reports the
    // control absent).
    await switchToInvestigateHost(browser)
    const control = await browser.$(INVESTIGATE)
    const exists = await control.isExisting()
    let pressed = false
    let accessibleName = null
    let actionPressed = false
    if (exists) {
      accessibleName = await control.getAttribute('aria-label')
      await control.click()
      pressed = true
      // The modal generates a snapshot first, then offers the preset actions —
      // so wait for the list rather than pressing into an unsettled panel.
      const listed = await pollUntil('preset prompt list', INVESTIGATE_TIMEOUT_MS, async () =>
        browser.$(PRESET_PROMPT_LIST).isExisting(),
      )
      if (listed) {
        const action = await browser.$(PRESET_ACTION)
        if (await action.isExisting()) {
          await action.click()
          actionPressed = true
        }
      }
    }
    const settled = await pollUntil('investigate outcome', INVESTIGATE_TIMEOUT_MS, async () =>
      logMentions(PULSE_DATA_DIR, (rec) => rec.target === 'investigate.run_action.request'),
    )
    // Dismiss the modal before anything else is pressed on this window: it
    // traps focus, so the later close press lands on the backdrop and the app
    // never hides — which reads as a dead close affordance rather than an
    // occluded one. (Escape dismissal is the a11y-mandated path.)
    let modalDismissed = false
    if (pressed) {
      await browser.keys(['Escape'])
      modalDismissed = await pollUntil('modal dismissed', ABSENT_TIMEOUT_MS, async () => {
        const open = await browser.$(PRESET_PROMPT_LIST).isExisting()
        return !open
      })
    }
    record('investigate', pressed && actionPressed, {
      control_found: exists,
      accessible_name: accessibleName,
      action_pressed: actionPressed,
      settled,
      modal_dismissed: modalDismissed,
    })

    // Stage 10 — empty-states: the stream is traces-only, so Metrics and Logs
    // stay empty for the whole run. The HINT is the discriminator — the honest
    // error variant renders a message with no hint.
    const routes = {}
    for (const route of ['metrics', 'logs']) {
      await switchToDashboard(browser)
      const navigated = await clickTab(browser, route)
      const seen = await pollUntil(`${route} empty state`, ABSENT_TIMEOUT_MS, async () => {
        await switchToDashboard(browser)
        const s = await readEmptyState(browser, route)
        return s.empty || s.error
      })
      await switchToDashboard(browser)
      routes[route] = { navigated, seen, ...(await readEmptyState(browser, route)) }
    }
    record('empty-states', routes.metrics.empty && routes.logs.empty, routes)

    // Stage 12 — dashboard-toggle (P-066): the button and the Ctrl+Shift+P
    // binding each prove a direction, across both windows, and the widget
    // stays visible throughout. Both directions run through JS hide()/show()
    // (no Rust record), so the verdict is the window API's own visibility.
    // Press ONCE then poll — never press inside a poll.
    lastInvokeError = null
    const mainVisible = async () => {
      await switchToWidget(browser)
      return isWindowVisible(browser, 'main')
    }
    const pressToggle = async () => {
      await switchToWidget(browser)
      const t = await browser.$(TOGGLE_DASHBOARD)
      if (!(await t.isExisting())) return false
      await t.click()
      return true
    }
    // FLIP assertions, not fixed directions: the toggle contract is
    // open-if-hidden / hide-if-shown, so each press must INVERT the visibility
    // it found — robust to whatever state earlier stages left, where a fixed
    // hide-then-show script drifts off by one and every poll asserts the wrong
    // direction (measured on run 2: presses toggled, the script's state
    // machine did not). Two button presses cover both directions between them;
    // the shortcut fires once from each window.
    const visReadback = async () => {
      await switchToWidget(browser)
      return windowInvoke(browser, 'is_visible', 'main')
    }
    const flip = async (label, pressFn) => {
      const before = await visReadback()
      const pressed = await pressFn()
      let flipped = false
      if (pressed && before !== null) {
        flipped = await pollUntil(`${label} flips dashboard`, ABSENT_TIMEOUT_MS, async () => {
          await switchToWidget(browser)
          return (await windowInvoke(browser, 'is_visible', 'main')) === !before
        })
      }
      const after = await visReadback()
      return { pressed, before, after, flipped }
    }
    const button1 = await flip('toggle button (1st)', pressToggle)
    const button2 = await flip('toggle button (2nd)', pressToggle)
    // The binding is mounted in BOTH windows; a hidden dashboard still hosts
    // its handler, so sending there is valid in either direction.
    const shortcutDashboard = await flip('shortcut from dashboard', async () => {
      if ((await switchToDashboard(browser)) === null) return false
      await browser.keys([Key.Ctrl, Key.Shift, 'p'])
      return true
    })
    const shortcutWidget = await flip('shortcut from widget', async () => {
      await switchToWidget(browser)
      await browser.keys([Key.Ctrl, Key.Shift, 'p'])
      return true
    })
    await switchToWidget(browser)
    const widgetStayed =
      (await isWindowVisible(browser, 'compact-widget')) &&
      (await browser.$(TOGGLE_DASHBOARD).isExisting())
    // Leave the dashboard SHOWN for the dashboard-close stage: after an even
    // number of flips it is back to shown, but state-correct rather than
    // assumed — press once more if the last readback says hidden.
    if (shortcutWidget.after === false) {
      await pressToggle()
      await pollUntil('dashboard restored for close stage', ABSENT_TIMEOUT_MS, mainVisible)
    }
    record(
      'dashboard-toggle',
      button1.flipped && button2.flipped && shortcutDashboard.flipped && shortcutWidget.flipped && widgetStayed,
      {
        button_flip_1: button1.flipped,
        button_flip_2: button2.flipped,
        shortcut_flip_dashboard: shortcutDashboard.flipped,
        shortcut_flip_widget: shortcutWidget.flipped,
        widget_stayed: widgetStayed,
        button_press_1: button1.pressed,
        button_press_2: button2.pressed,
        shortcut_sent_dashboard: shortcutDashboard.pressed,
        shortcut_sent_widget: shortcutWidget.pressed,
        main_before_1: button1.before,
        main_after_1: button1.after,
        main_after_2: button2.after,
        main_after_3: shortcutDashboard.after,
        main_after_4: shortcutWidget.after,
        invoke_error: lastInvokeError,
      },
    )

    // Stage 13 — dashboard-close (P-063): the dashboard's own ✕ collapses to
    // the widget. The `main → hidden` record is written ONLY by this path
    // (the toggle hides via JS with no record), and the close is SILENT — the
    // signpost count must not move; the toast belongs to the widget close.
    const signpostCount = () =>
      logCount(PULSE_DATA_DIR, (rec) => rec.target === 'tray.signpost.shown')
    const signpostsBefore = signpostCount()
    let dashboardClosePressed = false
    if ((await switchToDashboard(browser)) !== null) {
      const dashClose = await browser.$(CLOSE_TO_TRAY)
      if (await dashClose.isExisting()) {
        await dashClose.click()
        dashboardClosePressed = true
      }
    }
    const mainHiddenRecord = await pollUntil('main→hidden transition', ABSENT_TIMEOUT_MS, async () =>
      logMentions(
        PULSE_DATA_DIR,
        (rec) =>
          rec.target === 'ui.layout.transition' &&
          rec.fields?.layout_mode_from === 'main' &&
          rec.fields?.layout_mode_to === 'hidden',
      ),
    )
    const mainHiddenNow = await pollUntil('main hidden after close', ABSENT_TIMEOUT_MS, async () => {
      await switchToWidget(browser)
      return (await windowInvoke(browser, 'is_visible', 'main')) === false
    })
    const signpostDelta = signpostCount() - signpostsBefore
    record(
      'dashboard-close',
      dashboardClosePressed && mainHiddenRecord && mainHiddenNow && signpostDelta === 0,
      {
        close_pressed: dashboardClosePressed,
        main_hidden: mainHiddenNow,
        transition_seen: mainHiddenRecord,
        signpost_delta: signpostDelta,
      },
    )

    // Stage 14 — widget-close: this press sends the app to the tray. It WAS the
    // terminal stage; signpost-repeat now follows it deliberately, restoring the
    // widget to prove the toast fires on every close rather than only the first.
    // Carried forward from the single-press leg: a guard on
    // `core:window:allow-close`, which the ACL drops silently when ungranted.
    const widget = await switchToWidget(browser)
    let closePressed = false
    let closeName = null
    if (widget !== null) {
      const close = await browser.$(CLOSE_TO_TRAY)
      if (await close.isExisting()) {
        closeName = await close.getAttribute('aria-label')
        await close.click()
        closePressed = true
      }
    }
    const hidden = await pollUntil('widget→hidden transition', ABSENT_TIMEOUT_MS, async () =>
      logMentions(
        PULSE_DATA_DIR,
        (rec) =>
          rec.target === 'ui.layout.transition' &&
          rec.fields?.layout_mode_to === 'hidden' &&
          rec.fields?.layout_mode_from === 'compact-widget',
      ),
    )
    // The every-time P-063 signpost, by its bounded label FIELD — record
    // presence alone is exactly what the exact allowlist leaf rules out.
    const signpostSeen = await pollUntil('close signpost', ABSENT_TIMEOUT_MS, async () =>
      logMentions(
        PULSE_DATA_DIR,
        (rec) =>
          rec.target === 'tray.signpost.shown' &&
          rec.fields?.window_label === 'compact-widget',
      ),
    )
    record('widget-close', closePressed, {
      widget_found: widget !== null,
      accessible_name: closeName,
      transition_seen: hidden,
      signpost_seen: signpostSeen,
    })

    // Stage 15 — signpost-repeat (P-063 every-time half). One close proves the
    // signpost fires; it cannot prove it fires EVERY time. A second close needs
    // the widget back, and tray restore is an OS surface no synthesized input
    // reaches (the probe measured pointer Actions arriving with no effect), so
    // the re-show is programmatic SETUP. The ASSERTION still rests only on real
    // presses: both closes are clicks on the production control.
    const labelledSignposts = () =>
      logCount(
        PULSE_DATA_DIR,
        (rec) =>
          rec.target === 'tray.signpost.shown' &&
          rec.fields?.window_label === 'compact-widget',
      )
    const signpostsAfterFirst = labelledSignposts()
    const reshown = await windowInvoke(browser, 'show', 'compact-widget')
    const widgetBack = await pollUntil('widget re-shown for second close', ABSENT_TIMEOUT_MS, () =>
      isWindowVisible(browser, 'compact-widget'),
    )
    let secondClosePressed = false
    if (widgetBack && (await switchToWidget(browser)) !== null) {
      const closeAgain = await browser.$(CLOSE_TO_TRAY)
      if (await closeAgain.isExisting()) {
        await closeAgain.click()
        secondClosePressed = true
      }
    }
    const signpostRepeated = await pollUntil('second close signpost', ABSENT_TIMEOUT_MS, async () =>
      labelledSignposts() > signpostsAfterFirst,
    )
    record('signpost-repeat', secondClosePressed && signpostRepeated, {
      second_close_pressed: secondClosePressed,
      widget_reshown: widgetBack,
      reshow_result: reshown,
      signposts_after_first: signpostsAfterFirst,
      signposts_after_second: labelledSignposts(),
      restore_mechanism: 'programmatic-show (tray is an OS surface the driver cannot reach)',
      invoke_error: lastInvokeError,
    })

    return 0
  } finally {
    writeReport()
    injector?.kill()
    await browser.deleteSession().catch(() => {})
  }
}

main()
  .then(async (code) => {
    await sleep(500)
    injector?.kill()
    tauriDriver?.kill()
    process.exit(code)
  })
  .catch(async (err) => {
    console.error(`webview-drive: ${err?.message ?? err}`)
    writeReport()
    await sleep(500)
    injector?.kill()
    tauriDriver?.kill()
    process.exit(1)
  })
