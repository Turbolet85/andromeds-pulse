
## 2026-09-30-dual-license — cargo-deny license-checks the workspace's own crates
**Section:** §Dependency Security → CI integration
**Change:** The `cargo deny check bans licenses sources` bullet now states that the `licenses` check covers the workspace's OWN crates as well as its dependencies: `deny.toml` sets no `private` key, so cargo-deny's default (`private.ignore = false`) keeps every member in the checked set — as measured at this chunk (a scratch config without `"MIT"` in `[licenses] allow` rejected all 16 members, cargo-deny 0.20.2). The project's own expression `MIT OR Apache-2.0` passes because both `MIT` and `Apache-2.0` sit in `[licenses] allow`.
**Why:** the project moved to `MIT OR Apache-2.0`; the sibling project had to REMOVE a `private = { ignore = true }` exemption to get the same coverage, which this project never set — so the fact is recorded as measured rather than inferred, and `deny.toml` stayed untouched.
**Ref:** .andromeda/runs/2026-09-30T07-44-36Z-wrap/
