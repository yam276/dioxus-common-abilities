# DCA-023 Focus-scope Implementation Plan

Status：active

Catalog：`DCA-023`

Validation：`docs/validation/DCA-023-accessible-modal-focus.md`

## Progress

- `dioxus-focus-scope` crate constructed at version `0.1.0`, `publish = false`.
- Public boundary is a zero-configuration hook plus an optional initial-target
  option applied to the consumer's existing root.
- Five focused Rust tests cover identity, safe JavaScript literals, optional
  target generation and the absence of crate-owned Escape policy.
- Normal dependency tree contains Dioxus document/core support but no renderer,
  router, launcher, logger or product domain crate.
- The standalone fixture passes trusted browser Tab and Shift+Tab checks for
  basic, nested, dynamic and empty scopes, plus disabled-target and
  disconnected-opener fallbacks.
- Gentle Cards adopted the reviewed `acd3e51` revision in consumer commit
  `54fc8d460298e106b4a945a48d4e7b8525b2df62`; its automated gates, web build
  and manual desktop runtime receipt pass.
- Deductree nested adoption remains outstanding.

## Scope

Create a small Dioxus `0.7.9` crate named `dioxus-focus-scope`. A consumer marks
its own panel root as one focus scope; the crate then owns initial focus,
dynamic Tab and Shift+Tab containment, innermost-scope activation, no-tabbable
fallback and safe focus restoration when that scope unmounts.

The public boundary is a hook plus attributes or handlers applied to the
consumer's existing root element. It must not require a rendered wrapper or
prescribe `div`, `section`, portal or overlay structure. The root remains
programmatically focusable so it can serve as the fallback target.

## Non-goals

- Dialog, alertdialog, title or description markup
- Escape, backdrop, close, cancel or default-action policy
- Busy, submitting, animation or delayed-unmount state
- Product CSS, copy, icons, z-index or portal placement
- A modal component, router integration or domain callbacks
- OxDM migration or validation
- Compatibility with Dioxus before `0.7.9`

ARIA semantics remain a required consumer-adoption check, but not crate-owned
markup. This keeps a focus mechanism from becoming a second modal framework.

## Phase 1：Crate and public boundary

1. Add `crates/dioxus-focus-scope` to the workspace with `publish = false` and
   exact Dioxus `0.7.9` dependencies limited to the APIs the hook uses.
2. Define one hook-owned scope identity and one root-binding API. Keep the
   consumer's existing element and callbacks intact.
3. Capture the previously focused connected element at activation.
4. On activation, prefer the consumer-marked initial target, then the first
   currently tabbable descendant, then the focusable scope root.
5. Discover tabbable descendants at each Tab event so disabled, hidden, added
   and removed controls cannot stale a cached list.
6. Maintain an instance stack so only the innermost mounted scope handles Tab.
7. On teardown, restore the connected opener; if it is unavailable, fall back
   safely to the active parent scope rather than a removed node.
8. Remove every listener and registry entry owned by the instance on teardown.

Verify：the public API contains no role, Escape, backdrop, domain, styling,
router, persistence or async-runtime type, and multiple instances cannot share
an identity or remove one another's cleanup.

## Phase 2：Standalone runtime fixture

1. Replace the rejected upstream behavior in
   `validation/dca-023-focus-lifecycle` only at the focus boundary; retain stable
   probe IDs and semantic markup.
2. Exercise trusted browser Tab and Shift+Tab traversal for basic and nested
   dialogs.
3. Verify only the inner scope reacts while nested and the outer scope resumes
   after inner teardown.
4. Verify initial-target preference, first-tabbable default, dynamic controls,
   no-tabbable root fallback and disconnected-opener fallback.
5. Verify closing inner then outer restores first to the outer opener and then
   to the outside opener.
6. Verify Escape behavior remains fixture-owned and no unexpected browser
   diagnostic is emitted.

Verify：every acceptance row in the validation record passes in a real browser;
pure unit tests supplement but do not replace the renderer receipt.

## Phase 3：Gentle Cards adoption

1. Apply the shared scope to the existing modal root without changing backdrop
   dismissal, close callbacks, confirmation generations, markup classes or
   styling.
2. Keep default-action choice local by marking the existing intended initial
   target.
3. Add `role`, `aria-modal` and an accessible name in Cards itself; these are
   consumer composition fixes, not crate behavior.
4. Verify initial focus, forward and reverse containment, Escape ownership and
   restoration to the recycle-bin opener in the real web application.

Verify：Cards' complete local gate and browser receipt pass against one pinned
shared Git revision.

## Phase 4：Deductree nested adoption

1. Apply independent scopes to the existing Cast Library and nested asset
   picker roots without changing their backdrop-dismiss state or catalog edits.
2. Give the nested picker a product-owned accessible name.
3. Verify trusted forward and reverse traversal stays in the picker while it is
   active, then in the Cast Library after it closes.
4. Verify inner teardown restores the asset-field opener and outer teardown
   restores the Story Toolbar opener.
5. Verify empty or temporarily disabled picker controls use the safe root
   fallback.

Verify：Deductree's complete pre-commit gate passes and a hidden desktop-WebView
probe records the nested lifecycle without activating or controlling the user's
foreground application.

## Phase 5：Handoff

1. Record the shared revision and exact consumer commits in the validation
   record.
2. Document the root binding, optional initial target and product-owned semantic
   checklist in the crate README.
3. Keep DCA-023 `Planned` until both required consumers pass runtime receipts.
4. When all completion criteria hold, move this plan to `docs/done/`, remove the
   wishlist entry and record the result in `CHANGELOG.md`.

## Completion criteria

- Basic and nested initial focus pass.
- Forward and reverse Tab stay inside only the active scope.
- Dynamic tabbables are evaluated from current rendered state.
- A scope with no tabbable descendant focuses its root.
- Inner and outer teardown restore the correct opener in order.
- A removed opener falls back without panic or focus on a detached node.
- Escape and ARIA composition remain consumer-owned.
- Gentle Cards and Deductree pass their complete automated gates and runtime
  receipts against one reviewed shared revision.
