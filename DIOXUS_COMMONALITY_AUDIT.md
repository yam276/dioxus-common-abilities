# Dioxus Apps 共通性審計：Internal Product Platform 候選分析

審計日期：2026-08-25

狀態：analysis-only research。本文是跨 repository 的調查證據，不是 Deductree Story Editor 的第二份 wishlist／roadmap，也不授權任何實作或搬移。

## Scope 與方法

本次掃描以 `/Users/ladesine/Documents/Github` 為共同路徑基準，涵蓋目前 workspace roots 中六個 Dioxus 產品，以及它們直接相關的 Rust crates：

- `Deductree`
- `gentle`
- `gentle-cards`
- `oxdm`
- `Pedigoo`
- `NewShiny/ShinyColors_diolama`

主要證據來自 Cargo manifests、Dioxus entrypoints、i18n／settings／persistence／input／window／task／diagnostics／UI source，以及 git history。本文引用的路徑都相對於 `/Users/ladesine/Documents/Github`。

這是靜態審計，沒有啟動 GUI 做新的 CJK IME、window、mobile 或 failure-path 手動驗收。凡是原始碼只顯示「可能暴露於問題」而沒有 bug history 或 workaround 證據，本文都不把它寫成已證實 bug。

評分中的 `Coupling Risk` 是反向指標：`5` 代表抽取後很容易綁死 consumer，`1` 代表風險低；其他指標的 `5` 都代表候選更強。

## 1. Executive Summary

目前已經有「Dioxus internal product platform」的自然雛形，但它是**分散的 product lineage 與 paved-road fragments**，還不是一個適合一次搬進 toolbox 的完整 framework。

最有力的三組證據是：

1. `gentle-app` 與 `gentle-cards-app` 有共同 fork 血統：settings、locale lifecycle、desktop/mobile bootstrap，以及四個逐 byte 相同的 UI primitives。這是實質 copy/evolution，不只是概念相似。
2. IME correctness 已經歷至少三輪：多個局部 composition flags、WebView2 `compositionend` workaround、再抽成 `ImeGuard`；同時 global shortcut 又各自重做 `isComposing`／field exclusion。
3. Diolama 已展示正確的「generic base + consumer composition」：`AssetProtocol` 被 Deductree Story Editor 與 ShinyColors consumer 使用，`install_crash_logger` 也被 ShinyColors 使用。這證明作者已自然形成 reusable infrastructure 的做法，但這兩項目前仍只有 VN-family 證據，不應因此搬成全產品 dependency。

最值得依序處理的能力是：

1. **`dioxus-input`：IME composition 與 shortcut suppression semantics** — `Extract now`。
2. **Backdrop、modal focus 與 toast lifecycle** — 後續 source validation 證明是三個
   failure contracts；分別記為 `DCA-002`、`DCA-023` 與 `DCA-022`，不可先做成一個 UI crate。
3. **Product preferences lifecycle** — app owns schema，platform owns backend/result semantics；`Prepare boundary, extract later`。
4. **Product i18n lifecycle** — 先統一 Gentle siblings，不強迫 Pedigoo/Deductree domain translators；`Prepare boundary, extract later`。
5. **Background task ownership與progress contract** — 先把 scope-bound/detached 規則變成顯式 policy；`Prepare boundary, extract later`。
6. **Diagnostics bootstrap** — panic/tracing/log-path/support surface；`Prepare boundary, extract later`。
7. **Dioxus app shell** — 現在只適合 recipes 與小 helper，不適合 shared framework；`Keep duplicated for now`。

現在不適合抽的東西包括 ONVIF operations、album/tag model、card-table state、Pedigoo simulation、Visual Novel runtime、StoryDoc、asset/cast policy、各產品 import/export format、updater，以及整套 theme skin。它們或是 domain core，或只有一個 consumer，或需求差異仍大。

如果今天只允許抽一個 crate，應抽 **`dioxus-input`**。它的 API 可以維持約一個 hook／小 state object，不需要 schema、router、CSS、async runtime、domain type 或 dependency injection。

## 2. Application Inventory

### 2.1 Dioxus apps

| Name | Type | Domain | Target | Dioxus | Notes |
|---|---|---|---|---|---|
| `deductree-app` | Product app | 推理遊戲、case editor、Story Editor | Desktop | `0.7`, desktop | `Deductree/app/src/main.rs`; consumes `deductree-core` and Diolama authoring/player APIs. |
| `gentle-app` | Product app, excluded from root Cargo workspace because it is built by `dx` | 漫畫／album-tags content manager | Web, desktop, mobile/iOS | pinned `0.7.4`, router | `gentle/gentle-app/src/main.rs`; embedded native backend plus WASM frontend path. |
| `gentle-cards-app` | Product app, excluded from root Cargo workspace because it is built by `dx` | 卡牌／桌遊 content and live play | Web, desktop, mobile/iOS | pinned `0.7.9`, router | Fork/evolution of Gentle frontend; adds Linux native core dependency and network/play surfaces. |
| `oxvif-device-manager` (`oxdm`) | Product app | ONVIF camera management | Desktop | pinned `0.7.9`, desktop/router | Single-crate app over external `oxvif`; mature tracing, keyring and desktop settings. |
| `pedigoo-app` | Product app | Breeding/racing simulation | Desktop | `0.7`, desktop | Large context-signal app; source of the Dioxus patterns later documented in Deductree. |
| `ShinyColors_diolama` | Product/port app | Visual Novel／album port | Desktop | `0.7`, desktop | Consumer of `diolama`, `diolama-spine`, asset packs, keyring; extensive canvas/runtime state machines. |

### 2.2 Relevant libraries and cores

| Name | Type | Domain / responsibility | Dioxus dependency | Platform relevance |
|---|---|---|---|---|
| `deductree-core` | Domain core | Mystery contract, checker, save model | None | Must remain framework-independent. |
| `gentle-core` | Domain/backend core | Album/tag data, SQLite/server, migrations | None | Owns backend and tracing initialization used by `gentle-app`. |
| `gentle-cards-core` | Domain/backend core | Cards/boards/live sessions, SQLite/server | None | Forked lineage; owns backend and tracing used by cards app. |
| `pedigoo-core` | Domain core | Simulation/world persistence | None | Must remain framework-independent. |
| `oxvif` | External domain core | ONVIF protocol/device model | None | Not part of this extraction scope. |
| `diolama` | Reusable Dioxus library | VN runtime, authoring, reusable screens, desktop asset transport | Yes; desktop optional | Already a successful domain platform. Do not make unrelated products depend on it. |
| `diolama-assets` | Library | Asset source abstraction | None | Generic-looking, but current evidence is VN-family only. |
| `diolama-pack` / `dio-pack` | Library/tool | Versioned asset pack format | None | Persisted-data/domain distribution contract, not generic app settings. |
| Shiny runtime/album modules | Product-owned composition | Port-specific translation, canvas, album and assets | Yes | App-specific adapters over Diolama; not a generic core. |

