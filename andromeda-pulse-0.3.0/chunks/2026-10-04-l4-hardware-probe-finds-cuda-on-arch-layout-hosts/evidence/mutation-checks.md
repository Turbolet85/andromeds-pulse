# Mutation checks — Step 6

One-shot controls at /implement, 2026-10-04, after Step 2's widening read `13 tests run: 13 passed`. Each mutation was
applied with the anchored Edit tool (an anchor miss errors loudly), its run read, then restored the same way. The
restored tree is the one the listed gate block judges.

## (a) Drop `usr/lib` from `CUDA_PROBE_DIRS`
`cargo nextest run -p interpretation --profile ci -E 'test(/cuda_probe_/)' --no-fail-fast` → exit 100,
`13 tests run: 9 passed, 4 failed`.
- RED: `case_5_usr_lib_so` · `case_6_usr_lib_soname` · `cuda_probe_follows_the_arch_symlink_chain` ·
  `cuda_probe_candidate_set_is_exactly_dirs_by_names`
- GREEN (as predicted): the multiarch and toolkit cases, the `usr/lib64` cases, empty-root, unprobed, dangling.

Matches the plan's prediction exactly.

## (b) Drop `libcuda.so.1` from `CUDA_PROBE_NAMES`
Same command → exit 100, `13 tests run: 8 passed, 5 failed`.
- RED: `case_2_multiarch_soname` · `case_4_toolkit_soname` · `case_6_usr_lib_soname` · `case_8_usr_lib64_soname` ·
  `cuda_probe_candidate_set_is_exactly_dirs_by_names`
- GREEN: every `libcuda.so` case, the symlink chain (its `libcuda.so` link still resolves), empty-root, unprobed,
  dangling.

Matches the plan's prediction (the four soname cases and the exact-set pin).

## (c) Revert the Linux arm to the pre-chunk inline two-path array
`cargo clippy -p interpretation --all-targets --all-features -- -D warnings` → exit 101:
- `error: constant CUDA_PROBE_DIRS is never used`
- `error: constant CUDA_PROBE_NAMES is never used`
- `error: function cuda_driver_present_under is never used`
- `error: could not compile interpretation (lib) due to 3 previous errors`

The arm-wiring guard HOLDS on Linux. The `cfg_attr(not(target_os = "linux"), allow(dead_code))` leaves the
non-test Linux lib build with no allowance, so a reverted arm fails the clippy gate. The check was scoped to
`-p interpretation`; the dead-code finding is local to that crate's lib target, so the workspace form the plan names
reaches the same error.
