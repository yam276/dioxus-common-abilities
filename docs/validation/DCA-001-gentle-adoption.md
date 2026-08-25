# DCA-001 Gentle Adoption

Status：implemented; manual validation pending

Evidence date：2026-08-25

Consumer：`gentle/gentle-app`

## Upgrade gate

Gentle `develop` was clean, synchronized with `origin/develop` at `0991802`,
and still pinned Dioxus `0.7.4`. The app was first upgraded to exact Dioxus
`0.7.9` without adding `dioxus-input`.

The isolated framework upgrade passed：

- `cargo fmt -- --check`；
- `cargo clippy -- -D warnings` with no issues；
- `cargo test` with 63 tests across five suites；
- `dx build --platform desktop` and macOS app bundling。

No `0.7.4` compatibility shim was required.

## Migration result

After the upgrade gate passed, five manual composition guards moved to the
shared crate：

- `gallery_list.rs::ChipInput` keeps chip syntax, suggestion visibility and
  composition-end dispatch local；
- `create_work/tag_section.rs::TagCategoryRow` keeps suggestion and tag-creation
  policy local；
- `dynamic_form.rs::TextFieldInput` keeps its parent-controlled append policy；
- `dynamic_form.rs::ChipFieldInput` keeps chip Enter and blur commits local；
- `work_links.rs::PlatformPicker` keeps modal, focus and search policy local。

All former `Signal<bool>` composition cells and direct
`CompositionEvent::data()` extraction in those consumers are removed. Inputs
use `allows_input()`；local keyboard commands use
`allows_keyboard_event()`；suggestion visibility may use the non-reactive
`is_composing()` read.

## Final automated evidence

Run from `gentle/gentle-app` after adoption：

- `cargo fmt -- --check`：pass.
- `cargo clippy -- -D warnings`：pass with no issues.
- `cargo test`：63 tests passed across five suites.
- `dx build --platform desktop`：bundles `GentleApp.app` successfully on macOS
  and explicitly compiles `dioxus_input`.
- `dx serve --platform desktop`：launches the bundled application process；the
  app, development server and Tailwind watcher stop cleanly afterward.

## Distribution and remaining validation

Like the Cards trial, Gentle currently uses the sibling path
`../../dioxus-common-abilities/crates/dioxus-input`. This validates the source
boundary but must become an accessible pinned Git revision or published crate
before clean-checkout CI can build it.

Gentle is the required second upgraded consumer, but it shares lineage with
Cards. Independent-lineage validation and the Windows WebView2 CJK matrix remain
open, so `DCA-001` stays `Planned`.
