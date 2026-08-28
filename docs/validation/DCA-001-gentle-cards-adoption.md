# DCA-001 Gentle Cards Adoption

Status：implemented; manual validation pending

Evidence date：2026-08-25

Consumer：`gentle-cards/gentle-cards-app` at Dioxus `0.7.9`

## Scope

Replace Cards' local IME guard and its `arrow_layer` near-copy with the
`dioxus-input` crate without changing product-owned value, blur or command
semantics.

## Migration result

- `dynamic_form.rs` uses `allows_input()` for its text and chip fields and
  `allows_keyboard_event()` before the chip Enter command.
- `pages/decks_list.rs` keeps its existing replace-on-composition-end policy.
- `arrow_layer.rs::LabelInput` keeps append, Enter/Escape and blur behavior local
  while delegating composition state and keyboard suppression to `ImeGuard`.
- The 62-line local `components/primitives/use_ime.rs` implementation and its
  module re-export are removed.
- The product's global JavaScript keyboard listener remains local and continues
  to reject `e.isComposing`.

No form model, parser, resource, CSS, focus/blur policy or command type moved
into the shared crate.

## Automated evidence

Run from `gentle-cards/gentle-cards-app`：

- `cargo fmt -- --check`：pass.
- `cargo test`：109 tests passed across five suites.
- `cargo check --no-default-features --features desktop`：pass.
- `cargo tree -i dioxus-input`：resolves the shared crate as a direct Cards
  dependency.
- `cargo clippy -- -D warnings`：pass with no issues.
- `dx build --platform desktop`：bundles `GentleCardsApp.app` successfully on
  macOS.
- `dx serve --platform desktop`：launches the bundled application process; the
  development server and Tailwind watcher stop cleanly afterward.

Adding the new dependency also forces Cards' tracked app lockfile to catch up
with an existing manifest mismatch：`gentle-cards-core/Cargo.toml` requires SQLx
`0.9.0`, while the app lockfile still selected SQLx `0.8.6`. Cargo therefore
updates that dependency family while adding `dioxus-input`; the same mismatch is
present on `origin/develop`, and was not introduced by the adoption source edit.

## Distribution result

At validation time, Cards used the then-private Git repository pinned to
`afc7c77a732ebac56f46561f2d75c04522d8b5bc`. Its lockfile records the same full
source revision, and locked tests, desktop bundling and launch checks pass
without a sibling checkout.

At validation time, GitHub Actions used the read-only `consumer-actions-readonly` deploy key through
the `DIOXUS_COMMON_ABILITIES_SSH_KEY` repository secret. The manually dispatched
[`develop` workflow run 32841974744](https://github.com/yam276/gentle-cards/actions/runs/32841974744)
passed all three jobs on a clean runner；the app job authenticated, fetched the
pinned private revision, then passed format, Clippy and all 109 tests.

The later public-repository conversion removes the credential requirement. A
sibling checkout remains unnecessary.

## Remaining manual matrix

Still required on Windows WebView2：

- Zhuyin selection with Enter;
- Pinyin composition;
- Kana conversion;
- existing-prefix append and replace cases;
- cancelled/empty composition;
- Escape during composition;
- blur after commit;
- global shortcut suppression.

## Decision

The Cards migration confirms that the shared API replaces both the canonical
guard and the label-editor near-copy without product branches. `DCA-001` remains
`Planned` until the manual matrix and independent-lineage validation are
complete. The second upgraded consumer is recorded in
`docs/validation/DCA-001-gentle-adoption.md`.
