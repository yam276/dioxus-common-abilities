# DCA-023 focus-lifecycle validation fixture

This standalone Dioxus `0.7.9` app evaluates the current first-party
`dioxus-primitives` dialog implementation at pinned revision
`bf007c15d0cf4d04d3181cc46cf12325aa773955`.

It is evidence, not a production crate. It deliberately lives outside the root Cargo
workspace and must not be used by a consumer application.

The three scenarios cover:

- initial focus, forward/reverse tab wrapping, Escape and focus restoration;
- nested dialog scope isolation and restoration to the parent opener;
- fallback behavior when a dialog has no tabbable descendants.

Run it from this directory:

```sh
cargo check
dx serve --web
```

The renderer-level acceptance results belong in
`docs/validation/DCA-023-accessible-modal-focus.md`.
