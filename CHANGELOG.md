# Changelog

This file records completed common capabilities and governance changes. Open
candidate status belongs only in `COMMON_CAPABILITY_WISHLIST.md`.

## Unreleased

### Runtime crates

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

- accept `DCA-001` for planning on Dioxus 0.7.9 after comparing Cards' extracted
  guard, its label-editor near-copy, Gentle's manual guards and Dioxus keyboard events
- migrate Cards' canonical guard and label-editor near-copy to `dioxus-input`, preserving
  caller-owned value and blur semantics while recording distribution and manual-test gates
- upgrade Gentle to Dioxus 0.7.9 and migrate five manual guards to `dioxus-input`, with the
  app quality gate, desktop bundle and launch checks passing
- pin Cards and Gentle to one private Git revision and verify locked dependency resolution,
  desktop bundles and launch smoke tests without sibling repository checkouts
- start `DCA-011` validation with a product-neutral persisted-data checklist prototype
- validate the prototype against Deductree exact-version/additive-file behavior and
  Gentle additive-schema/structural-backup migration behavior
- separate raw-input, archive-expansion and parsed-cardinality bounds after consumer evidence
- validate the opposing OxDM config/keyring case and add secret-migration ordering plus
  observable write-outcome requirements
