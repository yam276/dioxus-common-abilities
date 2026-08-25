# dioxus-backdrop-dismiss

Small, renderer-neutral pointer state for reliable backdrop dismissal.

The state returns `true` only when the same pointer begins and ends on the
backdrop. Content releases, cancelled pointers and unrelated pointer IDs cannot
dismiss or leave stale state for the completed pointer.

```rust,ignore
use dioxus::prelude::*;
use dioxus_backdrop_dismiss::BackdropDismissState;

let mut dismiss = use_signal(BackdropDismissState::default);

rsx! {
    div {
        class: "product-backdrop",
        onpointerdown: move |event| {
            dismiss.write().pointer_down_on_backdrop(event.pointer_id());
        },
        onpointerup: move |event| {
            if dismiss.write().pointer_up_on_backdrop(event.pointer_id()) {
                on_close.call(());
            }
        },
        onpointercancel: move |event| {
            dismiss.write().pointer_cancel(event.pointer_id());
        },
        div {
            class: "product-panel",
            onpointerdown: move |event| {
                dismiss.write().pointer_down_on_content(event.pointer_id());
                event.stop_propagation();
            },
            onpointerup: move |event| {
                dismiss.write().pointer_up_on_content(event.pointer_id());
                event.stop_propagation();
            },
            {children}
        }
    }
}
```

The consumer owns rendering, whether dismissal is enabled, the actual close
mutation, Escape and context-menu policy, focus/ARIA behavior, CSS, animation
and content.

The crate has no Dioxus dependency. Its `i32` IDs accept Dioxus `0.7.9`
`PointerData::pointer_id()` values through the thin adapter shown above.
