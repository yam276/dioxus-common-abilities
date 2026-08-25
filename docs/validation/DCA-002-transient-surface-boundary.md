# DCA-002 Transient-surface Boundary Validation

Status：split accepted; backdrop state validating

Evidence date：2026-08-25

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
  consumer. This remains active and requires the API spike below.

## Decision

The original candidate must split. A modal backdrop gesture, accessible focus
lifecycle and toast queue have different owners, terminal events and failure
modes. Combining them would make consumers depend on behavior they do not use
and would hide unresolved policy differences behind a generic component.

The catalog now records:

1. `DCA-002` Backdrop-dismiss gesture state, promoted to `Validating`;
2. `DCA-022` Stable toast queue lifecycle, retained at `Evidence-backed`;
3. `DCA-023` Accessible modal focus lifecycle, retained at `Observed`.

No implementation boundary is yet `Planned`; this decision does not authorize
building any of the three crates.

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

The exact storage shape and hook names are intentionally not accepted yet. The
spike must decide whether one active pointer is sufficient or simultaneous
pointers require a small keyed set. That decision must follow tests rather than
the existing Boolean implementation.

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

All criteria above are satisfied. The next executable work is the `DCA-002`
pure-state API spike, not a modal component and not a toast crate.
