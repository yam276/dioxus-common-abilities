# Shared Agent Workflow Candidates

狀態：evidence catalog，非強制規範

候選 lifecycle、priority、ownership 與下一個 gate 的唯一權威是
`COMMON_CAPABILITY_WISHLIST.md`。本文件的 `Promoted`／`Ready to design`／`Incubate`
只描述調查證據成熟度，不得用來取代 wishlist status。

本文件整理各產品目前仍放在 local `AGENTS.md`／`CLAUDE.md`，但具有跨 repository
潛力的工作流與要點。它不是第二份產品 roadmap，也不代表列出的工作流已經標準化。

共通 `AGENTS.md` 只放已證實且不需要產品參數的基線；本文件負責保存下一層候選、
相反案例、local 參數與升格條件。

## 分級

- `Promoted`：證據已足夠，通用原則已進入中央 `AGENTS.md`。
- `Ready to design`：至少兩個獨立 product lineage 有相同 invariant，可以設計共通
  workflow、checklist、template 或 script interface。
- `Incubate`：有實際價值，但證據只有單一 lineage、單一產品或 implementation 差異
  尚未切清楚。
- `Keep local`：共通 workflow 可存在，但列出的值、命令、格式或 policy 必須由產品
  自己擁有。

Fork siblings 只算一個 product lineage。Gentle 與 Gentle Cards 的共同 implementation
可以證明重複和演化，不能單獨證明跨產品 API 已穩定。

## 候選總覽

| Candidate | Evidence lineages | Potential shared artifact | Status |
|---|---:|---|---|
| Repository identity preflight | 6 products plus this repository | Session-start rule or hook | `Promoted` |
| Active-work document routing | 4 | Small AGENTS rule plus local pointer table | `Ready to design` |
| One quality-gate entrypoint | 6 | Script/`xtask` interface convention | `Ready to design` |
| Change-impact target matrix | 4 | Local manifest/table consumed by the gate | `Ready to design` |
| Regression proof with a negative case | 3 | Test workflow/checklist | `Promoted` |
| Persisted-data evolution checklist | 5 | Reusable checklist or agent workflow | `Ready to design` |
| Version-identity separation | 3 | Shared rule plus local version map | `Ready to design` |
| Localization completeness workflow | 4 | Checklist and test-contract template | `Ready to design` |
| Sensitive material and release exclusion | 4 | Security/release checklist | `Ready to design` |
| Release artifact smoke evidence | 3 | Parameterized smoke record template | `Ready to design` |
| Authored/generated source-of-truth declaration | 3 | Asset rule template | `Ready to design` |
| Upstream dependency iteration and request routing | 3 | Dependency workflow/checklist | `Ready to design` |
| Diagnostics/support evidence capture | 3 | Debug receipt template | `Incubate` |
| Reusable in-app live probe | 1 | Development-only Dioxus component | `Incubate` |
| Rust worktree target reuse | 1 lineage | Session hook or user-level instruction | `Incubate` |
| Shared-instruction include verification | New shared layer | Lint/check script | `Incubate` |

## 1. Repository identity preflight

**Status:** `Promoted`

**Evidence**

- `gentle/CLAUDE.md` and `gentle-cards/CLAUDE.md` already require a session-start
  `git config user.email` check.
- Read-only verification found the same expected identity in Deductree, Gentle,
  Gentle Cards, OxDM, Pedigoo, ShinyColors and this repository:
  `59956724+yam276@users.noreply.github.com`.

**Common invariant**

Do not create history under an unrelated global Git identity.

**Local values**

Repository ownership, signing keys, remotes, branch rules and publishing authority
remain local.

**Decision**

The preflight and expected email now live in the central `AGENTS.md`. A future
hook MAY automate the check, but automation must not silently rewrite user config.

## 2. Active-work document routing

**Status:** `Ready to design`

**Evidence**

- `gentle/CLAUDE.md` routes current multi-session work through root `ROADMAP.md`.
- `gentle-cards/CLAUDE.md` separates `docs/active/` from `docs/done/` and says to
  grep historical changelogs rather than read archives by default.
- `Deductree/AGENTS.md` separates Diolama active/done plans from consumer
  integration plans.
- `NewShiny/ShinyColors_diolama/CLAUDE.md` requires an active plan for cross-module,
  data-format, player-lifecycle, Canvas, asset-pack and upstream-contract work.

**Common invariant**

At session start, find the repository-declared active-work entrypoint; read active
state, not the full archive. Start a plan only when the local complexity trigger
fires, and move completed plans through the local lifecycle after verification.

