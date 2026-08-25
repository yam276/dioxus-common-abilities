# Common Capability Wishlist

狀態：唯一權威 catalog

本文件管理 `dioxus-common-abilities` 有權擁有的未完成共通能力。候選可以是
Rust crate、agent rule、workflow、template、tool 或 composition recipe。

本文件不管理任何產品 feature，也不取代 Deductree、Diolama、Gentle、Gentle Cards、
OxDM、Pedigoo 或 ShinyColors 自己的 roadmap。調查證據分別保存在
`DIOXUS_COMMONALITY_AUDIT.md` 與 `SHARED_WORKFLOW_CANDIDATES.md`；候選的狀態、優先度、
ownership 與下一個 gate 只以本文件為準。

## 1. Ownership

### Source owner

`dioxus-common-abilities` 只在能力邊界已證明跨產品時成為 source owner。Source owner
負責：

- common invariant；
- public API 或 workflow contract；
- compatibility、版本與 migration；
- common tests、templates 和 documentation；
- 接受、延後、拒絕與完成狀態。

### Consumer owner

Consumer repository 永遠負責：

- domain model 與產品 policy；
- persistence schema、實際路徑、codec、credentials 和 migration values；
- router、root context、styling、copy 和 UX composition；
- 自己的 quality gate 與 release authority；
- 採用 common capability 所需的薄 adapter。

Consumer 可以提出 gap，但不能在 local wishlist 中建立第二個 common capability
authority。Local 文件應引用本 catalog 的 `DCA-###`，並只記錄 consumer-specific
impact、adapter 與驗證結果。

### Provisional consumer request

若 consumer 在 catalog 尚無對應項目，可先使用：

```text
<CONSUMER>/DCA-REQ-###
```

例如 `SHINY/DCA-REQ-001`。這是 provisional request，不是中央 ID。Triage 後由本
catalog 分配 `DCA-###`，合併重複項，或以 domain-specific／insufficient-evidence
理由拒絕。中央 ID 不由 consumer 自行分配。

## 2. Lifecycle

```text
Observed
  -> Evidence-backed
  -> Validating
  -> Planned
  -> Done

Any unresolved state -> Deferred
Any unresolved state -> Rejected
Deferred -> Evidence-backed
```

### `Observed`

有真實 implementation、事故或需求，但目前只有一個 product lineage，或 common
invariant 尚未與 product policy 分離。

### `Evidence-backed`

至少兩個獨立 product lineages 有同一問題，或存在高風險 correctness 例外；候選已
說清楚 common invariant、local values 與下一個 validation target。

Gentle 與 Gentle Cards 是同一 lineage。它們的 fork history 可證明重複、演化和事故，
但不能單獨滿足獨立 lineage 條件。

### `Validating`

已指定 consumer、假設、validation artifact 與 acceptance criteria，正在驗證邊界。
Validation 可以是 lift-and-import、兩份實際填寫的 checklist、API spike 或相反
implementation comparison；不能只有討論。

### `Planned`

Validation 已支持該邊界，scope 與 non-goals 已接受，並在 `docs/active/` 有可機械執行
的 implementation plan。只有 `Planned` 項目可以開始正式 shared implementation。

### `Done`

Implementation 或 workflow 已通過自己的 gate 和所有指定 consumer 驗證。完成項目
從本 wishlist 移除，移到 `docs/done/`，並在 `CHANGELOG.md` 留下 ID、結果與驗證；
wishlist 不兼任 changelog。

### `Deferred` 與 `Rejected`

- `Deferred`：方向仍可能正確，但缺 consumer、時機或前置 boundary。
- `Rejected`：證據顯示應保持 product-local、已有正確 owner，或抽取成本大於價值。

兩者都必須記錄原因。`Deferred` 只有新增證據後才能回到 active lifecycle；`Rejected`
若要重開，必須建立新 ID 並引用舊決定。

## 3. Priority

- `P0`：已知 correctness／data-safety 問題，或阻塞其他共通能力。
- `P1`：重複成本與事故證據明確，有指定 validation path。
- `P2`：有價值但 evidence、boundary 或 adopter 尚不足。

Priority 不取代 lifecycle。`P0 Observed` 仍不能直接開始 shared implementation；先補足
boundary validation。

## 4. Entry requirements

每個 unresolved entry 必須包含：

