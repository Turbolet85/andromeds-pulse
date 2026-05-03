# Runbook — `production-release` GitHub Environment

**Status:** scaffolded at chunk #6; secret population DEFERRED per chunk #3 scope split (route Decisions Log 2026-05-03).
**Authoritative source:** security-plan.md §Secret Management GitHub Environment scoping + arch §Cross-cutting Patterns "Config management".

## Purpose

The `production-release` GitHub Environment is the trust boundary for code-signing secrets and trusted-publisher credentials. ANY workflow step that needs:

- Azure Key Vault federation creds (Windows EV signing)
- Apple Developer ID creds (macOS notarization)
- Tauri updater Minisign **private** key (release artifact signing)
- Homebrew tap push token + Scoop bucket push token (distribution channels)

…MUST request `environment: production-release` in the workflow YAML, AND that environment MUST require manual approval from a designated reviewer. PR-time CI workflows (`ci.yml`, `secret-scan.yml`) MUST NOT request the environment, so signing secrets cannot enter PR check execution.

## ACTIVE scope (chunk #6 — landed by /implement)

The Environment is **created** as a named container with manual approval gate. No paid components are populated; the Environment is empty until the DEFERRED scope below is executed pre-v0.1.0.

### Create the Environment

Run from a maintainer account with admin permissions on the repository:

```bash
# 1. Create the environment with empty wait_timer (no auto-deploy delay)
gh api -X PUT repos/:owner/:repo/environments/production-release \
  -f wait_timer=0

# 2. Restrict deployment to the protected `main` branch only
gh api -X PUT repos/:owner/:repo/environments/production-release \
  -f 'deployment_branch_policy[protected_branches]=true' \
  -f 'deployment_branch_policy[custom_branch_policies]=false'

# 3. Add at least one required reviewer (manual approval gate). Replace USER_ID
#    with the numeric GitHub user ID of the designated approver — get it via:
#    `gh api users/<login> --jq .id`
REVIEWER_ID=$(gh api users/turbolet85 --jq .id)
gh api -X PUT repos/:owner/:repo/environments/production-release \
  -F "reviewers[][type]=User" \
  -F "reviewers[][id]=$REVIEWER_ID"
```

The three commands are idempotent — re-running them on an existing Environment is a no-op.

### Verify

```bash
gh api repos/:owner/:repo/environments/production-release \
  --jq '{name, deployment_branch_policy, protection_rules: [.protection_rules[] | {type, reviewers: (.reviewers // [] | length)}]}'
```

Expected output (paraphrased):

```json
{
  "name": "production-release",
  "deployment_branch_policy": {
    "protected_branches": true,
    "custom_branch_policies": false
  },
  "protection_rules": [
    { "type": "required_reviewers", "reviewers": 1 }
  ]
}
```

### Workflow reference shape

