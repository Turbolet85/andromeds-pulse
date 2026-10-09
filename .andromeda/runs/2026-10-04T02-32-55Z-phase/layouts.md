# layouts extract

## No domain coverage
The chunk only changes the backend corpus-key creation path (`crates/corpus/src/keychain.rs` cross-process locked create-or-read). Its scope §Boundaries rules out any webview, tray, notification or other surface change, and no layout-templates.md surface (desktop-webview or desktop-native) covers corpus, keychain or credential state.
