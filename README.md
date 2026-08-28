# Dioxus Common Abilities

Small, reusable Dioxus capabilities extracted from real applications.

This repository keeps each runtime capability in its own crate. Candidates are
promoted only after their shared invariant, consumer boundary, and validation
gate are documented. Product models, styling, routing, and command meaning stay
in the consuming application.

## Available crates

| Crate | Dioxus baseline | Purpose |
|---|---:|---|
| [`dioxus-input`](crates/dioxus-input) | `0.7.9` | Correct controlled-input behavior during CJK IME composition |
| [`dioxus-backdrop-dismiss`](crates/dioxus-backdrop-dismiss) | `0.7.9` adapter | Same-pointer backdrop dismissal without owning modal rendering |
| [`dioxus-focus-scope`](crates/dioxus-focus-scope) | `0.7.9` | Nested focus containment and safe opener restoration without owning modal policy |

`dioxus-input` provides a non-reactive `ImeGuard` that:

- blocks controlled-value writes during composition;
- recovers the final committed fragment from `compositionend` for WebView2;
- suppresses local keyboard commands using both guard and event state.

The consumer still decides whether committed text replaces, appends, or enters
a parser, and continues to own blur, focus, validation, and global shortcuts.

`dioxus-backdrop-dismiss` provides renderer-neutral pointer state. It prevents
press-inside/release-outside dismissal and handles content release,
`pointercancel` and simultaneous pointer IDs without owning modal markup, focus,
Escape policy or styling.

`dioxus-focus-scope` binds to a consumer-owned root. It chooses an initial
target, discovers current tabbable descendants on every Tab event, suspends
parent scopes while nested and restores a connected opener on teardown. Dialog
roles, accessible names, Escape and close policy remain local.

## Add the dependency

Pin a reviewed repository commit:

```toml
[dependencies]
dioxus-input = {
    git = "https://github.com/yam276/dioxus-common-abilities",
    rev = "<reviewed-commit-sha>",
}
```

The repository is public, so consumers do not need repository credentials. The
crates are intentionally not published to crates.io; pin a reviewed commit for
reproducible builds. A sibling checkout can use a path dependency while
developing locally:

```toml
dioxus-input = { path = "../dioxus-common-abilities/crates/dioxus-input" }
```

## Use `ImeGuard`

```rust,ignore
use dioxus::prelude::*;
use dioxus_input::use_ime_guard;

let mut ime = use_ime_guard();

rsx! {
    input {
        value: "{value}",
        oninput: move |event| {
            if ime.allows_input() {
                value.set(event.value());
            }
        },
        oncompositionstart: move |_| ime.handle_composition_start(),
        oncompositionend: move |event| {
            if let Some(fragment) = ime.handle_composition_end(event) {
                let current = value.peek().clone();
                value.set(format!("{current}{fragment}"));
            }
        },
        onkeydown: move |event| {
            if !ime.allows_keyboard_event(&event) {
                return;
            }
            // Product-owned keyboard behavior.
        },
    }
}
```

See the [crate README](crates/dioxus-input/README.md) for its exact boundary.

## Capability governance

- [Common capability wishlist](COMMON_CAPABILITY_WISHLIST.md) is the authoritative
  lifecycle and priority catalog.
- [Commonality audit](DIOXUS_COMMONALITY_AUDIT.md) records repository evidence.
- [Shared workflow candidates](SHARED_WORKFLOW_CANDIDATES.md) separates workflow
  evidence from runtime crates.
- [Active plans](docs/active) and [validation records](docs/validation) hold the
  acceptance evidence for promoted candidates.
- [Shared instruction adoption](ADOPTING_SHARED_INSTRUCTIONS.md) explains how a
  consumer repository can reuse the common agent guidance.

Candidates move through `Observed`, `Evidence-backed`, `Validating`, `Planned`,
and `Done`. A capability is not `Done` while required consumers still retain the
duplicate it is meant to replace.

## Development

Run the workspace gate from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

Runtime crates are intentionally small and do not pull in a renderer, router,
launcher, logger, storage layer, or product domain model unless their validated
boundary explicitly requires one.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)); or
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
