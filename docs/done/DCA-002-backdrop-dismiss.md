# DCA-002 Backdrop-dismiss Implementation Plan

Status：done

Catalog：`DCA-002`

Validation：`docs/validation/DCA-002-transient-surface-boundary.md`

## Progress

- Shared crate implemented at
  `0d2adc322b6f14057b59b0005cf8e19d9b46a6c5` with six pure tests and no
  dependencies.
- Gentle Cards adopted the state in its modal primitive, Card Zoom and Scry
  finish overlays at `9649afc`.
- Gentle adopted the state in its modal primitive at `a2a3839`.
- Deductree app adopted the shared crate in journal, asset-library and nested
  cast-library overlays; Diolama mirrors the same private state in confirmation,
  settings, log, present, save and system-menu overlays at `354fd43`.
- OxDM remains unchanged because its panel stops the initial `mousedown` and the
  backdrop closes immediately only when that down begins outside.
- All automated package and consumer gates pass.
- Runtime pointer matrices pass in Gentle Cards' actual shared `Modal` and
  Deductree's actual `JournalOverlay` at their current consumer revisions.

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

Status：**COMPLETE — 2026-08-28**

- The validation record retains implementation revision
  `0d2adc322b6f14057b59b0005cf8e19d9b46a6c5` and every original adoption
  commit. Gentle still pins that revision; later Cards and Deductree revisions
  pin descendant `acd3e513fab7ec370c4e7a241ed5585770a7a75a`, whose backdrop crate is
  unchanged.
- Temporary hidden desktop-WebView probes exercised the actual Cards `Modal`
  and Deductree `JournalOverlay` event adapters. Both passed same-pointer close,
  both cross-boundary directions, cancellation and mismatched-pointer checks.
- Probe code was removed after each run. Both consumer worktrees were clean,
  Cards bypassed its backend, and no user document or database was modified.
- Common, Cards and Gentle gates pass at the recorded revisions. Deductree and
  Diolama retain the complete green gates recorded with their current adoption
  commits.
- The plan is archived under `docs/done/`, its active wishlist record is
  removed, and the completed outcome is recorded in `CHANGELOG.md`.

## Completion criteria

- Same-pointer backdrop down/up dismisses once.
- Inside-down/outside-up never dismisses.
- Outside-down/inside-up never dismisses and leaves no stale arm.
- `pointercancel` never dismisses and clears only its pointer.
- A mismatched pointer release cannot dismiss or erase another pointer.
- Cards, Gentle and Deductree/Diolama compile against the intended behavior.
- OxDM's existing safe opposing policy is recorded without semantic churn.
