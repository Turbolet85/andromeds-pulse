# Red before green — the `cuda_probe_` pins against the base candidate set

Step 1, measured 2026-10-04 at /implement. The root-taking seam `cuda_driver_present_under` was exposed over the
CURRENT candidate set only: dirs `usr/lib/x86_64-linux-gnu` · `usr/local/cuda/lib64`, names `libcuda.so` (the base's
two literal paths, root-relative). The six pins of Step 3 were written in full and run before the widening:

`cargo nextest run -p interpretation --profile ci -E 'test(/cuda_probe_/)' --no-fail-fast` → exit 100,
`13 tests run: 5 passed, 8 failed`.

## Failed (8) — the property the chunk builds
- `cuda_probe_reads_present_for_each_candidate::case_5_usr_lib_so` — new location (the entry's own case)
- `cuda_probe_reads_present_for_each_candidate::case_6_usr_lib_soname` — new location
- `cuda_probe_reads_present_for_each_candidate::case_7_usr_lib64_so` — new location
- `cuda_probe_reads_present_for_each_candidate::case_8_usr_lib64_soname` — new location
- `cuda_probe_reads_present_for_each_candidate::case_2_multiarch_soname` — soname-only, pre-existing dir
- `cuda_probe_reads_present_for_each_candidate::case_4_toolkit_soname` — soname-only, pre-existing dir
- `cuda_probe_follows_the_arch_symlink_chain` — the dev host's layout lives under `usr/lib`
- `cuda_probe_candidate_set_is_exactly_dirs_by_names` — the base set is two dirs × one name

## Passed (5) — preserved by the base as well
- `cuda_probe_reads_present_for_each_candidate::case_1_multiarch_so` — a base path
- `cuda_probe_reads_present_for_each_candidate::case_3_toolkit_so` — a base path
- `cuda_probe_reads_absent_on_an_empty_root` — the safe default
- `cuda_probe_reads_absent_for_unprobed_names_and_dirs` — exact match, no prefix or search
- `cuda_probe_reads_absent_for_a_dangling_symlink` — `exists()` follows the link

The plan predicted the four new-location cases and the two soname-only cases RED, with empty-root and unprobed
GREEN. Both held; the symlink-chain and exact-set pins are RED too, as their subjects require the widened set.
After Step 2's widening the same run reads `13 tests run: 13 passed` (recorded in the gate trail).
