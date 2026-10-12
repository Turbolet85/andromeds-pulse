# Security validation — route draft

---

## Insert
- Between `Theme 0 checked` and `Token lifecycle by engine command`: **"Security posture re-based for a networked engine — tier, auth model, attack surface restated for commands, token, receiver, door, disk store before loopback leaves (P-087, P-116)"** (epoch: `Epoch 4`)
  Reason: per security-plan §Threat Model Summary, tier Minimal is justified by no network surface, no auth surface and no persistent store, and P-087, P-088 and P-091 each replace one of those. §Bootstrap phases marks `auth-scaffolding-baseline` SKIP and says auth is re-evaluated in the security master, never silently re-added via route — yet the draft's next chunk is its first auth chunk.
- Between `Theme 5 checked by the external harness` and `Real service watched for days`: **"Named service's personal data bounded — compliance triggers re-read on what the founder names; each kind scrubbed before storage or its keeping decided (P-101)"** (epoch: `Epoch 9`)
  Reason: per security-plan §Threat Model Summary → Compliance triggers and §Compliance Controls, "None" rests on telemetry staying on the user's own machine, and the scrubber catalog in §Security Anti-Patterns → Logging covers few personal-data kinds. The draft names the data inside the chunk that already sends and keeps it, so no boundary chunk precedes the first real send.

## Rewrite
- `Engine end-to-end gate reachable`: "a second host sends" → "a loopback sender sends"
  Reason: per security-plan §Security Anti-Patterns → API (the loopback-only bind is the authorization boundary), a second host recorded green in Epoch 1 needs a non-loopback receiver with no token and no encrypted channel, three epochs before `Network OTLP receiver behind the token`. `Theme 1 checked by the external harness` already pins the other-host path in this same gate.
- `Token lifecycle by engine command`: "never written to a log or a report" → "not recoverable from the node's stores; never written to a log or a report"
  Reason: per security-plan §Secret Management "What counts as secret" and §Data Protection → At rest, `Corpus encryption at rest retired` leaves unencrypted stores and no credential store, so "shown once" holds only if what the engine keeps cannot yield the token back.
- `One place on a node`: "operator-set locations for stores, log and pid" → "operator-set, owner-only locations for stores, log and pid"
  Reason: per security-plan §Data Protection → At rest (access control by OS user permissions per medium) and §Logging & Monitoring → Access controls, the account boundary is the only at-rest control left after `Corpus encryption at rest retired`, and for the week `Telemetry store on disk` keeps (§Security Anti-Patterns → Data Protection keeps the store's own encryption off). Both of those lines are at the word cap, and this resolver places both stores.
- `Engine delivered to a node`: "one Linux binary" → "one auditable Linux binary"
  Reason: per security-plan §Dependency Security → Supply chain integrity, `cargo-auditable` on the shipped artifact is what lets a later advisory be matched to a release. Its production build lives in `release.yml`, which leaves with `Desktop distribution retired`, while `ci.yml` keeps only a build smoke.
