# Tauri Updater Minisign Keypair Rotation Runbook

**Scope:** procedure for rotating the Tauri updater Minisign Ed25519 keypair embedded in `pulse-app/tauri.conf.json` `plugins.updater.pubkey` and consumed by `release.yml` (route#46) for signing `latest.json`.

**Audience:** project maintainer (operator). This runbook is **not** executed by `/andromeda-implement` — it is operator-driven before / during release events.

**Status of current keypair (2026-05-03):** generated locally via `minisign -G -W -f -s ~/.tauri/andromeda-pulse.key -p ~/.tauri/andromeda-pulse.key.pub` for ACTIVE-scope local-dogfooding work. The key is stored **without password** in `~/.tauri/`. This is acceptable for solo dev iteration but **MUST be regenerated with a password and uploaded to Azure Key Vault Premium SKU before the v0.1.0 public release** (DEFERRED scope per route.md Decisions Log 2026-05-03 entry).

---

## When to rotate

1. **Routine rotation (planned)** — when the current keypair is suspected of approaching end-of-life (e.g., Apple Developer ID cycle, EV cert renewal cycle, or annual rotation policy).
2. **Emergency rotation (unplanned)** — when the private key is suspected of compromise: lost laptop, accidental disclosure, suspected supply-chain incident, or audit finding.
3. **Pre-release migration (one-time)** — when transitioning from local-dogfooding keypair (no password, in `~/.tauri/`) to production HSM-backed keypair (Azure Key Vault Premium SKU, with password).

---

## Phase 1 — Generate new keypair

### Local generation (dogfooding / pre-release)

```powershell
# Standalone minisign (current ACTIVE scope)
minisign -G -f -s $HOME\.tauri\andromeda-pulse.key.new -p $HOME\.tauri\andromeda-pulse.key.new.pub
# minisign prompts for password twice; choose strong password and store in password manager
```

OR, when `tauri-cli` is available (post-MSVC-toolchain switch or post-`rustup component refresh`):

```powershell
tauri signer generate -w $HOME\.tauri\andromeda-pulse.key.new
# tauri-cli prompts for password; store in password manager
```

The two tools produce **interoperable** Minisign Ed25519 keypairs — either output is verifiable by `tauri-plugin-updater` 2.x in the user binary.

### Capture artifacts

After generation, capture three things:

1. **New public key (base64 string starting with `RW`)**: read line 2 of `~/.tauri/andromeda-pulse.key.new.pub`. This goes into `tauri.conf.json`.
2. **New private key file path**: `~/.tauri/andromeda-pulse.key.new`. This must NEVER appear in the repo or any build artifact.
3. **Password (if set)**: store in password manager. Required to decrypt the private key during signing.

---

## Phase 2 — Upload to Azure Key Vault (DEFERRED scope, pre-v0.1.0)

This phase becomes load-bearing when the project is ready to ship public signed releases. Until then, the local keypair in `~/.tauri/` is sufficient.

### Prerequisites (DEFERRED scope items D1-D3 from `.andromeda/phases/phase-2/plan.md`)

- Azure subscription with Key Vault Premium SKU instance provisioned (HSM-backed)
- DigiCert or GlobalSign Windows EV certificate enrolled (HSM-RSA only)
- Apple Developer ID Application certificate enrolled ($99/year)
- GitHub OIDC federation trust between `turbolet85/andromeda-pulse` repo and Azure managed identity

### Upload procedure

1. Authenticate to Azure via OIDC federation:
   ```powershell
   az login --identity
   ```
2. Upload the Minisign private key + password as Key Vault secrets:
   ```powershell
   $vaultName = "andromeda-pulse-prod-kv"   # must exist (Premium SKU)
   az keyvault secret set --vault-name $vaultName --name "tauri-signing-private-key" --file $HOME\.tauri\andromeda-pulse.key.new
   az keyvault secret set --vault-name $vaultName --name "tauri-signing-private-key-password" --value "<password from Phase 1>"
   ```
3. Configure GitHub Environment `production-release` to consume the Key Vault secrets via `azure/login` action + Azure SDK secret-fetch step in `release.yml` (lands at route#46).
4. Confirm `release.yml` workflow can read the secrets via a dry-run signing step.

---

## Phase 3 — Transitional release (multi-pubkey window)

This phase prevents existing users from being orphaned by the rotation. Users running version N must continue receiving signed updates while the rotation propagates.

### Strategy A — single-pubkey replace (only safe if NO existing user base)

If the project has no released version (pre-v0.1.0), the rotation is a hard cutover:

1. Replace `plugins.updater.pubkey` in `pulse-app/tauri.conf.json` with the new pubkey base64.
2. Build a new release.
3. Old keypair is retired; no transition needed.

**This is the strategy applicable today (pre-v0.1.0).**

### Strategy B — two-release transitional window (post-v0.1.0)

For projects with an existing user base, rotation must span two releases:

1. **Release N+1 (transitional):** updater config carries BOTH old and new pubkeys. `tauri-plugin-updater` 2.x supports multi-pubkey verification — accept signature if either pubkey verifies.
   ```json
   {
     "plugins": {
       "updater": {
         "endpoints": ["..."],
         "pubkey": "<NEW pubkey>",
         "pubkey_alt": "<OLD pubkey>",
         "dialog": false
       }
     }
   }
   ```
   _Note: as of Tauri 2.11, `pubkey_alt` field naming may differ — check current Tauri docs at release time. The intent is dual-pubkey acceptance; the schema enforces it._

2. Sign release N+1 with the OLD private key (so users on version N can verify and update to N+1).
3. **Release N+2:** updater config carries ONLY the new pubkey. Sign with new private key. Users on N+1 verified the new pubkey was already trusted in their installed config; transition is complete.

4. **Advisory communication** — publish a security advisory to `https://github.com/turbolet85/andromeda-pulse/security/advisories` explaining the rotation, approximate timeline, and any user action required. See template below.

### Advisory communication template

```markdown
# Updater Signing Key Rotation — andromeda-pulse vX.Y.Z

**TL;DR:** Tauri updater Minisign Ed25519 signing keypair rotated. Existing
users on vX.Y.(Z-1) and earlier will receive an automatic update to vX.Y.Z
signed with the old key; subsequent updates are signed with the new key.
**No user action is required for routine updates.**

**Context:** {planned-rotation | emergency-rotation-due-to-{compromise|loss}}.
The previous Minisign Ed25519 public key, `<OLD pubkey>`, is RETIRED as of
release vX.Y.Z. Going forward, only the new public key,
`<NEW pubkey>`, is trusted.

**Verification:** users can verify the new pubkey by checking
`tauri.conf.json` `plugins.updater.pubkey` in the source repo or the
release notes for vX.Y.Z.

**If you are stuck on a pre-rotation version,** download vX.Y.Z directly
from `https://github.com/turbolet85/andromeda-pulse/releases/tag/vX.Y.Z`
and install manually.

**Reporting suspected fraudulent updates:** if you receive an update
purporting to come from andromeda-pulse but signed with a key not listed
above, do NOT install it. Report to security@<contact>.
```

---

## Phase 4 — Final rotation release

After the transitional release N+1 has propagated to ≥95% of users (estimated 1-4 weeks for the typical update tail):

1. Cut release N+2 with `tauri.conf.json` carrying ONLY the new pubkey.
2. Sign with new private key.
3. Decommission the old keypair:
   - Delete the OLD private key from Azure Key Vault.
   - Update GitHub Environment secrets — remove old key references.
   - Archive the OLD `tauri.conf.json` `pubkey` value with rotation metadata in `docs/runbooks/key-rotation-history.md` (create on first rotation).
4. Close any related security advisory as resolved.

---

## Phase 5 — Verify post-rotation

After the rotation is complete:

1. **Local verification:**
   ```powershell
   # Build a fresh local pulse-app and confirm updater accepts the new pubkey
   cargo build -p pulse-app
   # Check tauri.conf.json carries ONLY new pubkey
   grep -E '"pubkey"' pulse-app\tauri.conf.json   # should show new key only
   ```

2. **Repo hygiene:**
   ```powershell
   # Confirm no private key material in working tree or git history
   git log --all -p -- '**/*.key' '**/*.p12' '**/*.pem' '**/*.pfx'   # expect empty
   git ls-files | xargs grep -E '(BEGIN PRIVATE KEY|TAURI_SIGNING_PRIVATE_KEY=)'   # expect empty
   ```

3. **Updater end-to-end (route#46+ is implemented):**
   - Trigger a synthetic release via `release.yml` workflow_dispatch.
   - Confirm `latest.json` signature verifies against the new pubkey.
   - On a second machine, run the existing pulse-app binary and confirm it accepts the new release.

---

## Anti-patterns to avoid

- NEVER ship the Minisign **private** key in any commit, GitHub Releases asset, or build artifact. Only the public key belongs in `tauri.conf.json`.
- NEVER override `tauri-plugin-updater` 2.x Minisign Ed25519 signature verification in code or in CI. The verification path is load-bearing in-transit integrity.
- NEVER reuse a compromised keypair "until the next planned rotation." Compromise → emergency rotation → immediately.
- NEVER store the Azure Key Vault service principal as a long-lived GitHub Actions secret (use OIDC federation instead).
- NEVER skip the transitional Phase 3 if there is an existing user base — this orphans users.
- NEVER skip the advisory communication for an emergency rotation — users have a right to know.

---

## References

- Security plan: `.andromeda/security-plan.md` §Code-signing key custody
- Architecture: `.andromeda/architecture.md` §Established Decisions [Code Signing], §Occupied Resources Updater channel
- Phase plan: `.andromeda/phases/phase-2/plan.md` ACTIVE / DEFERRED scope split
- Route Decisions Log: `.andromeda/route.md` 2026-05-03 entry "Chunk #3 scope split"
- Tauri 2.x Updater docs: https://v2.tauri.app/plugin/updater/
- Minisign spec: https://jedisct1.github.io/minisign/
