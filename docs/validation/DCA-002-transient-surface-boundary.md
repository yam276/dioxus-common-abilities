# DCA-002 Transient-surface Boundary Validation

Status：complete; backdrop state adopted and runtime-validated

Evidence date：2026-08-25

Completion date：2026-08-28

Dioxus baseline：`0.7.9`

## Objective

Determine whether the original `DCA-002` candidate, "headless modal and toast
behavior", describes one common lifecycle. If it does not, split it by failure
semantics before any shared crate is implemented.

This validation compares current source, not only the Gentle fork history. The
source revisions inspected were:

- Gentle Cards `d6c168e399802f74a4e932ca859a9cfd0d586616`;
- Gentle `7bb8c954ff622186a6b9f2506df258ef238d73df`;
- OxDM `36886a45dfa260c9090048c8ece76482810b8691`;
- Deductree/Diolama `ac2b0b977e7768db868dc080de200b783fc6d020`.

## Hypotheses

- **Combined hypothesis:** modal gesture/focus and toast expiry share enough
  lifecycle to belong in one crate. The current-source comparison rejects this.
- **Narrow hypothesis:** backdrop release correctness can be expressed as a
  small pointer-state machine that removes duplicated edge-case logic from a
  consumer. Mapping the API below onto Cards, Gentle and Deductree/Diolama
  accepts this hypothesis.

## Decision

The original candidate must split. A modal backdrop gesture, accessible focus
lifecycle and toast queue have different owners, terminal events and failure
modes. Combining them would make consumers depend on behavior they do not use
and would hide unresolved policy differences behind a generic component.

At this boundary-validation gate, the catalog recorded:

1. `DCA-002` Backdrop-dismiss gesture state, promoted to `Planned`;
2. `DCA-022` Stable toast queue lifecycle, retained at `Evidence-backed`;
3. `DCA-023` Accessible modal focus lifecycle, retained at `Observed`.

Only `DCA-002` advanced to `Planned` at that gate. `DCA-022` and `DCA-023`
remained outside this implementation scope and continued through their own
independent lifecycle decisions.

## Current modal evidence

| Consumer | Backdrop policy | Escape and focus | Important consequence |
|---|---|---|---|
| Gentle Cards | Boolean armed by backdrop `pointerdown`; backdrop `pointerup` closes when armed | Backdrop requests focus on mount; Escape calls the same close callback | Fixes the known press-inside/release-outside click-ancestry bug, but does not match pointer IDs or reset on `pointercancel` |
| Gentle | Backdrop `onclick`; panel stops click propagation | Same focus request and Escape callback as Cards | Still carries the false-dismiss behavior fixed in the Cards fork |
| OxDM | Backdrop `mousedown` closes immediately; panel stops propagation | Focusable overlay, but no explicit initial-focus request | Avoids click ancestry by choosing a different product policy: release location is irrelevant |
| Diolama confirmation | Backdrop `onclick`; panel stops propagation | `alertdialog`, `aria-modal`, default focus, instance-scoped Tab trap and return focus | Useful focus-lifecycle reference, but its confirmation generation/cancel policy is domain behavior, not a generic modal API |

Cards commit `ea6e36e` is concrete incident evidence: it replaced backdrop
`onclick` with pointer down/up state after the user observed a dialog closing
when a press began inside and ended outside.

The current Boolean fix is evidence for the invariant, not the final API. It
has two remaining lifecycle gaps:

- it does not prove that down/up came from the same pointer;
- `pointercancel` and a backdrop-down/panel-up gesture do not immediately clear
  the armed state.

Dioxus `0.7.9` exposes `PointerData::pointer_id()` and `onpointercancel`, so the
validation spike can test those terminal paths without a JavaScript listener.

## Accepted DCA-002 boundary

The shared mechanism may own only the pointer-gesture state and its truth
table. It emits a dismiss intent; it never unmounts a modal or decides whether
the product is allowed to close.

| Event sequence | Required result |
|---|---|
| Backdrop down A, backdrop up A | Emit one dismiss intent and clear A |
| Panel down A, backdrop up A | Do not dismiss |
| Backdrop down A, panel up A | Do not dismiss and clear A |
| Backdrop down A, cancel A | Do not dismiss and clear A |
| Backdrop down A, backdrop up B | Do not dismiss for B |
| Any later event after a terminal event | Observe no stale state for the completed pointer |