- `Kind`
- `Status`
- `Priority`
- `Problem`
- `Common invariant`
- `Evidence`
- `Candidate consumers`
- `What stays local`
- `Next gate`

若新增證據只是既有 entry 的另一個 implementation，更新原 entry，不建立近義候選。
若兩個需求只有名稱相同但 failure semantics 不同，拆成不同 entry。

## 5. Active index

| ID | Candidate | Kind | Status | Priority |
|---|---|---|---|---|
| `DCA-001` | Dioxus IME composition and shortcut layer | crate | `Planned` | `P0` |
| `DCA-002` | Headless modal and toast behavior | crate | `Evidence-backed` | `P1` |
| `DCA-003` | Product preferences backend/result boundary | crate | `Evidence-backed` | `P1` |
| `DCA-004` | Product i18n locale/fallback boundary | crate | `Evidence-backed` | `P1` |
| `DCA-005` | Background task ownership/progress contract | crate/workflow | `Evidence-backed` | `P1` |
| `DCA-006` | Product diagnostics bootstrap | crate | `Evidence-backed` | `P1` |
| `DCA-007` | Dioxus app-shell paved-road recipes | documentation | `Observed` | `P2` |
| `DCA-008` | Active-work document routing | agent workflow | `Evidence-backed` | `P1` |
| `DCA-009` | One quality-gate entrypoint | tool/workflow | `Evidence-backed` | `P1` |
| `DCA-010` | Change-impact target matrix | tool/workflow | `Evidence-backed` | `P1` |
| `DCA-011` | Persisted-data evolution checklist | template/workflow | `Validating` | `P0` |
| `DCA-012` | Version-identity map | template/rule | `Evidence-backed` | `P1` |
| `DCA-013` | Localization completeness workflow | template/workflow | `Evidence-backed` | `P1` |
| `DCA-014` | Sensitive-material and release-exclusion workflow | template/workflow | `Evidence-backed` | `P1` |
| `DCA-015` | Release artifact smoke receipt | template/workflow | `Evidence-backed` | `P1` |
| `DCA-016` | Authored/generated source-of-truth declaration | template | `Evidence-backed` | `P1` |
| `DCA-017` | Upstream dependency iteration and request routing | workflow | `Evidence-backed` | `P1` |
| `DCA-018` | Diagnostics/support evidence receipt | template/workflow | `Observed` | `P2` |
| `DCA-019` | Reusable in-app live probe | crate/component | `Observed` | `P2` |
| `DCA-020` | Rust worktree target reuse | tool/workflow | `Observed` | `P2` |
| `DCA-021` | Shared-instruction include verification | tool | `Observed` | `P1` |

## 6. Candidate records

### `DCA-001` Dioxus IME composition and shortcut layer

- **Kind:** crate
- **Status:** `Planned`
- **Priority:** `P0`
- **Problem:** CJK controlled inputs and global shortcuts repeatedly implement incomplete
  composition handling; WebView2 may omit the final input event.
- **Common invariant:** intermediate composition cannot commit, cancel or trigger commands;
  final committed text must survive the WebView event sequence.
- **Evidence:** `DIOXUS_COMMONALITY_AUDIT.md` candidate `dioxus-input`; Gentle lineage has
  multiple manual guards, a later extracted `ImeGuard`, keyboard filtering and bug history.
- **Candidate consumers:** Gentle Cards first, Gentle second; OxDM or Deductree supplies
  independent-lineage validation.
- **What stays local:** form model, validation, value commit, CSS and command meaning.
- **Next gate:** verify the pinned private Git dependency in authenticated clean CI, then run
  the manual Windows WebView2 CJK matrix and validate one independent-lineage consumer. No
  `0.7.4` compatibility layer will be added.

### `DCA-002` Headless modal and toast behavior

- **Kind:** crate
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** Gentle siblings copy modal/toast primitives while Cards contains a later
  backdrop press-origin fix; other products implement similar lifecycle differently.
- **Common invariant:** backdrop, Escape/focus and toast-queue behavior should be reliable
  without owning product styling.
- **Evidence:** `DIOXUS_COMMONALITY_AUDIT.md` UI behavior finding and byte-identical Gentle
  primitives.