Once `release.yml` lands (route#46), it references the Environment per-job:

```yaml
jobs:
  publish:
    name: build + sign + notarize + publish
    runs-on: ubuntu-22.04
    environment: production-release   # <-- gates this job behind manual approval
    steps:
      - uses: step-security/harden-runner@<SHA> # vX.Y.Z
        with: { egress-policy: audit }
      # … signing + notarization steps reading $AZURE_*, $APPLE_*, $MINISIGN_PRIVATE_KEY, etc.
```

`ci.yml` and `secret-scan.yml` (chunk #5 + #6) do NOT include `environment: production-release` — they run on every PR and signing secrets must not enter that path.

## DEFERRED scope (pre-v0.1.0 release blockers — chunk #3 split)

These items are NOT executed by /andromeda-implement at chunk #6. They are the maintainer's manual responsibility before tagging the first signed release.

### 1. Azure Key Vault Premium SKU + EV signing certificate

- Provision Azure Key Vault Premium SKU (HSM-backed) in the maintainer's Azure subscription.
- Order an EV code-signing certificate from DigiCert or GlobalSign (HSM-RSA only). Annual cost ~$300-600.
- Import the EV certificate into Key Vault. Private key NEVER leaves the HSM.
- Configure Azure AD app registration + federated identity credentials for GitHub OIDC. NEVER store an Azure service principal secret in GitHub Actions secrets — OIDC federation only.
- Add the federation creds to the `production-release` Environment as secrets:
  - `AZURE_TENANT_ID`
  - `AZURE_CLIENT_ID`
  - `AZURE_SUBSCRIPTION_ID`
  - `AZURE_KEY_VAULT_NAME`
  - `AZURE_KEY_VAULT_CERT_NAME`

Reference: security-plan.md §Code Signing Windows EV. Tracked: pre-v0.1.0 release blocker.

### 2. Apple Developer ID enrollment

- Enroll the maintainer's account in the Apple Developer Program ($99/yr).
- Generate a Developer ID Application certificate + Developer ID Installer certificate via Apple Developer portal.
- Export the certificates as `.p12` (NEVER commit — `.gitignore` covers `*.p12`); base64-encode for storage as a GitHub Environment secret.
- Generate an App Store Connect API key for `notarytool`.
- Add to the `production-release` Environment as secrets:
  - `APPLE_CERTIFICATE_P12_BASE64`
  - `APPLE_CERTIFICATE_PASSWORD`
  - `APPLE_DEVELOPER_ID_APPLICATION`
  - `APPLE_DEVELOPER_ID_INSTALLER`
  - `APPLE_NOTARY_KEY_ID`
  - `APPLE_NOTARY_KEY_BASE64`
  - `APPLE_NOTARY_ISSUER_ID`

Reference: security-plan.md §Code Signing macOS notarization. Tracked: pre-v0.1.0 release blocker.

### 3. Tauri updater Minisign keypair

- The local Minisign private key generated by chunk #3 (`docs/runbooks/updater-key-rotation.md`) provides the artifact signing seed. The PUBLIC key is baked into `tauri.conf.json` at build time (already wired by chunk #3).
- Add the PRIVATE key to the `production-release` Environment as a secret. NEVER commit the `.key` file (`.gitignore` covers `*.key`):
  - `MINISIGN_PRIVATE_KEY` (base64-encoded contents of the `.key` file)
  - `MINISIGN_PRIVATE_KEY_PASSWORD` (passphrase set at chunk #3 keygen)

Reference: chunk #3 ACTIVE scope (`docs/runbooks/updater-key-rotation.md`) + security-plan.md §Code Signing Tauri updater. Tracked: pre-v0.1.0 release blocker.

### 4. Distribution channel push tokens

- Create a GitHub Personal Access Token (fine-grained, single-purpose) with `contents: write` on the Homebrew tap repository (e.g., `turbolet85/homebrew-andromeda-pulse`).
- Create a GitHub Personal Access Token with `contents: write` on the Scoop bucket repository (e.g., `turbolet85/scoop-andromeda-pulse`).
- Add to the `production-release` Environment as secrets:
  - `HOMEBREW_TAP_PUSH_TOKEN`
  - `SCOOP_BUCKET_PUSH_TOKEN`

Reference: arch §Distribution Channels + route#46 release pipeline. Tracked: pre-v0.1.0 release blocker (downgradeable to v0.2.0 per route Decisions Log if Homebrew/Scoop setup blocks ship — see arch §Established Decisions [Distribution Channels]).

## Rotation

- Azure Key Vault EV cert: rotate annually before expiration (~$400-600 per renewal).
- Apple Developer ID: rotate annually with Apple Developer Program renewal.
- Tauri updater Minisign keypair: rotate per `docs/runbooks/updater-key-rotation.md` (chunk #3 ACTIVE scope) — quarterly recommended.
- GitHub PATs (Homebrew + Scoop): rotate per organization PAT-rotation policy.

After rotation: update the corresponding secret in the `production-release` Environment via `gh secret set --env production-release SECRET_NAME` or GitHub UI. Workflow YAML does NOT change.

## Verification (PR-time, no secret access)

The chunk #6 acceptance criterion "GitHub Environment `production-release` exists with manual approval gate" is satisfied entirely by the ACTIVE scope above (Environment creation + reviewer rule). DEFERRED scope (paid components) does NOT block chunk #6 — the workflow files (`ci.yml`, `secret-scan.yml`) reference no production-release secrets at PR time, and the Environment serves as a forward-declared trust boundary for `release.yml` (chunk #46).

```bash
# AC verification: Environment exists with required_reviewers protection rule
gh api repos/:owner/:repo/environments/production-release \
  --jq '.protection_rules[] | select(.type == "required_reviewers")'
```

Non-empty output confirms the AC.
