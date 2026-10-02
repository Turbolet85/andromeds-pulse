
## 2026-09-30-perf-instruments-measure-their-budgets — canvas fallback renders on every unavailable adapter result
**Section:** §Surface: desktop-webview → Component Patterns (WebGPU initialization)
**Change:** The fallback renders whenever the adapter request yields no usable device — `navigator.gpu` undefined, a null adapter, a rejected adapter request (caught since this chunk; before it the rejection escaped and nothing rendered), or a failed device request. Was: "if `navigator.gpu` is undefined" only.
**Why:** every `unavailable` result already rendered the shared fallback; this chunk added the rejected-request reason, and the line named only one trigger.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
