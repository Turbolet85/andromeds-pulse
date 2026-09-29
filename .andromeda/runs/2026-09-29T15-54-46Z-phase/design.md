# design extract

## No domain coverage
The chunk only changes CI and the dev loop: it splits `ci.yml` into parallel jobs, adjusts rust-cache keying, removes duplicate rebuilds, adds a WSL pre-push check and registers xtask smokes in arch. It renders no surface and touches no token, typography, motion, iconography or component pattern, and design-system.md names no CI gate. The a11y CI step it may move keeps its token-contrast verification (design-system §Self-Validation Protocol → 6. Contrast Test), which belongs to the a11y extractor.