- **Candidate consumers:** Gentle Cards, Gentle, then OxDM or Deductree.
- **What stays local:** CSS, icons, copy, layout and modal contents.
- **Next gate:** separate behavior from Tailwind classes in one Gentle consumer and compare
  with a non-Gentle dialog.

### `DCA-003` Product preferences backend/result boundary

- **Kind:** crate
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** products repeat load/default/migrate/save/reactive lifecycle but deliberately
  use different paths, codecs, error and secret policies.
- **Common invariant:** storage backends and typed outcomes can be shared without the common
  layer owning schema or location.
- **Evidence:** Gentle/Cards JSON and localStorage, OxDM TOML/keyring, Deductree RON and
  ShinyColors versioned localStorage in the audit.
- **Candidate consumers:** Gentle lineage followed by OxDM or Deductree.
- **What stays local:** schema, path, codec, migration, autosave, secrets and settings UI.
- **Next gate:** write two API sketches representing Gentle and OxDM without product branches.

### `DCA-004` Product i18n locale/fallback boundary

- **Kind:** crate
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** every app reimplements locale identity, fallback and reactive access through
  incompatible resource/key models.
- **Common invariant:** locale and fallback semantics may be common while resource ownership
  remains with the product.
- **Evidence:** typed TOML Gentle lineage, string-key OxDM, typed Deductree/Pedigoo and runtime
  Story Editor overrides in the audit.
- **Candidate consumers:** Gentle and Gentle Cards, then OxDM.
- **What stays local:** keys, translations, domain text, resource format, plurals, fonts and
  localized assets.
- **Next gate:** unify only the Gentle locale/fallback lifecycle and test an OxDM adapter.

### `DCA-005` Background task ownership/progress contract

- **Kind:** crate/workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** detached work, component teardown and progress signals have repeatedly caused
  frozen progress or dropped-value failures.
- **Common invariant:** callers must select task lifetime and receive observable progress,
  cancellation and failure semantics.
- **Evidence:** Gentle/Cards detached import/upload flows, scope-bound counterexamples and git
  history summarized in the audit.
- **Candidate consumers:** one detached Gentle/Cards import and one component-scoped flow.
- **What stays local:** business operation, retry, network protocol and result UI.
- **Next gate:** express the two opposing lifetimes as acceptance tests before choosing crate
  versus workflow ownership.

### `DCA-006` Product diagnostics bootstrap

- **Kind:** crate
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** tracing/panic installation order and guard lifetime are repeated, while support
  UI and destinations differ.
- **Common invariant:** explicit installation must keep guards alive and surface log/support
  locations without stealing product error policy.
- **Evidence:** Diolama crash logger, ShinyColors host installation, OxDM rolling logs and
  Gentle tracing bootstrap in the audit.
- **Candidate consumers:** OxDM plus Gentle or ShinyColors.
- **What stays local:** redaction, telemetry, destinations, error taxonomy and recovery UX.
- **Next gate:** compare install order and lifetime only; do not combine with `DCA-018` yet.

### `DCA-007` Dioxus app-shell paved-road recipes

- **Kind:** documentation
- **Status:** `Observed`
- **Priority:** `P2`
- **Problem:** entrypoints repeat settings, tracing, window, backend and context setup, but
  current products have materially different startup constraints.
- **Common invariant:** a recipe may document safe ordering without owning the app.
- **Evidence:** near-copy Gentle shells plus product-specific Deductree, OxDM, Pedigoo and
  ShinyColors shells in the audit.
- **Candidate consumers:** the next new Dioxus app.
- **What stays local:** window policy, router, contexts, runtime, backend and asset protocols.
- **Next gate:** wait until at least three lower-level abilities are validated and repeatedly
  composed; no app-builder crate.

### `DCA-008` Active-work document routing

- **Kind:** agent workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** agents need current plans without loading large historical archives or inventing
  a second roadmap.
- **Common invariant:** each repo declares an active entrypoint, read-on-demand references,
  archive and plan trigger.
- **Evidence:** section 2 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Deductree and one non-Diolama product.
- **What stays local:** paths, triggers, authority documents and completion gates.
- **Next gate:** test the proposed four-field pointer table in two repositories.

### `DCA-009` One quality-gate entrypoint

- **Kind:** tool/workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** every repository has a gate, but agents must reconstruct different commands and
  change classes from prose.
- **Common invariant:** one discoverable entrypoint runs the complete local gate and propagates
  failures.