### 2.3 Relevant dependency observations

- No app uses Fluent, ICU or another common localization runtime. Every product owns an i18n implementation.
- `serde`, `toml`, `ron`, `tokio`, `tracing`, `reqwest` and `dirs` only count as evidence where repeated policy is present; their mere use is not treated as a platform candidate.
- Dioxus version skew already exists (`0.7.4`, `0.7.9`, and `^0.7`). Any shared Dioxus crate must define a deliberate compatibility policy instead of silently forcing upgrades.
- Gentle apps deliberately live outside their root Cargo workspaces. A new shared crate must therefore be referenced by path/workspace-independent manifests or placed in a separate repository with explicit versioning.

## 3. Capability Matrix

Legend: `Strong` = mature implementation with explicit policy; `Partial` = some lifecycle exists; `Risk` = relevant surface exists but lacks the guard seen elsewhere; `None` = no evidence found.

| Capability | Deductree | Gentle | Gentle Cards | OxDM | Pedigoo | ShinyColors | Cross-app similarity |
|---|---|---|---|---|---|---|---|
| i18n / locale | Strong: typed `Key`; separate Story Editor override map | Strong: typed TOML bundle, detection, persistence | Strong: same lineage + named template helper | Strong: string-key modules, English fallback, tests, persistence | Strong: typed enum + many domain helpers, session choice | Partial: localized VN content and session ADV language; little app-chrome i18n | Same lifecycle, four incompatible APIs |
| Settings / preferences | Partial: Diolama settings RON, controlled signal; StoryDoc is separate | Strong: `AppSettings`, JSON/localStorage, window/theme/locale | Strong: forked `AppSettings` with more card prefs | Strong: TOML config + keyring split + reactive autosave | Partial: startup choices and game save, no app prefs store | Partial: bounded/versioned unit-filter localStorage | High in Gentle pair; medium conceptually |
| IME / composition | Risk: editor shortcut JS skips fields but not `e.isComposing` | Strong but duplicated manual guards | Strong: reusable `ImeGuard`, one manual near-copy, global shortcut filter | Risk: Enter-driven controlled inputs, no composition guard | None found | None found | Very high Gentle lineage; high-value exception to Rule of Three |
| App bootstrap / root contexts | Strong, product-specific | Strong multi-target shell | Near-copy of Gentle shell | Strong desktop shell | Strong desktop/QA shell | Strong desktop crash/debug shell | High only inside Gentle lineage |
| Modal / dialog behavior | Several app/Diolama overlays | `Modal` | Evolved `Modal` fixes press-origin bug | `DialogOverlay` | Local overlays | Product error/modal state machines | Medium behavior; low styling similarity outside Gentle |
| Toast / notification | Context queue + timer | Global typed queue | Byte-identical queue | Context queue + typed levels | Notice signal | Visible diagnostics/error shells | High behavior, divergent ownership/styling |
| Background task ownership | Component tasks + Diolama lifecycle state machines | Multiple detached uploads and root progress | Same plus explicit scope-bound exceptions and fixes | Component-scoped network tasks | Timer/component tasks | Explicit runtime/session state machines | High pain in Gentle lineage, different products elsewhere |
| Diagnostics / logging | Diolama panic hook exists but host does not install it | Core tracing to file | Forked core tracing to file | Stderr + optional rolling logs + About surface | Mostly stderr/QA | Installs Diolama crash logger + bounded error shell | Medium; policy not unified |
| App directories / storage | `dirs::data_dir()/deductree` | Portable/executable-adjacent JSON/data; localStorage on web | Same lineage | `~/.oxdm`, TOML + keyring | Game save contract | Asset profiles/keyring plus localStorage record | Deliberately divergent product policy |
| Window / desktop integration | 16:9 correction and asset protocols | Restored size, platform menu, iOS WKWebView fix | Near-copy + context-menu suppressor | icon, min size, no menu/context menu | QA window focus/on-top behavior | crash log, debug port, window bounds | Helper-level only |
| Theme | Diolama preset + app CSS | String setting + reactive context | Same lineage + card CSS vars | Typed enum + persisted CSS class | Local dark display toggle | Product canvas art direction | Visually and semantically divergent |
| Import/export | `.dtpack`, case/story persistence | backup/restore, CBZ/import flows | packs, matches, backup/restore | JSON/JUnit/snapshot exports | game save/load | asset pack/portable release | Domain formats should stay separate |
| Updater/version handling | Diolama package/CLI versions, no app updater | Update-check endpoint/UI | No matching updater surface found | Release metadata only | None found | Build/runtime identity contracts | Not a shared candidate yet |
| Keyboard command handling | Pure command catalog + two JS bridges | Ad hoc inputs/menus | Canvas JS bridge with field + IME suppression | Root signal dispatcher + local modal Escape | Local UI handlers | Session intents | Conceptual duplication, API not stable |

## 4. Detailed Findings

### Candidate: `dioxus-input` composition and shortcut semantics

**Category:** Dioxus Infrastructure

**Evidence**

- `gentle/gentle-app/src/pages/gallery_list.rs` — manual `is_composing` state for live search.
- `gentle/gentle-app/src/pages/create_work/tag_section.rs` — manual composition guard for tag input.
- `gentle/gentle-app/src/components/dynamic_form.rs` — `TextFieldInput` and chip input each own a manual guard.
- `gentle/gentle-app/src/components/work_links.rs` — another controlled-input composition guard.
- `gentle-cards/gentle-cards-app/src/components/primitives/use_ime.rs` — `ImeGuard`, `use_ime_guard`.
- `gentle-cards/gentle-cards-app/src/components/dynamic_form.rs` — two consumers of `ImeGuard`.
- `gentle-cards/gentle-cards-app/src/pages/decks_list.rs` — another `ImeGuard` consumer.
- `gentle-cards/gentle-cards-app/src/components/arrow_layer.rs` — `LabelInput`, a second hand-written implementation with Enter/Escape/blur semantics.
- `gentle-cards/gentle-cards-app/src/components/play_canvas/keyboard.rs` — `KEYDOWN_JS`, which separately checks focused fields and `e.isComposing`.
- `Deductree/app/src/story_editor/mod.rs` and `Deductree/app/src/story_editor/map.rs` — two global key bridges that filter text fields but do not share the cards app's `e.isComposing` policy.
- `oxdm/src/views/ptz.rs` — preset name controlled input commits on Enter without an explicit composition guard.
- `oxdm/src/components/device_panel.rs` — profile name commits/cancels on Enter/Escape without an explicit composition guard.
- Git history: Gentle commit `f9a531e` added guards to three live-suggest inputs; `b35f28c` added another form-field implementation; shared-history commit `3416d07` added the arrow-label version; `3ce19e3` finally extracted `use_ime`; cards commits retain and extend it after fork.

