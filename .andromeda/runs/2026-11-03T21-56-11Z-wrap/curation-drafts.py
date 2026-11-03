import shutil, sys
R = ".andromeda/runs/2026-11-03T21-56-11Z-wrap"
BIND = ("**[corrected 2026-11-03: the `mcp-server` feature, its `mcp.*` router and the `andromeda-pulse-mcp` sidecar left at "
        "chunk `2026-11-03-stdio-sidecar-retired` — every regenerating op now writes the one bindings shape, so the no-mcp "
        "shape, the `--features mcp-server` regen and the `grep -c '\"mcp\":'` probe no longer exist; the order that stands is "
        "test-plan's per-chunk gate discipline (`capability-drift` before the workspace run, the featureless "
        "`emit_taurpc_bindings` regen last, `git diff --quiet {chunk-base} -- pulse-app/ui/src/bindings/index.ts` closing), "
        "green at chunk `2026-11-03-advisories-since-2026-10-12-closed`]**")
SIB = ("**[corrected 2026-11-03: its example left — `andromeda-pulse-mcp` and the `mcp-server` feature were retired at chunk "
       "`2026-11-03-stdio-sidecar-retired`; the rule (nextest does not build a sibling crate's binary) stands for any test "
       "that spawns one]**")
FEAT = ("**[corrected 2026-11-03: the `mcp-server` feature named here left at chunk `2026-11-03-stdio-sidecar-retired` "
        "(`pulse-app/Cargo.toml [features]` holds `default` and `otap-ingest`); the mechanism stands for any second feature "
        "graph]**")
ADV = ("**[corrected 2026-11-03: `cargo deny check advisories` exits 0 (`advisories ok`) since chunk "
       "`2026-11-03-advisories-since-2026-10-12-closed` pruned the withdrawn GTK3 ignores and closed the owned set by "
       "upgrade, so it is no longer designed-red; the two-invocation split stays the safe form, since a new owned finding "
       "reddens `advisories` alone]**")
N1 = ("- 2026-11-03: Before writing a `deny.toml [bans] skip` for a duplicate a bump surfaced, dry-run "
      "`cargo update -p {holder} --dry-run` for each holder of the OLD copy: an in-range update of one holder can drop the "
      "old copy whole, and a skip's «not source-fixable» clause is a measurement, never an assumption.\n")
N2 = ("- 2026-11-03: A gate probe that greps a source tree holding a committed GENERATED artifact (the TauRPC bindings) "
      "reads that artifact's committed copy, stale until the regenerating entry runs — order such a probe AFTER the regen, "
      "or keep the artifact out of its scope.\n")
N3 = ("## 2026-11-03 — A `git diff` scope guard cannot read a recorded companion\n\n"
      "A plan's scope-guard entry written as `git diff --name-only {base} -- {dirs}`, with research's file list as its "
      "exclusions, reads every changed file outside that list as out of scope — including one the operator recorded as a "
      "companion in the chunk's scope record, which the guard never reads. `gate.py scope` reads the scope record and is "
      "the authoritative scope read: a plan either excludes recorded companions from its guard or lets `gate.py scope` "
      "stand as the scope gate. Where it applies: any chunk whose scope record gains a companion after research listed "
      "the files.\n\n---\n\n")

def draft(path):
    d = f"{R}/draft-{path.replace('/', '-')}"
    shutil.copyfile(path, d)
    return d

def tag(lines, n, text, expect):
    i = n - 1
    assert expect in lines[i], (n, expect)
    body = lines[i].rstrip("\n")
    lines[i] = body + " " + text + "\n"

# rules/security.md
d = draft(".claude/rules/security.md")
L = open(d, encoding="utf-8", newline="").readlines()
tag(L, 132, BIND, "2026-05-19: EXTENSION to the 2026-05-13 bindings.ts regen entry")
tag(L, 176, BIND, "2026-06-12 (session 186): CRITICAL EXTENSION")
tag(L, 180, ADV, "2026-08-17: A COMBINED `cargo deny check")
if not L[-1].endswith("\n"): L[-1] += "\n"
L.append(N1)
open(d, "w", encoding="utf-8", newline="").writelines(L); print("ok", d)

# rules/testing.md
d = draft(".claude/rules/testing.md")
L = open(d, encoding="utf-8", newline="").readlines()
tag(L, 169, BIND, "2026-05-13: `pulse-app/ui/src/bindings/index.ts` regenerates")
tag(L, 179, BIND, "2026-05-17: The 2026-05-13 entry above")
tag(L, 187, SIB, "2026-05-22 (session 119)")
tag(L, 214, BIND, "2026-05-25 (session 150)")
tag(L, 238, FEAT, "2026-06-28:")
tag(L, 244, FEAT, "2026-07-02: DISK-HYGIENE")
tag(L, 258, BIND, "2026-08-15: EXTENSION to the bindings.ts regen family")
if not L[-1].endswith("\n"): L[-1] += "\n"
L.append(N2)
open(d, "w", encoding="utf-8", newline="").writelines(L); print("ok", d)

# docs/session-learnings.md
d = draft(".claude/docs/session-learnings.md")
L = open(d, encoding="utf-8", newline="").readlines()
tag(L, 633, BIND, "**Regenerate `bindings.ts` after:**")
t = "".join(L)
head = "# Session Learnings\n\n\n"
assert t.startswith(head)
t = head + N3 + t[len(head):]
open(d, "w", encoding="utf-8", newline="").write(t); print("ok", d)
