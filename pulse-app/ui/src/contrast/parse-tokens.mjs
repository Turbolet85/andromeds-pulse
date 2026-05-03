const TOKEN_DECLARATION = /(--[a-z][a-z0-9-]*)\s*:\s*(#[0-9a-fA-F]{3,8})\s*;/g;

export function parseTokens(css) {
  const tokens = new Map();
  for (const match of css.matchAll(TOKEN_DECLARATION)) {
    const [, name, hex] = match;
    tokens.set(name, hex.toUpperCase());
  }
  return tokens;
}