**Common behavior**

- Ignore intermediate `oninput` updates while an IME composition is active.
- Read the flag through `Signal::peek()` so composition state does not itself cause a controlled-input rerender.
- Read final data directly from `compositionend` because WebView2 may omit the following `input` event.
- Suppress Enter/Escape and global shortcuts while composing, so candidate selection does not submit, cancel, undo or unmount the editor.
- Keep commit ownership with the caller: a text field may replace, append, create a chip, or call a domain callback.

**Differences**

- Parent-controlled fields append the final composed fragment to a prop snapshot; local fields append to their signal.
- `LabelInput` also owns blur commit and Enter/Escape semantics.
- Canvas/global shortcut listeners operate in JavaScript and need `KeyboardEvent.isComposing`; input hooks operate on Dioxus composition events.
- Other apps expose relevant inputs but have no confirmed reproduction history. They are validation targets, not proof of current failure.

**Why duplication exists**

This is a Dioxus/WebView integration issue learned incrementally. Initial bug fixes were local, the Gentle fork copied them, and only one branch later created a small hook. Shortcut bridges evolved separately from controlled-input hooks.

**Proposed stable boundary**

Own only composition state and the two proven platform rules. Do not build a full input component or form framework. A sibling pure helper may expose the global-shortcut predicate (`is field` or `is composing`) for generated JS.

**What must remain app-specific**

- Value ownership and how a composed fragment is applied.
- Validation, typeahead queries, chip creation and form submission.
- Blur policy, Enter/Escape callbacks and visual input component.
- Which global commands exist.

**Possible API shape**

```rust
#[derive(Clone, Copy)]
pub struct CompositionGuard {
    composing: Signal<bool>,
}

pub fn use_composition_guard() -> CompositionGuard;

impl CompositionGuard {
    pub fn is_composing(self) -> bool;
    pub fn start(&mut self);
    pub fn finish(&mut self, event: Event<CompositionData>) -> Option<String>;
    pub fn allows_command(self) -> bool;
}

pub const GLOBAL_SHORTCUT_GUARD_JS: &str;
```

`GLOBAL_SHORTCUT_GUARD_JS` should remain a tiny predicate/snippet, not a registry or JS event framework.

**Score**

| Metric | Score |
|---|---:|
| Repetition | 5/5 |
| Stability | 5/5 |
| Pain | 5/5 |
| Leverage | 4/5 |
| Coupling Risk | 1/5 |
| Domain Independence | 5/5 |

**Recommendation:** `Extract now`

### Candidate: product i18n lifecycle

**Category:** Product Infrastructure with a Dioxus adapter

**Evidence**

- `Deductree/app/src/i18n.rs` — `Lang`, exhaustive `Key`, `t`, `m` for UI chrome.
- `Deductree/app/src/story_editor/i18n.rs` — `BUILTIN`, `load_overrides`, `flatten`, `tr`; runtime `<data>/deductree/lang/<tag>.toml` override/addition.
- `Deductree/app/src/main.rs` — reactive `Ui.lang`, defaulting to `ZhTw` but not persisted.
- `Deductree/app/src/scene_host.rs` — language-neutral `diolama::Localized` labels, late resolution and fallback.
- `Pedigoo/app/src/i18n.rs` — `Lang`, `Key`, `t`, plus many domain enum/value translators and formatted helpers.
- `gentle/gentle-app/src/i18n.rs` — `Locale = String`, `AVAILABLE_LOCALES`, typed `Strings`, compile-time `include_str!` TOMLs, `strings`, `detect_locale`.
- `gentle-cards/gentle-cards-app/src/i18n.rs` — same lineage, larger `Strings`, `apply_template` for named interpolation.
- `gentle/gentle-app/src/utils.rs` and `gentle-cards/gentle-cards-app/src/utils.rs` — locale persistence in `AppSettings` and target-specific stores.
- `gentle/gentle-app/src/main.rs` and cards equivalent — saved-locale validation, environment detection fallback, `Signal<Locale>` context.
- `oxdm/src/i18n/mod.rs` — `t(Locale, &str)` with English fallback and release missing-key behavior.
- `oxdm/src/i18n/{en,zh_tw,ru}.rs` — separate string-key match tables.
- `oxdm/src/tests/i18n_tests.rs` — explicit per-locale key coverage.
- `NewShiny/ShinyColors_diolama/src/canvas/shell.rs` — `AdvDisplayLanguage` for localized VN content, kept in session state by `canvas/album_shell/session.rs`.

**Common behavior**

- A current locale lives in reactive Dioxus state/context.
- Static/bundled translations have a deterministic fallback language.
- Locale change rerenders UI or late-resolves `Localized` content without rebuilding domain data.
- Product content/domain enums are translated near their owning domain rather than forced into the UI runtime.
- Missing translations must not blank or panic the release UI.

**Differences**

- Resource shape: Rust enum matches, typed TOML-to-struct, string-key modules, dynamic TOML map, or `Localized` content values.
- Locale identity: closed Rust enum versus extensible string/BCP-47 tag.
- Persistence: Gentle and OxDM persist; Deductree/Pedigoo/Shiny language is currently session/default driven.
- Validation: exhaustive compiler checks, serde parse tests, manual key-set tests, or fallback-to-key.
- Interpolation is ad hoc (`format!`, `.replace`) except for cards' `apply_template`; no app has plural rules.
- Deductree distinguishes UI chrome, case content, Story Editor overrides and Diolama late resolution. A single translator object must not collapse these authorities.
- Fonts/assets are product-owned. Shiny has language-specific visual assets; the other apps mostly do not.

**Why duplication exists**

Pedigoo's closed enum approach was deliberately copied into Deductree, as recorded in `Deductree/docs/DioxusPatterns_V1.md`. Gentle later needed web/native locale detection, three bundled languages and data-driven resources. OxDM independently optimized for a small desktop string table and tests. Deductree Story Editor later needed runtime translator overrides. Each new need changed the resource model, so no version is a strict superset.

**Proposed stable boundary**

The stable seam is narrower than a translation engine:

- canonical `LocaleId`/BCP-47 normalization;
- explicit active locale + fallback chain;
- a `Translator` contract returning owned/borrowed text without blanking;
- Dioxus context/hook that reacts to locale changes;
- optional integration with an app-owned preferences store.

Resource representation, compile-time validation and plural/message formatting are not stable enough to standardize yet. The first shared implementation should serve only Gentle and Gentle Cards, where the typed TOML model is already the same pattern.

**What must remain app-specific**

- Translation keys and `Strings` schema.
- Pedigoo/Deductree domain enum rendering.
- Mystery/Story localization authority and Diolama `Localized` resolution.
- Runtime external override policy.
- Fonts, language-specific art and voice selection.

**Possible API shape**

