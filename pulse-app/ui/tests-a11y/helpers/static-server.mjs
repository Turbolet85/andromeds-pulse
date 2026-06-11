// Chunk #99 — shared static-server lifecycle for the a11y chain stages
// that audit URLs outside playwright (lighthouse, pa11y). Spawns the
// http-server BIN directly under the current node executable: killing an
// `npx` wrapper on Windows orphans the actual server child (observed this
// chunk — a stale no-fallback server hijacked port 4173 across runs), so
// the child here IS the server process and `kill()` is reliable. The
// `-P <self>?` proxy gives SPA fallback for the dashboard's
// browser-history routes (/settings, /diagnostics, ...).
import { spawn } from "node:child_process";
import { join } from "node:path";

export function startStaticServer(root, port) {
  return new Promise((resolve, reject) => {
    const bin = join(root, "node_modules", "http-server", "bin", "http-server");
    const child = spawn(
      process.execPath,
      [bin, "dist", "-p", String(port), "--silent", "-P", `http://localhost:${port}?`],
      { cwd: root, stdio: "ignore", shell: false },
    );
    const timer = setTimeout(() => {
      reject(new Error(`http-server on :${port} did not start within 10s`));
    }, 10_000);
    setTimeout(() => {
      clearTimeout(timer);
      resolve(child);
    }, 1_500);
    child.on("error", (err) => {
      clearTimeout(timer);
      reject(err);
    });
  });
}
