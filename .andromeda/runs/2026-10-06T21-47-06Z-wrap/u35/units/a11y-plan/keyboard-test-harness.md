### Keyboard test harness

**Sequences per ARIA pattern:**
- **Button:** Enter / Space invokes default action
- **Link:** Enter activates
- **Dialog:** Escape closes; Tab cycles within trap
- **Tabs:** Arrow keys navigate (Left/Right); Home / End jump
- **Combobox:** Down opens listbox; Arrow navigates options; Enter selects; Escape closes
- **Menu:** Arrow navigates (Up/Down); Enter activates; Escape closes
- **Switch:** Space toggles state
- **Slider:** Arrow keys adjust value; Home / End set min/max

**Tooling:** **Playwright** 1.49.x `page.keyboard.press()` / `page.keyboard.type()` per surface. No Cypress (commercial) dependency; stick to Playwright reusing tests' E2E session.
