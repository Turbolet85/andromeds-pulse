# Vendored: llama.cpp `json_schema_to_grammar.py`

| | |
|---|---|
| Source repository | `ggml-org/llama.cpp` |
| Tag | `b9305` |
| Commit | `63248fc3e33e6f3b579dce6a743fd6ce8939af9c` |
| Upstream path | `examples/json_schema_to_grammar.py` |
| sha256 | `677553718afb2bc2a63182fa812240c8436981f3fe43381e2e48a27b355352a8` |
| License | MIT, `LICENSE` beside this file (Copyright (c) 2023-2026 The ggml authors) |

The file is byte-identical to upstream. It is stdlib-only Python 3.

## What uses it

`pulse-app/tests/unit_l4_grammar.rs` runs it on the shipped L4 schema
(`crates/interpretation/src/schema.json`) and asserts the output equals the committed grammar
`pulse-app/src/l4-output.gbnf`, byte for byte. A schema edit without a regenerated grammar fails that test.

The shipped binary never runs it. The product embeds the committed grammar and passes it to the pinned
b9305 `llama-cli` through `--grammar-file`.

## Regenerating the grammar

```sh
python3 pulse-app/vendor/llama-cpp/json_schema_to_grammar.py crates/interpretation/src/schema.json > pulse-app/src/l4-output.gbnf
```

Bump this file only together with the pinned llama.cpp tag, and record the new commit and sha256 here.