- **Evidence:** section 3 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Gentle Cards and ShinyColors first because their gates differ most.
- **What stays local:** commands, packages, platforms, features and packaging policy.
- **Next gate:** prototype interface semantics together with `DCA-010`; do not select shell,
  PowerShell, `just` or `xtask` in advance.

### `DCA-010` Change-impact target matrix

- **Kind:** tool/workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** host/default builds miss feature-, target-, Canvas- and release-only code.
- **Common invariant:** changed surfaces select every matrix arm capable of owning their
  failure.
- **Evidence:** section 4 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Gentle Cards WASM and ShinyColors Canvas; Gentle feature profiles
  and OxDM dev-only mocks follow.
- **What stays local:** predicates, targets, features, environments and commands.
- **Next gate:** model the first two consumers with no product-specific fields in the shared
  schema; pair validation with `DCA-009`.

### `DCA-011` Persisted-data evolution checklist

- **Kind:** template/workflow
- **Status:** `Validating`
- **Priority:** `P0`
- **Problem:** apps own different public files, databases, backups, settings and save data, but
  every change must reason about identity, compatibility, migration, untrusted input and
  failure UX.
- **Common invariant:** a change cannot proceed until every persisted identity and both
  compatibility directions are explicit and verified proportionately.
- **Evidence:** section 6 of `SHARED_WORKFLOW_CANDIDATES.md` and the audit's settings/domain
  boundaries.
- **Candidate consumers:** Deductree file contract and Gentle database/backup contract first;
  OxDM config/keyring is the opposing third case.
- **What stays local:** schema, version values, codecs, SQL, limits, certification and restore
  promises.
- **Next gate:** retrospective validation in Deductree, Gentle and OxDM is complete. Use
  `templates/PERSISTED_DATA_CHANGE_CHECKLIST.md` prospectively before the next real persisted
  contract change; promote to `Planned` only if it produces executable acceptance criteria
  without product-specific template fields.

### `DCA-012` Version-identity map

- **Kind:** template/rule
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** package version is repeatedly confused with database, backup, document, wire,
  request-catalog or dependency identity.
- **Common invariant:** every version identity has one owner, source, consumer set and bump
  trigger.
- **Evidence:** section 7 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Deductree and Gentle.
- **What stays local:** version values, SemVer policy, compatibility promises and tags.
- **Next gate:** validate the version-map section inside `DCA-011`; split it only if it proves
  independently useful.

### `DCA-013` Localization completeness workflow

- **Kind:** template/workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** adding UI text requires different resource updates but always risks missing
  locales, fallback or domain/chrome separation.
- **Common invariant:** user-visible chrome enters the localization path, updates required
  resources and passes completeness tests.
- **Evidence:** section 8 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Gentle and OxDM or Deductree.
- **What stays local:** keys, locale list, resource format, fallback and domain vocabulary.
- **Next gate:** use one checklist on a typed TOML key and a string/enum-key product.

### `DCA-014` Sensitive-material and release-exclusion workflow

- **Kind:** template/workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** gitignored secrets/private assets may still leak into packages; ordinary config,
  keyring secrets and licensed assets have different boundaries.
- **Common invariant:** source, generated, private, secret, runtime and distributable material
  are declared and checked independently for Git and release.
- **Evidence:** section 9 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** OxDM credentials, ShinyColors private assets and Gentle `.env`.
- **What stays local:** threat model, keychain service, licenses, redaction and profiles.
- **Next gate:** draft one table that represents all three without equating their policies.

### `DCA-015` Release artifact smoke receipt

- **Kind:** template/workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** unit and development-tree checks do not prove packaged assets, runtime or display
  behavior.
- **Common invariant:** test the user-delivered artifact outside the development tree and record
  environment-sensitive evidence.
- **Evidence:** section 10 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Gentle fresh-folder package and ShinyColors WebView2 visible smoke.
- **What stays local:** build commands, profiles, OS, routes and approval.
- **Next gate:** fill one shared receipt template for both scenarios without weakening either.

### `DCA-016` Authored/generated source-of-truth declaration

- **Kind:** template
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** generated and hand-authored assets use opposing Git rules and stale instructions
  cause accidental edits or missing artifacts.
- **Common invariant:** every nontrivial artifact has one declared authority, derivation method,
  Git policy and release policy.
