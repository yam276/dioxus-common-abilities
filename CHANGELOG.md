# Changelog

This file records completed common capabilities and governance changes. Open
candidate status belongs only in `COMMON_CAPABILITY_WISHLIST.md`.

## Unreleased

### Runtime crates

- add `dioxus-focus-scope` on Dioxus 0.7.9 with consumer-owned root markup,
  dynamic Tab containment, nested-scope activation and safe focus restoration
- cover scope identity and JavaScript boundary generation with five tests while
  keeping dialog semantics, Escape, backdrop and domain callbacks outside the crate
- add the renderer-neutral `dioxus-backdrop-dismiss` pointer state with
  same-pointer, content-release, cancellation and simultaneous-pointer coverage
- add the initial `dioxus-input` crate on Dioxus 0.7.9 with a non-reactive
  `ImeGuard`, WebView2 composition-end recovery and two-source keyboard suppression
- cover the pure composition state with five tests and keep renderer, router,
  launch and logger dependencies outside the crate

### Governance

- establish the shared `AGENTS.md` baseline and `CLAUDE.md` compatibility entrypoint
- centralize the verified `yam276` repository identity preflight
- centralize regression-proof, Rust, Dioxus and quality-gate behavioral rules
- add the authoritative common capability wishlist and consumer request ownership
- catalog cross-product workflow evidence separately from lifecycle decisions

### Validation

- replace the rejected first-party focus behavior in the standalone DCA-023
  fixture and pass trusted browser checks for basic, nested, dynamic, empty,
  disabled-target and disconnected-opener lifecycles
- complete DCA-023 adoption against reviewed revision `acd3e51` in Gentle Cards
  commit `54fc8d4` and Deductree commit `c98f0f3`, preserving consumer-owned
  dialog, Escape, backdrop and domain policy while passing both product gates
  and runtime receipts
- split `DCA-002` into backdrop gesture, toast queue and modal focus contracts,
  then validate and implement only the renderer-neutral backdrop pointer state
- adopt the same-pointer backdrop truth table in Cards, Gentle and
  Deductree/Diolama while preserving OxDM's already-safe close-on-down policy
- complete DCA-002 with hidden desktop-WebView pointer matrices on Gentle Cards'
  shared modal and Deductree's journal overlay, covering both cross-boundary
  directions, cancellation, mismatched pointers and same-pointer dismissal
- accept `DCA-001` for planning on Dioxus 0.7.9 after comparing Cards' extracted
  guard, its label-editor near-copy, Gentle's manual guards and Dioxus keyboard events
- migrate Cards' canonical guard and label-editor near-copy to `dioxus-input`, preserving
  caller-owned value and blur semantics while recording distribution and manual-test gates
- upgrade Gentle to Dioxus 0.7.9 and migrate five manual guards to `dioxus-input`, with the
  app quality gate, desktop bundle and launch checks passing
- pin Cards and Gentle to one private Git revision and verify locked dependency resolution,
  desktop bundles and launch smoke tests without sibling repository checkouts
- authenticate both consumer workflows with one read-only deploy key and verify their complete
  clean-runner quality gates pass
- start `DCA-011` validation with a product-neutral persisted-data checklist prototype
- validate the prototype against Deductree exact-version/additive-file behavior and
  Gentle additive-schema/structural-backup migration behavior
- separate raw-input, archive-expansion and parsed-cardinality bounds after consumer evidence
- validate the opposing OxDM config/keyring case and add secret-migration ordering plus
  observable write-outcome requirements
