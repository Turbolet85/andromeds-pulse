
## 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts — the Linux CUDA probe candidate-set pins
**Section:** §4 Unit Test Strategy → What unit tests cover → interpretation crate
**Change:** the bullet gains the probe pins: 6 co-located `cuda_probe_` pins in `hardware.rs` (13 cases) over the private seam `cuda_driver_present_under(root)` — an 8-case rstest over `CUDA_PROBE_DIRS` × `CUDA_PROBE_NAMES` (4 dirs × `libcuda.so` · `libcuda.so.1`) each reading present; an empty root and the unprobed `libcudart.so` · `libcuda.so.2` · `opt/cuda/libcuda.so` reading absent; the Arch symlink chain present and a dangling link absent (`cfg(unix)`); an exact-set pin. `TempDir` fixtures, no host-dependent pin. RED 8/13 pre-chunk; mutation-checked (drop `usr/lib` → 4 RED, drop `libcuda.so.1` → 5 RED, revert the arm → clippy dead-code RED, the arm-wiring guard).
**Why:** the chunk added the seam and its pins; the bullet named only the prompt surface.
**Ref:** .andromeda/runs/2026-10-04T17-56-08Z-wrap/