```rust
pub struct LocaleId(String);

pub trait Translator {
    fn text(&self, locale: &LocaleId, key: &str) -> Cow<'static, str>;
    fn fallback(&self) -> &LocaleId;
}

pub struct LocaleState {
    pub active: Signal<LocaleId>,
    pub translator: &'static dyn Translator,
}

pub fn use_locale() -> LocaleState;
```

This sketch intentionally does not choose TOML, Fluent, code generation, plural syntax or dynamic registries.

**Score**

| Metric | Score |
|---|---:|
| Repetition | 5/5 |
| Stability | 2/5 |
| Pain | 4/5 |
| Leverage | 5/5 |
| Coupling Risk | 4/5 |
| Domain Independence | 5/5 |

**Recommendation:** `Prepare boundary, extract later`

Immediate conclusion: i18n is worth unifying **inside the Gentle sibling lineage now**, but there is not enough convergence to replace Deductree, Pedigoo, OxDM and Diolama localization with one implementation today.

### Candidate: product preferences lifecycle

**Category:** Product Infrastructure

**Evidence**

- `gentle/gentle-app/src/utils.rs` — `AppSettings`, `app_data_dir`, `settings_path`, `load_settings`, `save_settings`, `update_settings`, `resolve_theme`.
- `gentle-cards/gentle-cards-app/src/utils.rs` — forked versions of the same symbols and storage split, with additional card preferences.
- Both Gentle entrypoints load settings before launching the backend/window and again hydrate root Dioxus contexts.
- Both Gentle apps persist window size through `use_wry_event_handler` and use serde defaults/read-side legacy fields for migration.
- `oxdm/src/persist.rs` — `ConfigFile`, `load_config`, `save_config`, `theme_from_str`, `locale_from_str`; separate keyring contract for secrets.
- `oxdm/src/main.rs` — reactive autosave effect for theme/locale/log/TLS and an early config read required before logging initialization.
- `Deductree/app/src/persist.rs` — `read_settings`/`write_settings` for controlled `diolama::Settings`, RON under the platform data directory.
- `Deductree/app/src/settings_overlay.rs` — controlled signal commit plus host persistence.
- `NewShiny/ShinyColors_diolama/src/canvas/album_shell/filter_storage.rs` — a versioned, byte-bounded localStorage record with typed success/failure receipts.

**Common behavior**

```text
resolve backend/path
→ read
→ deserialize
→ migrate/default
→ hydrate reactive state
→ mutate through UI
→ persist
```

The Gentle pair additionally shares desktop/web/mobile backend selection, locale/theme/window settings and the exact policy of loading before both embedded backend startup and root context creation.

**Differences**

- Schema is necessarily product-owned and differs substantially.
- Formats are JSON, TOML, RON and versioned JSON-in-localStorage.
- Locations express product policy: executable-adjacent portable data, `dirs::data_dir`, `~/.oxdm`, browser localStorage and keyring.
- Save errors range from silently ignored, through tracing-only, to typed visible errors in Story Editor document persistence.
- Autosave ranges from every signal change, immediate control commit, window-resize writes, to explicit save.
- Secrets must be keyring-only in OxDM/Shiny; ordinary preference infrastructure must never absorb credential policy.
- StoryDoc, case saves and asset manifests are document/data contracts, not preferences.

**Why duplication exists**

Each app started with a local schema and target-specific storage. Gentle Cards forked a proven implementation and extended it. OxDM needed secrets and log-before-launch ordering. Deductree inherited a reusable settings model from Diolama but retained host persistence. Shiny added bounds/version/error receipts for a single browser record. The lifecycle is stable; failure and storage policy are not.

**Proposed stable boundary**

Platform owns a small typed load/save result and backend contract. The consumer owns `T`, codec, migration and save policy. Backends should be explicit presets (`FileStore`, `LocalStorageStore`), not auto-selected globals.

**What must remain app-specific**

- Settings schema/defaults/migrations.
- Product root selection and portable-mode decision.
- Codec/format and version contract.
- Debounce versus explicit save.
- Keyring/credentials and document/project persistence.
- User-visible recovery policy.

**Possible API shape**

```rust
pub trait PreferencesCodec<T> {
    fn decode(&self, bytes: &[u8]) -> Result<T, PreferencesError>;
    fn encode(&self, value: &T) -> Result<Vec<u8>, PreferencesError>;
}

pub trait PreferencesStore {
    fn load(&self) -> Result<Option<Vec<u8>>, PreferencesError>;
    fn save(&self, bytes: &[u8]) -> Result<(), PreferencesError>;
}

pub enum LoadOutcome<T> {
    Loaded(T),
    Missing(T),
    Recovered { value: T, error: PreferencesError },
}
```

Do not add a generic settings UI or reflection-based schema.

**Score**

| Metric | Score |
|---|---:|
| Repetition | 4/5 |
| Stability | 3/5 |
| Pain | 4/5 |
| Leverage | 5/5 |
| Coupling Risk | 3/5 |
| Domain Independence | 5/5 |

**Recommendation:** `Prepare boundary, extract later`

The lifecycle is stable enough to define and test, but not yet stable enough to choose one default path/format/error policy for every product.

### Candidate: headless modal, toast and transient-surface behavior

**Category:** Dioxus Infrastructure / UI Convention

**2026-08-25 validation update:** current-source comparison split this combined survey
candidate into `DCA-002` backdrop-dismiss gesture state, `DCA-022` stable toast queue
lifecycle and `DCA-023` accessible modal focus lifecycle. The evidence below remains useful,
but its original single-crate boundary is superseded by
`docs/validation/DCA-002-transient-surface-boundary.md`.

**Evidence**

- `gentle/gentle-app/src/components/toast.rs` and `gentle-cards/gentle-cards-app/src/components/primitives/toast.rs` have identical SHA-256 and 154 lines each: `ToastKind`, bounded queue, root `ToastHost`, per-kind timeouts and click dismissal.
- The Gentle pair also has byte-identical `ImportProgressBar` (125 lines), `PageHeader` (39 lines), and `SectionCard` (33 lines).
- `gentle/gentle-app/src/components/modal.rs` — original focus/Escape/backdrop shell.
- `gentle-cards/gentle-cards-app/src/components/primitives/modal.rs` — evolved shell; commit `ea6e36e` fixes the press-inside/release-outside false dismissal with pointer-origin state.
- `oxdm/src/components/dialog_overlay.rs` — same click-outside/Escape/content-stop behavior with product CSS.
- `oxdm/src/components/toast.rs` and `oxdm/src/state.rs` — context-owned typed queue and timer.
- `Deductree/app/src/toast.rs` — context-owned queue and timer, simpler single style.
- `Deductree/diolama/src/confirmation_dialog.rs` — reusable VN-family confirmation behavior.

**Common behavior**