- **Evidence:** section 11 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** generated Gentle CSS/icons and hand-authored OxDM CSS.
- **What stays local:** tools, paths, exact artifacts and licenses.
- **Next gate:** validate a manifest/table against both opposing cases.

### `DCA-017` Upstream dependency iteration and request routing

- **Kind:** workflow
- **Status:** `Evidence-backed`
- **Priority:** `P1`
- **Problem:** consumers may copy upstream state machines, leave local path dependencies or
  document stale dependency shapes instead of routing gaps to the correct owner.
- **Common invariant:** verify current API, compose, adapt thinly, then request upstream; restore
  a reproducible pin before CI/release.
- **Evidence:** section 12 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** Diolama/ShinyColors and `oxvif`/OxDM.
- **What stays local:** source catalog, version, features, registry and compatibility checks.
- **Next gate:** apply one checklist to the next real Diolama and `oxvif` upgrade/request.

### `DCA-018` Diagnostics/support evidence receipt

- **Kind:** template/workflow
- **Status:** `Observed`
- **Priority:** `P2`
- **Problem:** logs, crash reports and GUI runtime conditions are collected differently and may
  omit identity or leak secrets.
- **Common invariant:** support evidence is bounded, reproducible and redacted.
- **Evidence:** section 13 of `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** OxDM and ShinyColors, with Gentle tracing as install-order evidence.
- **What stays local:** destinations, telemetry, redaction and support UX.
- **Next gate:** collect two real debug receipts before defining common fields.

### `DCA-019` Reusable in-app live probe

- **Kind:** crate/component
- **Status:** `Observed`
- **Priority:** `P2`
- **Problem:** WebView-only values invite one-off debug overlays that are hard to copy and easy
  to leave behind.
- **Common invariant:** a probe is selectable/copyable, development-only and removed after use.
- **Evidence:** Deductree `DebugConsole`; section 14 of
  `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** one non-Deductree GUI diagnosis, preferably ShinyColors.
- **What stays local:** styling, values and mount location.
- **Next gate:** validate a host-neutral component without importing Deductree state.

### `DCA-020` Rust worktree target reuse

- **Kind:** tool/workflow
- **Status:** `Observed`
- **Priority:** `P2`
- **Problem:** linked Rust worktrees may duplicate multi-gigabyte target directories.
- **Common invariant:** detect worktree state and offer safe reuse only when build identities are
  compatible.
- **Evidence:** Gentle lineage session hooks; section 15 of
  `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** an independent workspace with active linked worktrees.
- **What stays local:** target path, toolchain, features and cache isolation.
- **Next gate:** measure disk/time benefit and cache failure risk outside Gentle lineage; decide
  whether this belongs at user level instead.

### `DCA-021` Shared-instruction include verification

- **Kind:** tool
- **Status:** `Observed`
- **Priority:** `P1`
- **Problem:** Codex and Claude entrypoints, nested repos and relative includes may load different
  or stale authorities.
- **Common invariant:** shared include resolution and required local sections are observable.
- **Evidence:** the new `AGENTS.md`, `CLAUDE.md`, adoption guide and section 16 of
  `SHARED_WORKFLOW_CANDIDATES.md`.
- **Candidate consumers:** one sibling repository and nested ShinyColors.
- **What stays local:** local file contents, precedence exceptions and tool availability.
- **Next gate:** migrate those two shapes manually and inspect loaded instructions before writing
  a linter.

## 7. Triage rules

When reviewing a request:

1. Search by invariant and failure mode, not only by proposed name.
2. Add evidence to an existing `DCA-###` when the invariant matches.
3. Split the entry when consumers require different lifecycle or failure semantics.
4. Reject domain features and route them to the owning product or domain platform.
5. Prefer validation by opposing implementations, not only fork siblings.
6. Do not move an entry to `Planned` without a completed validation record.
7. Do not mark an entry `Done` while a required consumer still carries the replaced duplicate.

## 8. Current focus

- **Active validation:** `DCA-001` Cards and Gentle use one pinned private Git revision；
  authenticated CI, manual CJK and independent-lineage validation are next.
- **Prospective workflow validation:** `DCA-011` on the next real persisted-data change.
- **Next tool candidate after governance validation:** `DCA-009` plus `DCA-010`.

Only one focus changes status at a time. New evidence may be recorded for other entries without
silently starting their implementation.
