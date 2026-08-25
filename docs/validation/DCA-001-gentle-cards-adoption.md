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
- `cargo clippy -- -D warnings -A unknown-lints`：pass with no issues.
- `dx build --platform desktop`：bundles `GentleCardsApp.app` successfully on
  macOS.
- `dx serve --platform desktop`：launches the bundled application process; the
  development server and Tailwind watcher stop cleanly afterward.

The repository's exact `cargo clippy -- -D warnings` command remains red on the
pre-existing `#[allow(clippy::manual_option_zip)]` in
`components/play_canvas/widget_layer.rs`; the installed toolchain reports that
lint name as unknown. The adoption does not modify that file. This is recorded
as a consumer gate blocker, not hidden as an IME migration failure.

Adding the new dependency also forces Cards' tracked app lockfile to catch up
with an existing manifest mismatch：`gentle-cards-core/Cargo.toml` requires SQLx
`0.9.0`, while the app lockfile still selected SQLx `0.8.6`. Cargo therefore
updates that dependency family while adding `dioxus-input`; the same mismatch is
present on `origin/develop`, and was not introduced by the adoption source edit.

## Distribution constraint

The trial uses the sibling path
`../../dioxus-common-abilities/crates/dioxus-input`. It proves source and type
compatibility in the maintainer workspace, but a clean Cards checkout or CI job
does not fetch a sibling repository. Before this adoption can ship, replace the
path with an accessible pinned Git revision or a published crate version.

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
`Planned` until distribution, the manual matrix and independent-lineage
validation are complete. The second upgraded consumer is recorded in
`docs/validation/DCA-001-gentle-adoption.md`.
