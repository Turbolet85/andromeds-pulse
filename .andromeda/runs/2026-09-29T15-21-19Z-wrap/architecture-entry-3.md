
## 2026-09-29-p-025-hue-shift-observable-made-gradable — harness:status derives liveness from the pid
**Section:** §Occupied Resources → xtask CLI surfaces → `cargo xtask harness:status` · §Occupied Resources → Filesystem locations → `run/andromeda-pulse.pid`
**Change:** Was "derived from the pidfile plus the log family's mtime"; now from the pidfile, the liveness of the pid it holds, and the log family's mtime — a dead pid is `not-running` whatever the log says (`ps -o stat=` on Unix, a zombie counts as dead; `tasklist` on Windows). Before it, a crashed app read `running-healthy` for up to 60 s. The verdict JSON and its four arms are unchanged.
**Why:** measured on a CI boot smoke whose app panicked in 12 ms and still read healthy; fixed in-chunk on a founder ruling.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
