### WCAG criteria mapping

- **Tier coverage:** Standard tier → WCAG 2.1 AA full (~50 SCs)
- **Compliance trigger override:** No Section 508 / EAA / EN 301 549 mandate in security plan (Minimal tier); Creator Brief rigor signals (WCAG color discipline, reduced-motion respect) justify Standard AA without escalation to Comprehensive.
- **Motion-sensitive trigger escalation:** ADD SC 2.3.3 Animation from Interactions (AAA) to Standard AA mapping. Lighthouse a11y audit (v12+) includes prefers-reduced-motion check; custom Playwright `page.emulateMedia({ reducedMotion: 'reduce' })` assertion validates reduced-motion override disables non-essential motion.
