# Dioxus Common Abilities

Small, reusable Dioxus capabilities extracted from real applications.

This repository keeps each runtime capability in its own crate. Candidates are
promoted only after their shared invariant, consumer boundary, and validation
gate are documented. Product models, styling, routing, and command meaning stay
in the consuming application.

## Available crate

| Crate | Dioxus baseline | Purpose |
|---|---:|---|
| [`dioxus-input`](crates/dioxus-input) | `0.7.9` | Correct controlled-input behavior during CJK IME composition |

`dioxus-input` provides a non-reactive `ImeGuard` that:

- blocks controlled-value writes during composition;
- recovers the final committed fragment from `compositionend` for WebView2;
- suppresses local keyboard commands using both guard and event state.

The consumer still decides whether committed text replaces, appends, or enters
a parser, and continues to own blur, focus, validation, and global shortcuts.

## Add the dependency

Pin a reviewed repository commit:

```toml
[dependencies]
dioxus-input = {
    git = "https://github.com/yam276/dioxus-common-abilities",
    rev = "<reviewed-commit-sha>",
}
```

Private-repository access must already be configured for Git. A sibling checkout
can use a path dependency while developing locally:

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