The source comparison shows simultaneous pointers require a small keyed set: a
release for pointer B must neither dismiss nor erase pointer A. The accepted
renderer-neutral API is:

```rust
pub struct BackdropDismissState { /* private */ }

impl BackdropDismissState {
    pub fn pointer_down_on_backdrop(&mut self, pointer_id: i32);
    pub fn pointer_down_on_content(&mut self, pointer_id: i32);
    pub fn pointer_up_on_backdrop(&mut self, pointer_id: i32) -> bool;
    pub fn pointer_up_on_content(&mut self, pointer_id: i32);
    pub fn pointer_cancel(&mut self, pointer_id: i32);
}
```

The state uses pointer IDs directly and has no Dioxus dependency. Consumers
extract `PointerData::pointer_id()` in thin event adapters. A rendered component
or hook would own too much markup and event-policy composition for this
invariant.

The consumer continues to own:

- whether backdrop dismissal is enabled at all;
- busy/import/delete guards and the actual close mutation;
- Escape and context-menu policy;
- markup, portal/layer placement, CSS, width and animation;
- panel contents, actions, copy and icons;
- initial-focus target, focus trap, return focus and ARIA composition.

## Why toast is a separate candidate

Gentle and Cards have byte-identical toast files with SHA-256
`f9a041a7df372acc975c7161238b1e40365277dde6b0b0e3d110d618cb5b471b`.
That proves fork duplication, but the independent implementations expose a
different lifecycle from modal gestures and from each other.

| Consumer | Queue and identity | Expiry | Manual dismissal and host |
|---|---|---|---|
| Gentle/Cards | Global bounded queue, capacity five, atomic `u64` IDs, four severity levels | Per-kind 4/6/8 second browser timers | Whole card dismisses; host is above the router |
| OxDM | Context-owned unbounded queue, `u32` IDs, four severity levels | Uniform four-second Tokio task | Explicit close button; one app-shell host |
| Deductree | Context-owned unbounded queue, wrapping `u64` IDs, text payload only | Uniform 3.2-second `futures_timer::Delay` | No manual dismissal; `ToastHost` is mounted at four screen/route sites |

The common toast invariant is stable identity and safe removal: an expiry or
manual dismissal may remove only its original toast, repeated removal is a
no-op and an optional capacity drops the oldest item deterministically. Payload,
severity, duration, renderer and visual dismissal control remain consumer
policy.

The unresolved part is scheduler ownership. A shared queue that leaves every
timer/remount rule local may be too small to justify a dependency, while a
shared host would incorrectly own router placement and runtime choice. Therefore
`DCA-022` remains `Evidence-backed` until two opposing API sketches prove value.

## Why focus is a separate candidate

The products have not converged on a focus contract:

- Gentle/Cards focus the backdrop and rely on key bubbling;
- OxDM makes the overlay focusable but does not explicitly move focus;
- Diolama installs an instance-scoped Tab trap, chooses a default focus target
  and restores the previously focused element.

This is enough to preserve an accessibility candidate, but not enough to choose
a reusable component or hook. `DCA-023` must first define acceptance for initial
focus, Tab/Shift+Tab containment, nested instances, return focus, semantic role
and cancellation policy without importing Diolama's confirmation state machine.

## Validation acceptance

This boundary validation passes when:

- modal pointer state, modal focus and toast queue are represented as separate
  failure contracts;
- the Cards incident and an independent opposing implementation are both
  represented;
- no candidate owns product CSS, copy, domain state or router structure;
- the backdrop truth table includes panel release, pointer cancellation and
  mismatched pointer identity;
- no shared implementation is started before the candidate's next gate.

All criteria above were satisfied at the boundary-validation gate. `DCA-002`
then advanced to `Planned`; its completed implementation plan is archived at
`docs/done/DCA-002-backdrop-dismiss.md`. The delivered mechanism is the pure
state crate and thin consumer adapters, not a modal component or toast crate.

## Implementation evidence

