import js from "@eslint/js";
import tseslint from "typescript-eslint";
import jsxA11y from "eslint-plugin-jsx-a11y";
import reactPlugin from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";
import globals from "globals";

export default [
  {
    ignores: [
      "dist/**",
      "node_modules/**",
      "coverage/**",
      "playwright-report/**",
      "test-results/**",
      ".lighthouseci/**",
      "pa11y-results/**",
    ],
  },

  js.configs.recommended,
  ...tseslint.configs.recommended,

  {
    files: ["**/*.{ts,tsx,jsx}"],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    settings: {
      react: { version: "detect" },
    },
    plugins: {
      react: reactPlugin,
      "react-hooks": reactHooks,
      "jsx-a11y": jsxA11y,
    },
    rules: {
      ...reactPlugin.configs.flat.recommended.rules,
      "react/react-in-jsx-scope": "off",
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "warn",
      ...jsxA11y.flatConfigs.recommended.rules,
      "jsx-a11y/alt-text": ["error", { elements: ["img"], img: ["NextImage"] }],
      "jsx-a11y/anchor-is-valid": "off",
      // a11y-plan §9 pins these five at error explicitly rather than relying on
      // the spread above, so an upstream severity downgrade cannot silently
      // weaken the gate. They cannot live on the `--rule` CLI flag: that builds
      // a top-level config object where the jsx-a11y plugin is not in scope.
      "jsx-a11y/anchor-has-content": "error",
      "jsx-a11y/aria-props": "error",
      "jsx-a11y/aria-role": "error",
      "jsx-a11y/role-has-required-aria-props": "error",
    },
  },

  {
    files: [
      "scripts/**/*.{js,mjs}",
      "tests-a11y/**/*.{js,mjs}",
      "tests-e2e/**/*.{js,mjs}",
      "*.config.{js,mjs,ts}",
    ],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
];
