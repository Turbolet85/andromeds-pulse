// WebGPU-unavailable fallback message component for chunk #28 canvas substrate.
// Per a11y plan §1 + §7 Live regions: role="alert" + aria-live="assertive" so
// NVDA/VoiceOver/Orca announce the failure on render. Per design plan §Loading
// / Empty States: text in --color-text-tertiary, font-body, no color-alone
// state signaling. Canonical text "WebGPU not supported in this browser"
// chosen over layout-templates.md "in this context" wording per phase-25
// combined.md Cross-domain rot warning #1 resolution.

export function Fallback() {
  return (
    <div
      role="alert"
      aria-live="assertive"
      style={{
        color: "var(--color-text-tertiary)",
        fontFamily: "var(--font-body)",
        fontSize: "14px",
        padding: "var(--spacing-md)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        height: "100%",
        textAlign: "center",
      }}
    >
      WebGPU not supported in this browser
    </div>
  );
}