- Overlay owns focusability, Escape, outside dismissal and inside propagation barrier.
- Toasts use stable identity for delayed removal, but host placement and remount behavior differ.
- Long-running import progress must survive route/modal unmount and protect against browser unload.

**Differences**

- Tailwind classes versus product CSS classes.
- Global signal versus explicit app context ownership.
- Timer backend is JavaScript `setTimeout`, `tokio::time`, or `futures_timer`.
- Toast severity, queue bounds, close interaction and accessibility differ.
- Modal focus trap, return-focus and ARIA coverage are not consistently implemented.

**Why duplication exists**

Gentle Cards is a fork and retained exact files. Other apps independently rebuilt the same behavior around their own CSS/state convention. The modal evolution proves that behavior bugs do propagate unevenly: the cards fix did not flow back to Gentle or OxDM.

**Proposed stable boundary**

Do not extract one transient-surface crate. Validate the backdrop pointer state independently;
keep the toast queue/scheduler and accessible focus lifecycle as separate candidates until their
opposing implementations establish stable APIs. Do not move current Tailwind markup wholesale
into any supposedly generic crate.

**What must remain app-specific**

- Styling, spacing, animation, icons and visual severity colors.
- Dialog content and destructive confirmation semantics.
- Product-specific import progress text and navigation.
- Choice of global versus injected host unless a second style system validates one API.

**Possible API shape**

```rust
pub struct BackdropDismiss {
    pub on_pointer_down: EventHandler<PointerEvent>,
    pub on_pointer_up: EventHandler<PointerEvent>,
}

pub fn use_backdrop_dismiss(on_close: EventHandler<()>) -> BackdropDismiss;

pub struct ToastQueue<K> { /* ids, bound, dismissal only */ }
pub fn use_toast_queue<K>(capacity: usize) -> ToastQueue<K>;
```

**Score**

| Metric | Score |
|---|---:|
| Repetition | 4/5 |
| Stability | 3/5 |
| Pain | 4/5 |
| Leverage | 4/5 |
| Coupling Risk | 3/5 |
| Domain Independence | 5/5 |

**Recommendation:** `Prepare boundary, extract later`

The exact duplicates justify consolidation, but the stable cross-product asset is behavior, not the Gentle Tailwind skin.

### Candidate: background task ownership and progress contract

**Category:** Dioxus Infrastructure

**Evidence**

- `gentle/gentle-app/src/components/drop_import_modal.rs` — detached `spawn_forever` so closing the modal cannot cancel upload or freeze global progress.
- `gentle/gentle-app/src/pages/create_work_generic.rs` and `pages/edit_work.rs` — pre-collect owned snapshots before detached work because page-local signals will be dropped.
- `gentle-cards/gentle-cards-app/src/pages/settings/helpers.rs` — `run_restore`/pack import use `spawn_forever` and `try_write` for route-local status.
- `gentle-cards/gentle-cards-app/src/pages/cards_list.rs` — same detached upload pattern.
- `gentle-cards/gentle-cards-app/src/pages/instances_list.rs` — explicit counterexample: scope-bound `spawn` is required because the task owns local result/navigation state.
- `gentle-cards/gentle-cards-app/src/pages/play_instance_toolbar.rs` — another documented scope-bound cancellation choice.
- Git history: `7293117` fixes import progress by switching to `spawn_forever`; `adb5d08` applies the same fix to card drop; `9be2119` fixes `ValueDroppedError` from the wrong ownership choice.
- Byte-identical `ImportProgressBar` in both Gentle apps owns a route-independent progress signal and browser `beforeunload` guard.

**Common behavior**

- A task must declare whether route unmount cancels it or it outlives the component.
- Detached tasks may only own cloned data and root/runtime state; they must not blindly write dropped local signals.
- Progress and terminal error must reach a root host and must always clear.
- Navigation is a side effect that often belongs back in a live component, not in detached work.

**Differences**

- Upload, restore, SSE, polling and native backend tasks need different cancellation and retry policies.
- Dioxus `spawn`/`spawn_forever`, JS eval receipts, Tokio threads and Diolama state machines are not interchangeable.
- Many ordinary component requests are correctly handled by `use_resource` and should not use platform orchestration.

**Why duplication exists**

Dioxus task lifetime is implicit in the spawning API, while the product requirement is expressed in navigation semantics. The same mistake was fixed more than once after the Gentle fork.

**Proposed stable boundary**

Start with an ownership vocabulary and a tiny root task host for detached jobs. Require a job id, progress, terminal result and cancellation policy. Do not wrap every `spawn` or build a scheduler/retry framework.

**What must remain app-specific**

- Job body, retry, network protocol and domain errors.
- Whether a task is allowed to survive navigation.
- Navigation after completion.
- Shutdown semantics for native embedded servers.

**Possible API shape**

```rust
pub enum TaskLifetime {
    Component,
    App,
}

pub struct JobProgress {
    pub completed: u64,
    pub total: Option<u64>,
    pub label: String,
}

pub trait AppJob: Future<Output = Result<(), String>> + 'static {}

pub fn spawn_app_job(id: JobId, job: impl AppJob) -> JobHandle;
```

The concrete API should be validated against one upload and one restore before adding cancellation/retry.

**Score**

| Metric | Score |
|---|---:|
| Repetition | 4/5 |
| Stability | 3/5 |
| Pain | 5/5 |
| Leverage | 4/5 |
| Coupling Risk | 3/5 |
| Domain Independence | 5/5 |

**Recommendation:** `Prepare boundary, extract later`

### Candidate: diagnostics bootstrap and support surface

**Category:** Product Infrastructure

**Evidence**

- `Deductree/diolama/src/diagnostics.rs` — `install_crash_logger`, chaining the previous panic hook and writing panic/location/backtrace.
- `NewShiny/ShinyColors_diolama/src/main.rs` — installs that crash logger before Dioxus launch and prints the returned path.
- `oxdm/src/main.rs` — `init_logging`, `log_dir`, stderr + optional daily file layer, retained `WorkerGuard`, early preference read.
- `oxdm/src/components/about_dialog.rs` — user-facing log preference/path/open-folder support.
- `gentle/gentle-core/src/lib.rs` and `gentle-cards/gentle-cards-core/src/serve.rs` — separate `init_tracing(log_path)` implementations consumed before Dioxus launch.
- Both Gentle app entrypoints document the Dioxus logger initialization ordering bug: tracing must be installed before launch.
- `NewShiny/ShinyColors_diolama/src/canvas/error_shell.rs` — bounded diagnostics detail and categorized user-facing failure shell.
- `oxdm/src/components/tab_error.rs` — localized retry surface for resource failures.

**Common behavior**

- Initialize diagnostics before Dioxus can claim the global subscriber/hook.
- Preserve detailed technical context in logs while showing shorter user-facing messages.
- Keep a guard alive when asynchronous log flushing requires it.
- Give the user a discoverable log location and bounded/copyable details.

**Differences**

