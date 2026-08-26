# DCA-023 focus-lifecycle validation fixture

This standalone Dioxus `0.7.9` app validates the shared
`dioxus-focus-scope` crate in real browser behavior.

It deliberately lives outside the root Cargo workspace. Dialog markup, accessible
names, Escape and close callbacks remain fixture-owned consumer policy.

The scenarios cover:

- initial focus, forward/reverse tab wrapping, Escape and focus restoration;
- nested dialog scope isolation and restoration to the parent opener;
- fallback behavior when a dialog has no tabbable descendants;
- live tabbable discovery after controls are added or removed;
- disabled initial-target fallback and safe teardown after an opener is removed.

Run it from this directory:

```sh
cargo check
dx serve --web
```

The renderer-level acceptance results belong in
`docs/validation/DCA-023-accessible-modal-focus.md`.