The accepted boundary is implemented and pinned at shared revision
`0d2adc322b6f14057b59b0005cf8e19d9b46a6c5`.

| Consumer | Commit | Covered surfaces | Automated verification |
|---|---|---|---|
| Gentle Cards | `9649afc` | modal primitive, Card Zoom, Scry finish/preview | Clippy, native tests (109), wasm32 web check |
| Gentle | `a2a3839` | modal primitive | default and `adult` Clippy/tests (63/85) |
| Deductree app | `354fd43` | journal, asset library, cast library and nested asset picker | app Clippy with warnings denied |
| Diolama | `354fd43` | confirmation, settings, log, present, save and system menu | Clippy and 622 tests, including private state truth-table tests |
| OxDM | unchanged at comparison revision | central `DialogOverlay` | source inspection confirms close-on-backdrop-`mousedown` already rejects inside starts |

The Diolama state is intentionally private and locally mirrored. Diolama's
package metadata describes a publishable library; adding a private Git
dependency would make that contract undistributable. Its module cites `DCA-002`
and can switch to the shared package once that package has an approved public
identity.

Pointer event propagation is GUI behavior, so compilation and pure state tests
did not by themselves prove the RSX wiring. The final gate was a runtime matrix
on one Cards/Gentle surface and one Deductree/Diolama surface:

| Gesture | Expected |
|---|---|
| Backdrop down and up | Close once |
| Panel down, drag outside, release | Stay open |
| Backdrop down, drag inside, release | Stay open |
| Cancelled pointer gesture | Stay open; next legitimate backdrop gesture closes once |

## Runtime acceptance receipts

The final probes ran on 2026-08-28 in hidden desktop WebViews. Each probe used
cancelable pointer events against the consumer's actual rendered backdrop and
panel, so Dioxus event propagation, `pointer_id()` extraction, propagation
stops, state transitions and product close callbacks all remained in the path.
They did not attempt to infer native cursor geometry from JavaScript.

| Consumer surface | Current commit | Shared backdrop source | Result |
|---|---|---|---|
| Gentle Cards shared `Modal` | `54fc8d460298e106b4a945a48d4e7b8525b2df62` | descendant pin `acd3e513fab7ec370c4e7a241ed5585770a7a75a` | Pass |
| Deductree `JournalOverlay` | `c98f0f3f9f1887be4eabb1658201db1d0069869a` | descendant pin `acd3e513fab7ec370c4e7a241ed5585770a7a75a` | Pass |

| Gesture | Cards observed | Deductree observed | Result |
|---|---|---|---|
| Panel down A, backdrop up A | Close callback count remained zero | Journal remained mounted | Pass |
| Backdrop down A, panel up A | Close callback count remained zero | Journal remained mounted | Pass |
| Backdrop down A, cancel A, later up A | No close and no stale arm | No close and no stale arm | Pass |
| Backdrop down A, backdrop up B | No close; Cards then accepted A, proving B did not erase it | No close; A was cancelled before the next gesture | Pass |
| Backdrop down A, backdrop up A | Exactly one new close callback | Journal unmounted | Pass |

The Cards probe exited before its SQLite/backend startup. The Deductree probe
used the in-memory app context and did not enter a persistence path. Both probes
were removed immediately after the run, both consumer worktrees returned clean,
and neither hidden window activated the foreground application.

Current gate receipts remain green:

- common workspace formatter, warning-denied Clippy, and 16 tests;
- Gentle Cards formatter, warning-denied Clippy, and 109 tests;
- Gentle formatter plus warning-denied default/adult Clippy and 63/85 tests;
- Deductree current adoption commit's formatter, both warning-denied Clippy
  gates, 43 core tests and 128 focused Story Editor tests;
- Diolama's original adoption gate recorded above, including 622 tests and its
  private truth-table coverage.

## Conclusion

`dioxus-backdrop-dismiss` satisfies the DCA-002 truth table across the Gentle
lineage and an independent Deductree/Diolama lineage without owning close,
Escape, focus, markup or domain policy. OxDM remains unchanged because its
close-on-backdrop-down policy already rejects gestures that begin inside.
DCA-002 is complete and its plan is archived under `docs/done/`.