**Local values**

Directory names, which tasks require plans, source-of-truth documents, archive
locations and completion gates remain local.

**Proposed artifact**

A short central rule plus a required local table:

```text
Active entrypoint: <path or none>
Read on demand: <paths and triggers>
Archive: <path>
Plan required when: <local triggers>
```

**Promotion condition**

Validate the table in Deductree and one non-Diolama product without forcing the
same directory layout.

## 3. One quality-gate entrypoint

**Status:** `Ready to design`

**Evidence**

All six products define a clean gate, but callers must currently remember different
commands, packages, features, targets and working directories.

- Deductree selects specific workspace packages.
- Gentle tests store-safe and `adult` profiles across backend and frontend.
- Gentle Cards adds a WASM target check.
- OxDM includes build and i18n parity.
- Pedigoo defines a documentation-only exception.
- ShinyColors adds locked check, Canvas tests and Windows visible smoke triggers.

**Common invariant**

There should be one discoverable command that runs the repository's complete gate
for the current change class. Exact underlying commands remain product-owned.

**Proposed artifact**

Standardize an interface, not an implementation, for example:

```text
quality-gate <code|docs|release>
```

The implementation MAY be `cargo xtask`, `just`, PowerShell, shell or a small
cross-platform binary. Do not select the tool until Windows and non-Windows
consumers are both represented.

**Promotion condition**

Two independent products expose the same entrypoint semantics, including clean
failure propagation and a docs-only path.

**2026-08-28 feasibility result**

The viable boundary is narrower than the first model: a shared convention for
explicit `docs`, `code` and `release` intent plus honest pass/fail/pending
results. The wrapper and complete commands remain consumer-owned; current
evidence does not justify a shared process runner or require `DCA-010`.
The remaining two-consumer prototype gate is recorded in
`docs/validation/DCA-009-010-quality-gate-interface.md`.

## 4. Change-impact target matrix

**Status:** `Incubate`; central catalog status is `Deferred`

**Evidence**

- Gentle changes under adult-gated surfaces require both store-safe and `adult`
  lint/test profiles.
- Gentle Cards changes under `cfg(wasm32)` require a WASM check in addition to
  native clippy.
- ShinyColors Canvas/WebView2 changes require JavaScript coverage and visible
  Windows smoke, not only Rust tests.
- OxDM smoke tests depend on a dev-only `mock-server` feature that must not enter
  release dependencies.

**Common invariant**

A default host build cannot verify code excluded by feature, target, runtime or
packaging boundaries. Changed surfaces must select every matrix arm that can own
their failure.

**Local values**

Paths, features, targets, test commands and runtime environments remain local.

**Proposed artifact**

A local machine-readable manifest or concise table mapping change predicates to
required checks. The quality-gate entrypoint consumes it; agents should not infer
the matrix from prose scattered across several files.

**Promotion condition**

First model Gentle Cards WASM and ShinyColors Canvas because they exercise
different kinds of boundaries. Reject any format that cannot express both simply.

**2026-08-28 feasibility result**

Concrete selectors failed the value/completeness gate. Cards file lists become
stale when a new `cfg(wasm32)` appears, while ShinyColors Canvas behavior has
Rust producers outside `src/canvas/**` and visible smoke is a plan milestone,
not a path property. Safe broad rules collapse into the explicit `code` gate;
arbitrary predicates merely hide product scripts behind another syntax.
The catalog therefore defers `DCA-010`; see
`docs/validation/DCA-009-010-quality-gate-interface.md`.

## 5. Regression proof with a negative case

**Status:** `Promoted`

**Evidence**

- `Pedigoo/CLAUDE.md` requires a guard to be observed red, not merely green.
- OxDM records failures caused by reversed baseline/current diff orientation and
  stale prose about dependency features.
- Deductree requires bug reproduction and verified success criteria.

**Common invariant**

A regression test should fail when the protected behavior is removed or inverted,
when practical. Empty loops, unreachable assertions and overly loose thresholds
are not evidence.

**Decision**

The central `AGENTS.md` already contains the generalized rule. Exact probes and
domain margins remain local.

## 6. Persisted-data evolution checklist

**Status:** `Ready to design`

**Evidence**

- `Deductree/docs/FileContract_V1.md` treats imported mystery files as a public,
  versioned and untrusted contract.
- `gentle/gentle-core/CLAUDE.md` and the Cards equivalent coordinate database
  migrations, Serde defaults, backups, downgrade tolerance and fixture generation.
- OxDM separates ordinary TOML configuration from keyring credentials and retains
  legacy fields for migration.
