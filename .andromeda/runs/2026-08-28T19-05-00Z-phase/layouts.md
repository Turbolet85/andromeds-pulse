# layouts extract

## No domain coverage

Chunk scope §5 confines work to `crates/buffer` (consumer loop, `dispatch_batch` lock discipline, baseline-tap placement), a new `cargo xtask smoke:gap-resume` arm, and obs/test-harness contracts — no desktop-webview or desktop-native surface, region, component placement, focus order, responsive breakpoint, or modal is created or modified, and the nearest layout artifact (the full-dashboard footer `ConnectionStatusLine` ingest/buffer readout, per layout-templates §Component — Footer (read-only status bar) → Full dashboard) is a downstream consumer of `rows_ingested` whose layout the scope neither touches nor may touch.