- Panic hook versus tracing subscriber versus UI error shell.
- Temp file, executable-adjacent log or rotating product log directory.
- Opt-in versus always-on persistence and retention.
- Product-specific classification/localization/retry.

**Why duplication exists**

The failure chain crosses process startup, logging backend, Dioxus global initialization and UI. Each product solved the piece it needed. Diolama's helper is reusable but named/filed for VN consumers and has not been validated by a non-VN app.

**Proposed stable boundary**

Offer explicit startup helpers that return a lifetime guard and support metadata. Keep UI error mapping separate.

**What must remain app-specific**

- Log retention/opt-in policy and directory.
- Product error taxonomy and localization.
- Retry actions and privacy/redaction.
- Domain telemetry.

**Possible API shape**

```rust
pub struct DiagnosticsGuard { /* flush/panic-hook lifetimes */ }

pub struct SupportInfo {
    pub log_paths: Vec<PathBuf>,
    pub version: String,
}

pub fn install_diagnostics(config: DiagnosticsConfig)
    -> Result<(DiagnosticsGuard, SupportInfo), DiagnosticsError>;
```

**Score**

| Metric | Score |
|---|---:|
| Repetition | 3/5 |
| Stability | 3/5 |
| Pain | 4/5 |
| Leverage | 3/5 |
| Coupling Risk | 2/5 |
| Domain Independence | 5/5 |

**Recommendation:** `Prepare boundary, extract later`

### Candidate: Dioxus desktop bootstrap / app shell

**Category:** Dioxus Infrastructure

**Evidence**

- `gentle/gentle-app/src/main.rs` and `gentle-cards/gentle-cards-app/src/main.rs` repeat: settings-before-launch, tracing-before-Dioxus, embedded Tokio backend thread, window restore/icon/background, disabled native drag/drop, macOS menu exception, iOS WKWebView fix and root context hydration.
- `Deductree/app/src/main.rs` — `main`, `App`, `use_locked_aspect`, asset protocol registration and a large `Ui` context.
- `oxdm/src/main.rs` — early log preference, tracing guard, desktop window/config and reactive `Ctx` setup.
- `Pedigoo/app/src/main.rs` — explicit window focus/on-top/QA behavior and `Game` context.
- `NewShiny/ShinyColors_diolama/src/main.rs` — crash hook, bounded remote-debugging argument and desktop window.

**Common behavior**

- Do prelaunch work in the correct order.
- Construct `WindowBuilder`/`Config`, then launch a root component.
- Hydrate a Copy/signal context and mount root-level providers/hosts.
- Keep platform workarounds near startup.

**Differences**

- Product startup services, targets, router, window ratio, menu, drag/drop, debugging and failure policy all differ.
- Gentle is multi-target with an embedded backend; the other apps are primarily desktop.
- Root state structs are domain composition and should not be standardized.
- Dioxus version skew makes a shared launch crate a central upgrade constraint.

**Why duplication exists**

Some is true common infrastructure, but much of `main` is product composition. The Gentle pair copied the same product architecture; other apps only share Dioxus syntax.

**Proposed stable boundary**

For now, maintain a paved-road document and extract only proven helpers: diagnostics-before-launch, restored window size, platform menu policy and perhaps an embedded-backend guard. Do not create an `AppBuilder` with generic callbacks for every product.

**What must remain app-specific**

- Root context and router.
- Domain/backend startup.
- Window dimensions/aspect and asset protocol.
- Product CSS/theme and error screen.
- Debug/QA arguments.

**Possible API shape**

No shared framework API is justified yet. A helper-level shape is safer:

```rust
pub fn restored_window(
    title: &str,
    bounds: WindowBounds,
    icon: Option<Icon>,
) -> WindowBuilder;

pub fn install_platform_text_edit_menu(config: Config) -> Config;
```

These should only be added after a second non-fork app adopts them without exceptions.

**Score**

| Metric | Score |
|---|---:|
| Repetition | 4/5 |
| Stability | 2/5 |
| Pain | 4/5 |
| Leverage | 4/5 |
| Coupling Risk | 4/5 |
| Domain Independence | 4/5 |

**Recommendation:** `Keep duplicated for now`

## 5. Repeated Reinventions

| Problem | Implementations found | Likely reinvention count | Cost/Pain |
|---|---:|---:|---|
| IME controlled-input composition | 5 manual sites in Gentle, 1 manual site + 3 hook consumers in Cards, plus 2 independent global shortcut bridges | At least 3 implementation generations | High: broken CJK input, lost WebView2 commit, Enter/Escape misfire, unmount during composition |
| i18n runtime/resource model | Deductree chrome, Deductree Story Editor, Pedigoo, Gentle, Cards, OxDM, Shiny/Diolama content | 5 distinct patterns across 6 apps | High maintenance; no shared locale/fallback/persistence contract |
| Settings lifecycle | Gentle, Cards, OxDM, Deductree/Diolama, Shiny localStorage | 4 patterns, 5 consumers | Medium-high: startup ordering, schema migration, silent corruption/write failure, target splits |
| Modal outside/Escape/focus behavior | Gentle, Cards, OxDM, Deductree/Diolama | 4 implementations | Medium-high: cards already fixed a press-origin bug not propagated elsewhere |
| Toast queue/timer | Gentle, Cards, OxDM, Deductree | 4 implementations | Medium: async ownership and timer backend differ; two files are byte-identical |
| Long-running task lifetime/progress | Gentle create/edit/drop, Cards restore/import/drop, root progress | More than 5 flows | High: frozen progress and `ValueDroppedError` have history |
| Desktop launch/window workarounds | All 6 apps | 6 entrypoints; one strong fork pair | Medium: platform/menu/drag/drop/log ordering knowledge is repeatedly rediscovered |
| Logging/crash visibility | Gentle core, Cards core, OxDM, Diolama/Shiny | 4 implementations | Medium-high: wrong initialization order can silently lose logs |

### 5.1 IME evolution

```text
Pattern A — local boolean per input
  Gentle commit f9a531e: guard three live-suggest inputs
  Gentle b35f28c: repeat for dynamic fields

→ Pattern B — accumulated platform semantics
  use Signal::peek to avoid reactive rerender
  read CompositionData at compositionend for WebView2
  suppress Enter/Escape while composing
  keep global shortcuts out of inputs

→ Pattern C — partial reusable hook
  shared-history commit 3ce19e3: ImeGuard/use_ime_guard
  Cards consumes it in dynamic forms and deck search
  Cards still retains a manual LabelInput near-copy
  global shortcut JS separately adds e.isComposing
```

Pattern C contains real fixes learned after Pattern A, but the extraction is incomplete and remains inside one product fork.

### 5.2 i18n evolution

