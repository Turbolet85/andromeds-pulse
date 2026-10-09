# Acquisition (plan Step 6)

**Licence read at download** (Hugging Face model API `https://huggingface.co/api/models/unsloth/{repo}`,
`cardData.license`, read 2026-10-05T06:45:26Z, immediately before the downloads started):

| repo | card licence | repo sha | lastModified |
|---|---|---|---|
| `unsloth/Qwen3.5-4B-GGUF` | `apache-2.0` | `e87f176479d0855a907a41277aca2f8ee7a09523` | 2026-03-02T14:08:17Z |
| `unsloth/Qwen3.5-2B-GGUF` | `apache-2.0` | `f6d5376be1edb4d416d56da11e5397a961aca8ae` | 2026-03-02T14:07:35Z |
| `unsloth/gemma-4-E4B-it-GGUF` | `apache-2.0` | `bfc15c382204943c3a8fff0c750b94ae2364d7a3` | 2026-07-17T12:49:21Z |
| `unsloth/gemma-4-E2B-it-GGUF` | `apache-2.0` | `0314792d7f1f7e229411f620751375812bb9faf2` | 2026-07-17T12:46:32Z |

Every card still reads `apache-2.0`, and each repo sha equals the one inputs#I3 recorded at P3, so nothing changed
between the phase read and the download. Nothing was surfaced.

**Download:** the four Q4_K_M files only (no mmproj; the L4 path is text-only), from
`https://huggingface.co/{repo}/resolve/main/{file}` into the gitignored model dir (`AI-Model`, repo root), written as `{file}.part` and
renamed on success. Done 2026-10-05, 06:45Z–07:03Z.

**Integrity (plan entry 17, `leg = 'operator'`, driven once by hand at 2026-10-05T07:03:27Z):**
`cd AI-Model && sha256sum -c ../andromeda-pulse-0.3.0/chunks/{marker}/evidence/candidates.sha256` → exit 0.

```
Qwen3.5-4B-Q4_K_M.gguf: OK
Qwen3.5-2B-Q4_K_M.gguf: OK
gemma-4-E4B-it-Q4_K_M.gguf: OK
gemma-4-E2B-it-Q4_K_M.gguf: OK
```

All four expect atoms hold (`exit 0` plus the four `: OK` lines), so entry 17 reads green.

## The sixth model — NVIDIA Nemotron 3 Nano 4B (`preregistration-addendum-4.md`, inputs#I7)

- Card read (HF API, 2026-10-05, before the download): `unsloth/NVIDIA-Nemotron-3-Nano-4B-GGUF`, `license: other`,
  `license_name: nvidia-nemotron-open-model-license` (NOT Apache-2.0), repo sha
  `8e81be55c5aa3d63bb82b6ceec62d50805d9e1bb`, lastModified 2026-03-17T00:16:31Z;
  `NVIDIA-Nemotron-3-Nano-4B-Q4_K_M.gguf` 2,900,295,712 B, LFS sha256
  `e515c9ceb10ae503db22a201fade92167f757510d1247a65987f8b6ae9e296e7` (written to `nemotron.sha256`).
- Downloaded 2026-10-05 08:43:42Z–08:48:01Z, after addendum 4 (08:43:36Z) and after the in-flight confirmations
  had finished, into the gitignored model dir (`AI-Model`, repo root), as `{file}.part` then renamed.
- `sha256sum -c ../…/evidence/nemotron.sha256` → `NVIDIA-Nemotron-3-Nano-4B-Q4_K_M.gguf: OK`, exit 0.
