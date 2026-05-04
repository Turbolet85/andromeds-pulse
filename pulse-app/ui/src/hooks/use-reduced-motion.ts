// Single canonical re-export of motion/react `useReducedMotion` so all three
// surfaces (compact widget, full dashboard, tray Halo overlay) consume one
// import path per layout-templates.md three-surface coherence + a11y-plan §3
// `motion-tokens-respect-install` bootstrap phase.
export { useReducedMotion } from "motion/react";