- ShinyColors owns versioned local state and Diolama pack/save compatibility
  boundaries.

**Common invariant**

Before changing persisted or exchanged data, identify every identity separately:
package version, database schema, document/wire format, backup format and migration
path. Additive evolution, unknown-field policy, old/new direction, fixtures,
resource bounds and failure UX must be considered explicitly.

**Local values**

Schemas, version numbers, codecs, compatibility promises, migration SQL, size
limits, certification rules and restore policy remain local.

**Proposed artifact**

A reusable planning checklist with conditional sections rather than one universal
serializer or format crate.

**Promotion condition**

Apply the checklist to one database migration and one file-format change. It must
help both without importing either product's terminology.

## 7. Version-identity separation

**Status:** `Ready to design`

**Evidence**

- Deductree separates Diolama package, CLI, document, wire, save, authoring, pack,
  manifest and schema identities.
- Gentle and Gentle Cards expose app, database-schema and backup-format versions
  with different bump rules.
- OxDM's export schema and dependency version evolve independently of the app.

**Common invariant**

Executable/package SemVer is not a substitute for persisted-data, wire, document,
catalog or dependency identity. Every identity needs one owner and one source of
truth.

**Proposed artifact**

A local version map template:

| Identity | Owner/source | Consumers | Bump trigger | Compatibility rule |
|---|---|---|---|---|

**Promotion condition**

Validate the map against Deductree and Gentle without merging their distinct bump
policies.

## 8. Localization completeness workflow

**Status:** `Ready to design`

**Evidence**

- Gentle and Gentle Cards require every `Strings` field in every locale TOML and
  test non-empty values.
- OxDM has exhaustive key parity across English, Traditional Chinese and Russian.
- Deductree uses typed keys and a separate runtime-override system for Story Editor.

**Common invariant**

New user-visible chrome must enter the product's localization path, preserve its
fallback policy, update every required locale resource and pass a completeness
test. Domain-authored/localized content remains separate from UI chrome.

**Local values**

Key types, locale list, resource format, fallback language, external overrides,
plural rules, domain vocabulary and font/asset policy remain local.

**Proposed artifact**

An agent checklist and test-contract examples for typed enums, typed resource
structs and string-key maps. Do not force a single i18n runtime.

**Promotion condition**

Use the same checklist for one Gentle TOML key and one OxDM or Deductree key.

## 9. Sensitive material and release exclusion

**Status:** `Ready to design`

**Evidence**

- Gentle excludes `.env` from Git and release archives.
- OxDM stores credentials in the keychain, never plaintext TOML, and separates
  credential presence metadata from secret values.
- ShinyColors keeps private art, audio, video and original captures under an
  ignored local-private asset layer while requiring the app to run without it.
- ShinyColors and OxDM both use keyring-backed runtime state.

**Common invariant**

Every project should declare which material is source, generated, private,
secret, runtime-created or distributable. Git checks and release checks are
separate; a gitignored file can still leak into a bundle.

**Local values**

Keychain services, secret schemas, asset licenses, redaction rules, environment
variables and distribution profiles remain local.

**Proposed artifact**

A sensitive-material table plus release-exclusion checklist. It must cover both
credentials and licensed/private assets without pretending they have identical
threat models.

**Promotion condition**

Validate on OxDM credentials and ShinyColors private assets, then add Gentle's
`.env`/bundle exclusion as the packaging case.

## 10. Release artifact smoke evidence

**Status:** `Ready to design`

**Evidence**

- Gentle launches a packaged executable from a fresh folder to verify embedded
  assets and formats.
- Gentle Cards additionally verifies the embedded web bundle and LAN access.
- ShinyColors records commit, WebView2 runtime, DPI, viewport, route and result
  for visible Windows smoke; unit or SSR tests may not substitute for it.

**Common invariant**

Test the artifact that users receive, outside the development tree, and record
the environment dimensions that can change the result.

**Local values**

Build commands, profiles, OS matrix, routes, private assets, network topology and
release approval remain local.

**Proposed artifact**

A smoke receipt template containing artifact identity, clean launch location,
runtime/OS, display conditions, route/scenario, expected result, actual result
and evidence path.

**Promotion condition**

One template must describe both a Gentle fresh-folder test and a ShinyColors
WebView2 visual test without making either less precise.

## 11. Authored/generated source-of-truth declaration

**Status:** `Ready to design`

**Evidence**

- Gentle lineage treats Tailwind outputs as generated and `icon.png` as the icon
  source, with derived `.icns`/`.ico` artifacts.
