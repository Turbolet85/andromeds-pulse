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
const { remote } = require('webdriverio')

const PULSE_BIN = process.env.PULSE_BIN
const PULSE_DATA_DIR = process.env.PULSE_DATA_DIR
const MSEDGEDRIVER_PATH = process.env.MSEDGEDRIVER_PATH
const PULSE_INJECTOR = process.env.PULSE_INJECTOR ?? ''

const TRACE_TABLE = '[data-testid="trace-table"]'
const TRACE_TABLE_EMPTY = '[data-testid="trace-table-empty"]'
const TRACE_ROW = '[data-testid="trace-row"]'
const INVESTIGATE = 'button[aria-label="Investigate"]'
const CLOSE_TO_TRAY = 'button[aria-label="Close to tray"]'
const TOGGLE_DASHBOARD = 'button[aria-label="Toggle dashboard"]'
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
function logMentions(dataDir, predicate) {
  const logDir = join(dataDir, 'logs')
  let names
  try {
    names = readdirSync(logDir)
  } catch {
    return false
  }
  return names
    .filter((n) => n.startsWith('agent-latest.jsonl'))
    .some((n) => {
      let body
      try {
        body = readFileSync(join(logDir, n), 'utf8')
      } catch {
        return false
      }
      return body
        .split('\n')
        .filter(Boolean)
        .some((line) => {
          try {
            return predicate(JSON.parse(line))
          } catch {
            return false
          }
        })
    })
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
async function dumpHandles(browser, label) {
  const handles = await browser.getWindowHandles()
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
    let label = '<unknown>'
    try {
      label = await browser.execute(
        () => globalThis.__TAURI_INTERNALS__?.metadata?.currentWindow?.label ?? '<none>',
      )
    } catch {
      /* not a Tauri webview, or internals not exposed */
    }
    console.log(`webview-drive:   ${handle} label=${label} url=${url} ${JSON.stringify(marks)}`)
  }
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
    // Stage 1 — launch: the dashboard surface is reachable, which on this shell
    // means pressing the real in-widget affordance that opens it.
    await dumpHandles(browser, 'before toggle')
    // The dashboard webview mounts on its own schedule — measured present at
    // /traces before any toggle in some runs and still at / in others. So wait
    // for a self-mount FIRST, and only press the toggle if it stays absent;
    // pressing while it is merely slow would hide a window that was coming up.
    let found = await pollUntil(
      'dashboard self-mount',
      MOUNT_GRACE_MS,
      async () => (await switchToDashboard(browser)) !== null,
    )
    // Press ONCE, never inside a poll: re-pressing each interval would toggle
    // the dashboard shut again, a ~120-press flicker whose outcome is decided
    // by which parity the deadline lands on.
    let toggled = false
    if (!found) {
      toggled = await openDashboard(browser)
      found = await pollUntil(
        'dashboard window',
        DASHBOARD_TIMEOUT_MS,
        async () => (await switchToDashboard(browser)) !== null,
      )
    }
    if (!found) await dumpHandles(browser, 'after failed toggle')
    record('launch', found, {
      windows: (await browser.getWindowHandles()).length,
      toggle_pressed: toggled,
    })
    if (!found) return 3
    await switchToDashboard(browser)

    // Stage 2 — traces-empty: the pre-state stage 3 needs. Nothing has been
    // injected yet, so an empty table here is what makes empty→populated real.
    await clickTab(browser, 'traces')
    const emptyShown = await pollUntil('empty trace table', ABSENT_TIMEOUT_MS, async () => {
      await switchToDashboard(browser)
      return browser.$(TRACE_TABLE_EMPTY).isExisting()
    })
    record('traces-empty', emptyShown, { rows_before: await countRows(browser) })

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

    // Stage 4 — storm-incident: wait for the app's own incident record. This is
    // synchronization; the xtask side re-reads the log and owns the verdict.
    const stormBudget = injecting ? STORM_TIMEOUT_MS : ABSENT_TIMEOUT_MS
    const incident = await pollUntil('incident created', stormBudget, async () =>
      sawIncidentCreated(PULSE_DATA_DIR),
    )
    record('storm-incident', incident, {})

    // Stage 5 — investigate: press the REAL control by its accessible name, so
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

    // Stage 6 — empty-states: the stream is traces-only, so Metrics and Logs
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

    // Stage 7 — widget-close: terminal, because this press sends the app to the
    // tray. Carried forward from the single-press leg: it is the only guard on
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
    record('widget-close', closePressed, {
      widget_found: widget !== null,
      accessible_name: closeName,
      transition_seen: hidden,
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
