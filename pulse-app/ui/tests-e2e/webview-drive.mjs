// Headful webview driver: presses a real control in the live Tauri window.
//
// The boot-quit `cargo xtask self-verify` harness never clicks (it hides/quits
// programmatically), and the Playwright a11y suite drives a static `dist` in
// Chromium — so neither can catch a dead affordance. This script closes that
// gap via the test-plan §6 stack (tauri-driver + WebdriverIO).
//
// It presses the control and exits; it deliberately asserts NOTHING about the
// effect. The proof is the `ui.layout.transition` record the app itself writes
// to the obs log, which the xtask caller reads — a press that "returned without
// throwing" is not evidence that anything happened.
//
// Invoked by `cargo xtask webview-drive`. Env contract:
//   PULSE_BIN          absolute path to the pulse-app binary under test
//   PULSE_DATA_DIR     per-run data dir (also the obs-log root)
//   MSEDGEDRIVER_PATH  msedgedriver matching the installed WebView2 runtime
import { spawn } from 'node:child_process'
import { createRequire } from 'node:module'
import { createConnection } from 'node:net'
import { readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

const require = createRequire(import.meta.url)
const { remote } = require('webdriverio')

const PULSE_BIN = process.env.PULSE_BIN
const PULSE_DATA_DIR = process.env.PULSE_DATA_DIR
const MSEDGEDRIVER_PATH = process.env.MSEDGEDRIVER_PATH
const CLOSE_SELECTOR = 'button[aria-label="Close to tray"]'
const SESSION_TIMEOUT_MS = 60_000
const DRIVER_PORT = 4444
const POLL_INTERVAL_MS = 250
const DRIVER_READY_TIMEOUT_MS = 30_000
// Absence is the RED arm's expected outcome, so that arm necessarily spends the
// whole budget; keep it short enough to stay inside the xtask-side timeout.
const TRANSITION_TIMEOUT_MS = 20_000

for (const [name, value] of Object.entries({ PULSE_BIN, PULSE_DATA_DIR, MSEDGEDRIVER_PATH })) {
  if (!value) {
    console.error(`webview-drive: missing required env ${name}`)
    process.exit(2)
  }
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms))
let tauriDriver

// Poll a condition to a deadline instead of sleeping a guessed interval —
// test-plan §11 E2E bans sleep(N) synchronization, and a fixed wait here is also
// a race: tearing the session down before the click's IPC lands would look
// identical to a dead affordance.
async function pollUntil(label, deadlineMs, probe) {
  const deadline = Date.now() + deadlineMs
  while (Date.now() < deadline) {
    if (await probe()) return true
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

// The app's own record of the effect. Reading it here is what lets the press be
// synchronized on an explicit signal; the xtask caller re-reads the same family
// afterwards and owns the verdict.
function sawWidgetHidden(dataDir) {
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
            const rec = JSON.parse(line)
            return (
              rec.target === 'ui.layout.transition' &&
              rec.fields?.layout_mode_to === 'hidden' &&
              rec.fields?.layout_mode_from === 'compact-widget'
            )
          } catch {
            return false
          }
        })
    })
}

function startTauriDriver() {
  const cliJs = require.resolve('@crabnebula/tauri-driver/cli.js')
  tauriDriver = spawn(process.execPath, [cliJs, '--native-driver', MSEDGEDRIVER_PATH], {
    stdio: ['ignore', 'pipe', 'pipe'],
    env: {
      ...process.env,
      ANDROMEDA_PULSE_DATA_DIR: PULSE_DATA_DIR,
      ANDROMEDA_PULSE_LOG_LEVEL: 'debug',
    },
  })
  tauriDriver.stdout.on('data', (d) => process.stdout.write(`[tauri-driver] ${d}`))
  tauriDriver.stderr.on('data', (d) => process.stderr.write(`[tauri-driver] ${d}`))
}

// Four webview windows exist (compact-widget / main / findings / report) and the
// session lands on an arbitrary one. Only the two titlebar-bearing surfaces
// carry the close control; the borderless findings/report windows do not, so a
// driver that assumed the first handle would report the control missing.
// The compact widget is the primary surface (layout-templates §IA notes) and its
// close is the full app-to-tray path, so it is the canonical target.
async function findWidgetHandle(browser) {
  const handles = await browser.getWindowHandles()
  const surfaces = []
  for (const handle of handles) {
    await browser.switchToWindow(handle)
    const url = await browser.getUrl()
    const hasClose = await browser.$(CLOSE_SELECTOR).isExisting()
    surfaces.push({ handle, url, hasClose })
    console.log(`webview-drive: handle url=${url} hasClose=${hasClose}`)
  }
  const widget = surfaces.find((s) => s.hasClose && !s.url.includes('/traces'))
  return widget ?? surfaces.find((s) => s.hasClose) ?? null
}

async function main() {
  startTauriDriver()
  if (!(await pollUntil('tauri-driver listening', DRIVER_READY_TIMEOUT_MS, () => portAccepts(DRIVER_PORT)))) {
    console.error(`webview-drive: tauri-driver never bound 127.0.0.1:${DRIVER_PORT}`)
    return 4
  }

  const browser = await remote({
    hostname: '127.0.0.1',
    port: 4444,
    connectionRetryCount: 1,
    connectionRetryTimeout: SESSION_TIMEOUT_MS,
    logLevel: 'error',
    capabilities: { browserName: 'wry', 'tauri:options': { application: PULSE_BIN } },
  })

  try {
    const target = await findWidgetHandle(browser)
    if (!target) {
      console.error(`webview-drive: no window exposes ${CLOSE_SELECTOR}`)
      return 3
    }

    await browser.switchToWindow(target.handle)
    const control = await browser.$(CLOSE_SELECTOR)
    const accessibleName = await control.getAttribute('aria-label')
    console.log(`webview-drive: pressing "${accessibleName}" on ${target.url}`)
    await control.click()
    console.log('webview-drive: press dispatched')

    // Hold the session open until the app's transition record appears, or the
    // budget expires. Absence is a legitimate outcome (the RED arm), so this
    // decides only when teardown is safe — never the verdict.
    const observed = await pollUntil('widget→hidden transition', TRANSITION_TIMEOUT_MS, async () =>
      sawWidgetHidden(PULSE_DATA_DIR),
    )
    console.log(`webview-drive: transition observed by driver = ${observed}`)
    return 0
  } finally {
    await browser.deleteSession().catch(() => {})
  }
}

main()
  .then(async (code) => {
    await sleep(500)
    tauriDriver?.kill()
    process.exit(code)
  })
  .catch(async (err) => {
    console.error(`webview-drive: ${err?.message ?? err}`)
    await sleep(500)
    tauriDriver?.kill()
    process.exit(1)
  })
