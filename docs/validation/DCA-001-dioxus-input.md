# DCA-001 Dioxus IME Input Validation

Status：accepted for planning

Evidence date：2026-08-25

Dioxus baseline：`0.7.9`

Per maintainer decision, this capability does not support Dioxus `0.7.4`.
Consumers on older Dioxus versions must upgrade before adoption; compatibility
shims do not belong in the crate.

## Objective

Determine whether the repeated CJK/WebView2 controlled-input behavior can be
owned by one small Dioxus crate without owning form models, value semantics,
styling, routers or application commands.

## Evidence read

### Canonical seed

- `gentle-cards/gentle-cards-app/src/components/primitives/use_ime.rs`
  defines the 62-line `ImeGuard` and `use_ime_guard`.
- Cards consumers：`dynamic_form.rs` and `pages/decks_list.rs`.
- Cards near-copy：`arrow_layer.rs::LabelInput` adds Enter/Escape and blur
  behavior around the same composition state.
- Cards global shortcut：`play_canvas/keyboard.rs::KEYDOWN_JS` separately
  excludes editable targets and `e.isComposing`.

### Same-lineage validation

Gentle has manual `is_composing` implementations in：

- `pages/gallery_list.rs`
- `pages/create_work/tag_section.rs`
- two inputs in `components/dynamic_form.rs`
- `components/work_links.rs`

They use the same non-reactive `Signal::peek()` rule and direct
`compositionend.data()` recovery.

### Framework capability

Dioxus `0.7.9` exposes：

- `CompositionEvent` / `CompositionData::data()`；
- `KeyboardEvent::is_composing()`；
- `Signal` and `use_signal` without requiring a renderer feature。

## Common state machine

| Event/state | Common behavior | Caller-owned behavior |
|---|---|---|
| Initial | Input and commands allowed | Initial value |
| `compositionstart` | Mark composing without reactive subscription | None |
| `input` while composing | Ignore signal write | DOM/IME maintains in-flight buffer |
| `keydown` while guard active | Suppress product command | Browser handles candidate selection |
| `keydown` reports `is_composing` | Suppress even if guard state is stale | Browser handles candidate selection |
| `compositionend` with data | Clear composing and return committed fragment | Replace, append or dispatch fragment |
| `compositionend` empty | Clear composing and return `None` | Cancellation/no-op policy |
| Later `input` event | Allowed | Consumer may receive full DOM value |
| Blur | No universal commit | Product owns commit/cancel/unmount policy |

## Stable boundary

The crate owns：

- composition-active state；
- non-reactive state reads；
- WebView2 final fragment extraction；
- local Dioxus keyboard-event suppression using both state and event flag。

The consumer owns：

- controlled value and cursor semantics；
- whether the final fragment replaces, appends or enters a parser；
- Enter/Escape command meaning；
- blur behavior；
- validation, suggestions, chips and resources；
- global JavaScript listener installation/removal and editable-target policy。

Global JavaScript shortcuts must check `e.isComposing`, but a Rust crate should
not own a product's listener string or `window` global key.

## Accepted public API

```rust
#[derive(Clone, Copy)]
pub struct ImeGuard { /* private */ }

pub fn use_ime_guard() -> ImeGuard;

impl ImeGuard {
    pub fn is_composing(&self) -> bool;
    pub fn allows_input(&self) -> bool;
    pub fn allows_keyboard_event(&self, event: &KeyboardEvent) -> bool;
    pub fn handle_composition_start(&mut self);
    pub fn handle_composition_end(
        &mut self,
        event: CompositionEvent,
    ) -> Option<String>;
}
```

`is_composing` remains public because suggestion/dropdown visibility sometimes
depends on it. `allows_input` is the preferred controlled-input predicate.

## Non-goals

- Rendered `ControlledInput` component
- Form schema or validation trait
- Cursor/selection reconstruction
- Async suggestion/resource management
- Chip syntax or value merging
- Focus and blur policy
- Router/root context
- CSS or icons
- Product command enum
- Global JavaScript listener builder
- Dioxus `0.7.4` compatibility

## Test requirements

Pure internal state tests must prove：

- default state allows input and non-composing key events；
- start blocks input and keys；
- non-empty finish returns the exact fragment and re-allows input；
- empty finish clears state and returns `None`；
- a keyboard event's own composing flag blocks commands even if guard state is
  already false。

The test for keyboard event state may exercise an internal boolean-level
predicate; constructing renderer/platform events is not required to verify that
truth table.

## Manual consumer matrix

Before `Done`, run on WebView2 Windows and one non-Windows browser/WebView：

| Scenario | Expected |
|---|---|
| Zhuyin candidate selection with Enter | No submit/unmount; final Han text commits once |
| Pinyin candidate selection | No intermediate Latin buffer overwrite |
| Kana conversion | Conversion keys do not trigger product commands |
| Existing prefix plus composition | Caller-specific append/replace behavior remains unchanged |
| Empty/cancelled composition | Guard clears; no phantom text |
| Escape while composing | Browser/IME handles it; product cancel does not fire |
| Blur after committed composition | Product's existing blur policy runs |
| Global shortcut during composition | Listener rejects through `e.isComposing` |

## Validation decision

Accepted. Existing implementations fit the same mechanism without product
branches. The API is smaller than the consumer behaviors and does not require a
renderer, router, storage or domain dependency.

`DCA-001` may move to `Planned`. Current completion state：

1. shared crate quality gate：complete；
2. Cards adoption with no behavior expansion：implemented；
3. a second upgraded consumer：Gentle implemented after its Dioxus `0.7.9`
   upgrade；
4. independent-lineage validation：open；
5. the manual CJK matrix：open；
6. distributable dependency identity：complete at private Git revision
   `afc7c77a732ebac56f46561f2d75c04522d8b5bc`；
7. authenticated clean-CI resolution：open。

Cards implementation evidence is recorded in
`docs/validation/DCA-001-gentle-cards-adoption.md`. Its automated adoption checks
pass subject to the documented pre-existing Clippy lint-name blocker；authenticated
CI and the manual matrix remain open.

Gentle upgrade and adoption evidence is recorded in
`docs/validation/DCA-001-gentle-adoption.md`.