```text
Pedigoo: closed Lang + exhaustive Key + domain formatting helpers
  → Deductree: copied closed UI-chrome model, late-resolved case content
  → Deductree Story Editor: string keys + external runtime TOML overrides

Gentle: extensible locale string + typed bundled TOML + detection/persistence
  → Gentle Cards fork: larger schema + named apply_template helper

OxDM: independent string-key match modules + English fallback + key coverage tests
Shiny/Diolama: localized content values + session-owned display language
```

Later versions fix different pain points, not the same linear problem. Typed TOML improves resource maintenance; Story Editor adds runtime override; OxDM adds key coverage; Diolama preserves language-neutral content until render. None can replace all others unchanged.

### 5.3 Settings evolution

```text
Gentle AppSettings
  JSON beside executable on native
  localStorage keys on web
  serde defaults + legacy fields
  window/locale/theme startup integration

→ Gentle Cards fork
  same lifecycle and keys
  more product fields and constraints
  same window/platform bootstrap

Independent branches
  OxDM: TOML + keyring split + reactive autosave + visible logs
  Deductree: host-owned RON persistence for Diolama Settings
  Shiny: bounded, versioned localStorage record + typed failure receipt
```

The later branches show that the shared abstraction must preserve errors and versioning without owning the product schema or storage location.

## 6. Proposed Internal Platform

The minimum evidence-backed platform is an ecosystem, not one crate. It should begin with one extracted capability and keep the remaining boundaries as incubating contracts.

```text
dioxus-common-abilities/
└── crates/
    └── dioxus-input/                 # extract now

Later, only after two consumers validate each boundary:
├── crates/product-preferences/       # headless stores/results; app owns schema
├── crates/product-i18n/              # locale/fallback contract; app owns resources
├── crates/dioxus-backdrop-dismiss/   # only if the pointer-state spike proves worthwhile
├── docs/validation/                  # DCA-022 and DCA-023 remain documents first
└── crates/product-diagnostics/       # startup guard + support metadata

No dioxus-app-shell crate yet:
└── docs/paved-road/                  # recipes until startup semantics converge
```

| Unit | Responsibility | Dependencies | Initial consumers | Explicit non-responsibilities |
|---|---|---|---|---|
| `dioxus-input` | Composition state, WebView2 final commit extraction, command suppression predicate | Dioxus 0.7 only | Cards first, Gentle second; OxDM/Deductree validation | Form model, validation, text widgets, key registry, CSS |
| `product-preferences` | Raw backend + typed load/save outcomes, optional atomic file preset | `std`; optional serde adapters | Gentle/Cards first, then OxDM or Deductree | Schema, format mandate, app directories, secrets, settings UI |
| `product-i18n` | Locale identity/fallback and translator contract | `std`; serde optional | Gentle/Cards first | Translation keys, domain values, resource format, plurals, fonts/assets |
| `dioxus-backdrop-dismiss` candidate | Same-pointer backdrop gesture state only | `std`; thin Dioxus adapter if justified | Cards, Gentle; OxDM as opposing-policy check | Modal rendering, Escape/focus, toast queue, CSS and content |
| `product-diagnostics` | Install order, guard lifetime, log/support paths | `tracing` optional | OxDM plus a Gentle app or Shiny | Product error taxonomy, telemetry, redaction policy |
| paved-road docs | Startup sequence and composition examples | None | Every new app | Generic builder, DI container, router/context framework |

Dependency direction should remain:

```text
product-i18n        product-preferences       product-diagnostics
      ↑                     ↑                         ↑
      └──────── optional Dioxus adapters / hooks ────┘
                                  ↑
                           product application
                                  ↓
                              domain core
```

Domain cores must not depend on Dioxus. Diolama remains a domain platform for VN products; non-VN apps must not import Diolama merely to obtain generic-looking desktop helpers.

### Existing platform fragments that should stay where they are

- `Deductree/diolama/src/desktop.rs` — `AssetProtocol`, `SourceAssetBackend`; already consumed by `Deductree/app/src/story_editor/assets.rs` and `NewShiny/ShinyColors_diolama/src/assets/runtime.rs`. Keep in Diolama until a non-VN product demonstrates the exact range/cache/asset-source semantics.
- `Deductree/diolama/src/diagnostics.rs` — `install_crash_logger`; useful reference and current Shiny consumer. A future generic diagnostics crate may absorb it only with explicit compatibility/migration.
- `Deductree/app/src/commands.rs` — pure command model with one real product surface and conceptual analogues elsewhere. Keep local until a second consumer can use it without adding browser-focus/IME special cases.

## 7. Extraction Order

### 1. `dioxus-input`

Highest pain and stability, smallest surface, lowest coupling. It closes a correctness issue rather than merely deleting lines.

### 2. Backdrop-dismiss gesture state

Use the Cards modal incident as the regression seed and OxDM's close-on-down policy as the
opposing check. First prove same-pointer, panel-release and `pointercancel` semantics in a pure
state spike. Toast queue and accessible modal focus continue separately as `DCA-022` and
`DCA-023`; neither is part of this extraction step.

### 3. Product preferences backend/result boundary

Begin with Gentle/Cards exact lifecycle. Require typed failures and a consumer-owned schema. Validate the second phase with OxDM or Deductree because they exercise different path/format/save policies.

### 4. Product i18n locale/fallback boundary

Unify the Gentle siblings first. Do not migrate Pedigoo/Deductree domain helpers. Promote a broader API only after OxDM can consume the locale/fallback layer without abandoning its key coverage policy.

### 5. Background task ownership/progress host

Use one detached import and one component-cancelled flow as opposing acceptance tests. Avoid retry/scheduler features until these two lifetimes work.

### 6. Diagnostics bootstrap

Consolidate only install ordering, guard lifetime and support paths. Keep UI error presentation separate.

### 7. App shell recipes; reconsider a crate last

After settings, diagnostics, input and UI behavior have their own seams, reevaluate what genuinely remains duplicated in `main`. Until then, a shared app builder would mostly be callbacks around unstable product startup.

## 8. First Extraction Proposal

### Selection

Extract `dioxus-input` first.

### Why first

- It has a direct user-input correctness consequence for Chinese/Japanese/Korean text.
- It includes a documented WebView2-specific lost-event workaround.
- Git history demonstrates repeated fixes rather than speculative reuse.
- The stable common API is already approximately 62 lines in Cards.
- It has no domain, router, storage, CSS or backend coupling.

### Current implementations

- Canonical seed: `gentle-cards/gentle-cards-app/src/components/primitives/use_ime.rs` — `ImeGuard`.
- Manual sibling implementations: five `is_composing` sites in Gentle and `LabelInput` in Cards.
- Keyboard companion: `gentle-cards/gentle-cards-app/src/components/play_canvas/keyboard.rs` — `e.isComposing` and focused-field exclusion.
- Validation exposures: OxDM Enter/Escape controlled inputs and Deductree editor shortcut bridges.

### Proposed crate boundary

