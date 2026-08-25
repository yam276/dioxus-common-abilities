# DCA-002 Backdrop-dismiss Implementation Plan

Status：active

Catalog：`DCA-002`

Validation：`docs/validation/DCA-002-transient-surface-boundary.md`

## Scope

Create one renderer-neutral pointer-state crate that emits a dismiss intent
only when the same pointer begins and ends on a backdrop. Adopt it without
changing product close, Escape, context-menu, focus, markup or styling policy.

## Non-goals

- Rendered modal, overlay or portal components
- Focus trap, initial focus, return focus or ARIA composition
- Escape, context-menu, busy-state or domain cancellation policy
- Toast queue, timer, host or renderer behavior
- Product CSS, copy, icons, animations or panel contents
- Compatibility with Dioxus before `0.7.9`

## Phase 1：Pure state crate

1. Add `crates/dioxus-backdrop-dismiss` to the workspace.
2. Store backdrop-started pointer IDs in a deterministic keyed set.
3. Expose the five accepted transition methods and no renderer dependency.
4. Test matching release, panel start/release, cancellation, mismatched IDs and
   simultaneous pointers.
5. Document Dioxus `PointerData::pointer_id()` adapter wiring.

Verify：workspace format, Clippy and test gates pass; the package dependency
tree contains no Dioxus renderer, router or domain crate.

## Phase 2：Gentle lineage

1. Replace the Cards modal Boolean guard with the shared state.
2. Apply the same state to direct nested Card Zoom and Scry overlays.
3. Replace Gentle's click-based modal backdrop with the shared state.
4. Preserve each existing close callback and context-menu/Escape behavior.

Verify：each frontend's complete local gate passes and the dependency is pinned
to one reviewed Git revision.

## Phase 3：Independent lineage

1. Adopt the crate in Deductree app overlays whose panel is nested inside a
   click-closing backdrop.
2. Keep Diolama publishable: implement the same private truth table inside
   Diolama rather than adding a private Git dependency to its package contract.
3. Apply that internal state to every Diolama overlay that closes from a nested
   backdrop click.
4. Leave OxDM's deliberate close-on-backdrop-`mousedown` policy unchanged; it
   already cannot close from an inside-down/outside-up gesture.

Verify：Deductree's complete pre-commit gate passes and all affected surfaces
map every terminal event to the accepted state machine.

## Phase 4：Evidence and handoff

1. Record exact shared revision and consumer commits in validation evidence.
2. Keep `DCA-002` short of `Done` until a pointer-drag runtime smoke confirms
   event wiring on one desktop WebView and one browser/WebView consumer.
3. Do not start `DCA-022` or `DCA-023` as part of this work.

## Completion criteria

- Same-pointer backdrop down/up dismisses once.
- Inside-down/outside-up never dismisses.
- Outside-down/inside-up never dismisses and leaves no stale arm.
- `pointercancel` never dismisses and clears only its pointer.
- A mismatched pointer release cannot dismiss or erase another pointer.
- Cards, Gentle and Deductree/Diolama compile against the intended behavior.
- OxDM's existing safe opposing policy is recorded without semantic churn.
