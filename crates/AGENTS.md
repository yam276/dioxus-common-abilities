# dioxus-common-abilities crate instructions

These rules apply to code under `crates/` in addition to the repository root
`AGENTS.md`.

## Quality gate

Run from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

All commands must pass before executable crate work is done.

## Package boundaries

- Every crate owns an independent package version.
- Keep `publish = false` until its wishlist entry reaches `Done` and publishing
  is explicitly approved.
- Pin the supported Dioxus baseline exactly while a crate is under validation.
- Use only the Dioxus features required by the crate; do not pull a renderer,
  router, launcher, logger or asset system into a headless integration helper.
- Public APIs must contain no consumer product types.
- A consumer migration is a separate target change from creating the shared
  crate.