```text
crates/dioxus-input/
├── Cargo.toml
└── src/
    ├── lib.rs          # public composition hook/state
    └── shortcut.rs     # tiny field/composition JS predicate, if validated
```

The crate owns no rendered component.

### Minimal public API

```rust
pub use dioxus::events::CompositionData;

#[derive(Clone, Copy)]
pub struct CompositionGuard { /* private Signal<bool> */ }

pub fn use_composition_guard() -> CompositionGuard;

impl CompositionGuard {
    pub fn is_composing(self) -> bool;
    pub fn start(&mut self);
    pub fn finish(
        &mut self,
        event: Event<CompositionData>,
    ) -> Option<String>;
}
```

Do not add `ControlledInput`, validation traits, async typeahead, form schema or command registry.

### First adopter

`gentle-cards-app` is the safest first adopter because it already has the extracted `ImeGuard`. The initial migration should be a lift-and-import with unchanged behavior, then replace the remaining `LabelInput` boolean with the crate hook while preserving its app-owned blur/commit logic.

### Second consumer and abstraction validation

`gentle-app` is the second consumer. Replace its five manual signal sites. It validates both parent-controlled and locally controlled values, suggestions, chips and link fields.

After those are green, use one OxDM Enter-driven field as a **manual validation target**, not as an automatic migration. If OxDM requires no app-specific hook change, that supplies the first non-fork consumer evidence.

### Migration steps

1. Add a standalone crate pinned to the workspace-supported Dioxus 0.7 range; write pure/state-level tests where possible.
2. Move `ImeGuard` behavior from Cards without API expansion.
3. Adopt in Cards dynamic forms/deck search and replace the Cards `LabelInput` near-copy.
4. Verify zhuyin/pinyin/kana typing, final commit, Enter candidate selection, Escape and blur on WebView2 and one non-Windows browser/WebView.
5. Adopt in Gentle's five manual sites.
6. Re-run the same matrix, including live suggestion resources and chip creation.
7. Only then decide whether global shortcut suppression belongs in the same crate; validate against Cards Canvas and Deductree Story Editor bridges.

### Success criteria

- Delete the 62-line app-local `use_ime.rs` implementation after its behavior exists only in the shared crate.
- Remove six manual composition-state implementations across Gentle/Cards; callers retain only their value/commit callbacks.
- No regression in CJK composition, WebView2 final text commit, Enter/Escape, blur or typeahead refresh.
- A second consumer needs no domain-specific branch or workaround in the shared API.
- Domain cores remain Dioxus-free.
- The shared API contains no product names, CSS, router, storage, form schema or command enum.
- Global shortcuts never fire during IME composition if that companion seam is included.

## 9. What should not be abstracted

### A. Domain Core

- ONVIF protocol/device/session operations (`oxvif`, OxDM API/view state).
- Album/tag/content models and migrations (`gentle-core`, `gentle-cards-core`).
- Card-table commands, anti-cheat, SSE and replay state.
- Pedigoo world generation, genetics, racing and domain translation helpers.
- Deductree mystery checker/file contract.
- Diolama story/player/authoring semantics.

### B. Product infrastructure that still lacks convergence

- One universal app-data directory. Portable Gentle, platform-data Deductree and `~/.oxdm` are deliberate policies.
- Credential/keyring abstraction inside ordinary settings. Secrets have different threat and migration contracts.
- Import/export formats. `.dtpack`, CBZ, backup ZIP, card packs, match exports, JUnit and Diolama packs are domain contracts.
- Updater/version framework. Only Gentle shows an updater-like client path, while Diolama versioning is package/format governance.

### C. Dioxus/UI surfaces that should remain product-owned

- Whole theme/skin system. Existing themes represent different art direction and layout contracts.
- Root context structs (`Ui`, `Game`, `Ctx`) and routers.
- Full settings/about/update pages.
- Shiny canvas/error shell and Diolama VN screens.
- Story Editor project persistence, assets, cast policy and Story Map UX. Per Story Editor governance, reusable dialogue semantics remain Diolama-owned and Deductree owns project composition.

## 10. Explicit Answers

### 1. 我目前到底有沒有「Dioxus internal platform」的自然雛形？

有，但它是 fragmented prototype：Gentle fork lineage 提供 product-shell 證據，Diolama 提供成功 reusable crate 證據，其他 app 提供獨立驗證與差異。尚不足以建立一個 app framework，足以開始一個小型 capability ecosystem。

### 2. 哪些能力已經重造至少兩輪或三輪？

- IME：至少三輪，且後輪包含前輪 bug 修正。
- i18n：至少五種模式。
- settings lifecycle：四種模式／五個 consumers。
- modal/toast：四個 app implementations；Gentle pair 有四個 byte-identical primitives。
- long-running task ownership：多個 flows，且有重複修 bug 的 git history。
- desktop bootstrap/logging/window policy：每個 app 都有自己的版本，Gentle pair 是明確 fork copy。

### 3. `i18n` 是否已經值得立即統一？

值得立即統一 Gentle 與 Gentle Cards 的 locale/resource lifecycle；不值得一次遷移所有產品。全 workspace 先定義 locale/fallback boundary，保留 typed enum、typed TOML、runtime override 和 domain translation 的差異。

### 4. `settings` 是否已有穩定的共通 lifecycle？

有穩定 lifecycle，但沒有穩定的共同 path/format/error/autosave policy。可以準備 headless store/result boundary，schema、codec、migration、secret 與 UI 必須留在 app。

### 5. `IME/input` 問題是否能抽成一個小而可靠的 Dioxus integration layer？

可以，而且是本次唯一建議 `Extract now` 的 crate。現有 `ImeGuard` 已接近正確邊界，只需補齊 app-internal near-copies 和 shortcut composition policy 的驗證。

### 6. 哪些看似共通的東西其實不應抽象？

Domain cores、import/export formats、credential policy、one-size-fits-all app directory、整套 theme、root context/router、VN asset/runtime semantics，以及只有單一 app 的 updater。

### 7. 如果今天只允許抽一個 crate，應該抽哪一個？

`dioxus-input`。

### 8. 抽完第一個之後，下一個新 Dioxus app 能具體少寫哪些東西？

新 app 不必再手寫：

- composition boolean signal；
- `oninput` 中途更新屏障；
- non-reactive `.peek()` 規則；
- WebView2 `compositionend` final-data workaround；
- composing 時 Enter/Escape/shortcut suppression；
- 每種 controlled input 都重新驗證 CJK 的測試矩陣。

它仍需自己決定 value commit、validation、form UI 和 command semantics，這正是小 abstraction 應保留的產品差異。

## Final conclusion

> **目前最值得成為 paved road 的能力是小型 `dioxus-input` composition／shortcut layer；證據是 Gentle lineage 至少三輪、六個以上局部實作、WebView2 特定 workaround，以及已發生的 CJK 輸入與 task/shortcut 邊界修正。**
