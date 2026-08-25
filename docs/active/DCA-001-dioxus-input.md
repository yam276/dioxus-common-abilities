# DCA-001 Dioxus Input Implementation Plan

Status：active

Catalog：`DCA-001`

Validation：`docs/validation/DCA-001-dioxus-input.md`

## Progress

- Workspace and `dioxus-input` crate constructed.
- Minimal `ImeGuard` API and five pure state tests implemented.
- Format, Clippy and test gates pass.
- Dependency tree contains no renderer, router, launch or logger package.
- Cards' canonical guard and `arrow_layer` near-copy are migrated with 109 tests
  and the desktop feature check passing.
- Gentle upgraded from Dioxus `0.7.4` to `0.7.9`, then migrated five manual
  guards with 63 tests, Clippy and desktop bundle/launch checks passing.
- Cards and Gentle consume the same private Git revision, with locked desktop
  builds and launch smoke tests passing without a sibling checkout.
- Authenticated clean-CI resolution remains to be verified.
- The manual WebView2 CJK matrix remains outstanding.

## Scope

Create the first `dioxus-common-abilities` runtime crate: a Dioxus `0.7.9`
composition guard for controlled text input and local keyboard-event suppression.

This plan does not upgrade or migrate consumer applications. Each consumer
migration is a later independently verifiable target change.

## Phase 1：Workspace and crate

1. Create the root Cargo workspace.
2. Add `crates/dioxus-input` with independent package version and `publish = false`.
3. Depend on exact Dioxus `0.7.9` with only `hooks`, `signals` and `html` features.
4. Add the exact format/lint/test gate in `crates/AGENTS.md`, so consumer
   repositories that include the shared root instructions do not inherit this
   workspace's package commands.

Verify：Cargo metadata resolves no renderer or router dependency for this crate.

## Phase 2：Minimal behavior

1. Implement private, pure composition state.
2. Implement the `ImeGuard` hook API accepted by validation.
3. Use `Signal::peek()` for non-reactive reads.
4. Read the final committed fragment directly from `CompositionEvent`.
5. Combine internal state and `KeyboardEvent::is_composing()` when allowing keys.

Verify：public API contains no form, product, CSS, router, async or storage types.

## Phase 3：Tests and documentation

1. Test every pure state transition and keyboard truth-table arm.
2. Include a concise usage example showing caller-owned append semantics.
3. Document replace/append/dispatch and blur as consumer responsibilities.
4. Run the complete workspace quality gate.

Verify：tests fail if start no longer blocks, finish no longer clears, empty data
commits, or either keyboard-composition source is ignored.

## Phase 4：Handoff

1. Update catalog next gate from crate construction to Cards adoption.
2. Record crate construction in `CHANGELOG.md` without marking `DCA-001` done.
3. Leave this plan active until the required consumer migrations and manual CJK
   matrix complete.

## Completion criteria

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets`
- exact Dioxus `0.7.9` baseline
- no default renderer/router/launch/logger dependency
- all accepted public methods documented
- Cards adoption evidence is recorded without moving product behavior into the crate
- authenticated clean-CI resolution of the pinned private Git dependency
- Catalog status remains `Planned` until consumer validation is complete
