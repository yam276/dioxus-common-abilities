# dioxus-focus-scope

`dioxus-focus-scope` owns one narrow overlay invariant for Dioxus `0.7.9`:
focus enters the active scope, Tab and Shift+Tab stay in its current controls,
nested scopes suspend their parent, and teardown restores a safe opener.

The crate does not render a wrapper and does not own dialog roles, accessible
names, Escape, backdrop dismissal, close callbacks, busy state, styling or
domain actions.

## Use

Create the hook in a component that mounts and unmounts with the scope root:

```rust,ignore
use dioxus::prelude::*;
use dioxus_focus_scope::{use_focus_scope_with_options, FocusScopeOptions};

let scope = use_focus_scope_with_options(
    FocusScopeOptions::default().with_initial_focus_id("primary-action"),
);
let root_id = scope.root_id().to_string();
let tab_index = scope.root_tab_index();
let mounted_scope = scope.clone();

rsx! {
    section {
        id: root_id,
        tabindex: tab_index,
        role: "dialog",
        aria_modal: true,
        aria_labelledby: "dialog-title",
        onmounted: move |_| mounted_scope.activate(),
        h2 { id: "dialog-title", "Product-owned title" }
        button { id: "primary-action", "Continue" }
    }
}
```

Use `use_focus_scope()` when the first current tabbable descendant is the right
initial target. If no tabbable descendant exists, the root receives focus.

The consumer must:

- keep the hook lifecycle aligned with the root's mounted lifetime;
- apply the returned root ID and `tabindex`;
- compose the correct dialog semantics and accessible name;
- keep initial-target IDs unique in the document;
- own Escape, backdrop, close and domain policy.
