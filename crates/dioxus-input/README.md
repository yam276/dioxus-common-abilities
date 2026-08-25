# dioxus-input

Small Dioxus `0.7.9` input-correctness primitives.

The first capability is `ImeGuard`, which prevents controlled inputs and local
keyboard handlers from disrupting CJK composition. It also recovers the final
committed fragment directly from `compositionend`, covering a WebView2 event
sequence where the follow-up `input` event may be absent.

```rust,ignore
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
            // Product-owned Enter/Escape behavior.
        },
    }
}
```

The returned composition data is a fragment, not a full controlled value. The
consumer decides whether to append, replace or send it through a parser.

This crate does not own form models, validation, focus/blur behavior, async
suggestions, CSS, routers, application commands or global JavaScript listeners.
Global listeners remain consumer-owned and must reject `e.isComposing`.
