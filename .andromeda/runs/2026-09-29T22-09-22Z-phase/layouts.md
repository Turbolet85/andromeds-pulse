# layouts extract

## No domain coverage
The chunk is a backend precision fix to the scrubber's `credit_card` arm (and possibly `ssn`) plus its tests. It creates or modifies no surface, region, component placement, focus order, modal or breakpoint. The only downstream webview touch is a text VALUE, not layout: the workspace string shown in the Report window's six-section `Report` (per layout-templates §Surface: desktop-webview → Primary screens, "Report window") may render a real basename where it used to show a redaction placeholder. Whether that surface renders the workspace key at all is a question for research, not for this plan.