- OxDM's `assets/main.css` is deliberately hand-authored and tracked, with no CSS
  pipeline.
- Embedded formats, templates and locale resources in Gentle have explicit source
  directories and release behavior.

**Common invariant**

Every nontrivial artifact should say whether it is authored, generated, derived,
private or runtime-created, and name exactly one source of truth. Opposite policies
are valid; ambiguity is not.

**Proposed artifact**

A local asset manifest/table with source, derived outputs, regeneration command,
Git policy and release policy.

**Promotion condition**

The format must express both generated Gentle CSS and hand-authored OxDM CSS
without treating either as an exception.

## 12. Upstream dependency iteration and request routing

**Status:** `Ready to design`

**Evidence**

- OxDM temporarily uses a local `oxvif` path dependency, then re-pins a published
  registry version before CI; its API adapter and smoke tests verify upgrades.
- ShinyColors follows existing Diolama API, simple composition, thin host adapter,
  isolated consumer policy, then files a source-owned request instead of copying
  the runtime state machine.
- Deductree maintains the authoritative Diolama feature-request lifecycle and exact
  package identities.

**Common invariant**

Resolve dependency gaps in this order: verify the current versioned API, compose
existing primitives, add a thin consumer adapter, then file a request with the
upstream owner. Local path iteration must end in a reproducible pin before CI or
release.

**Local values**

Registry, version, feature set, request catalog, adapter ownership, release order
and compatibility gate remain local.

**Proposed artifact**

A dependency-upgrade/request checklist with before/after pin verification and a
mandatory list of consumer call sites or contract tests.

**Promotion condition**

Validate it once on Diolama and once on `oxvif`; these exercise workspace-owned
and external upstreams.

## 13. Diagnostics and support evidence capture

**Status:** `Incubate`

**Evidence**

- OxDM has rolling logs and a visible About/support surface.
- Gentle lineage installs tracing before Dioxus takes the global subscriber.
- ShinyColors installs a crash logger and records WebView2/display conditions for
  visual failures.

**Potential common invariant**

A bug report should capture executable identity, runtime/platform details,
reproduction route, bounded logs and user-visible failure without leaking secrets.

**Why not promoted**

Logging destinations, crash policy, support UX and redaction are not yet aligned.
First define a debug receipt template; do not prematurely build a common logging
crate from the agent workflow.

## 14. Reusable in-app live probe

**Status:** `Incubate`

**Evidence**

- Deductree has a development-only `DebugConsole` for selectable live GUI values
  such as coordinates, sizes and counts.
- ShinyColors requires runtime visual evidence but does not yet share that
  component.

**Potential common invariant**

Live GUI probes should be reusable, copyable, clearly development-only and fully
removed after diagnosis.

**Promotion condition**

Adopt the concept in one non-Deductree app without importing Deductree state or
styling. Until then, keep `DebugConsole` local.

## 15. Rust worktree target reuse

**Status:** `Incubate`

**Evidence**

Gentle and Gentle Cards detect linked worktrees and point `CARGO_TARGET_DIR` at
the main clone to avoid duplicating multi-gigabyte Rust targets.

**Potential common invariant**

Detect worktrees at session start and offer a safe shared target directory when
toolchain, features and lockfile make reuse valid.

**Why not promoted**

Evidence is one fork lineage, and a shared target can produce confusing cache or
toolchain interactions. This may belong in a user-level hook rather than every
repository's `AGENTS.md`.

## 16. Shared-instruction include verification

**Status:** `Incubate`

**Evidence**

The new common layer has two discovery entrypoints (`AGENTS.md` and `CLAUDE.md`),
while consumer repositories currently use different nesting and include paths.

**Potential common invariant**

CI or a local check should prove that the shared include resolves, the local file
still contains required gates and domain references, and stock tutorial text has
not become a conflicting authority.

**Promotion condition**

Migrate and inspect one sibling repository and ShinyColors' nested repository.
Only then define a linter; do not guess every tool's include semantics first.

## Suggested order

1. Migrate shared instructions into one consumer and verify include precedence.
2. Standardize the active-work pointer table.
3. Design the one-command quality-gate interface together with the impact matrix.
4. Draft the persisted-data and version-identity checklists.
5. Add i18n completeness, sensitive-material and release-smoke templates.
6. Reassess diagnostics, live probes and worktree hooks after more consumers exist.

The rule for promotion is the same as for runtime crates: centralize the invariant,
not the product's values. A successful workflow removes repeated judgment and
missed checks while leaving exact commands, domain contracts and release authority
with the repository that owns them.
