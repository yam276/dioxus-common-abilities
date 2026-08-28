# DCA-011 Validation Example：Deductree Mystery Contract

Evidence date：2026-08-25

Consumer：workspace sibling `Deductree`

本例重建兩個已存在的 contract decisions，不提出格式變更：

- additive case：`Character.title` 是 current format 內的 optional field；
- structural case：`format_version` 3 引入 `Testimony` 並移除舊 `Challenge` shape。

## 1. Change identity

- **Owner:** `deductree-core` mystery/format contract
- **Data crosses:** process restart、app upgrade、community file exchange
- **Trust boundary:** public untrusted import
- **Canonical representation:** RON `.dtree`
- **Derived representation:** JSON export，non-authoritative

## 2. Version identities

| Identity | Owner/source | Current | Additive case | Structural case |
|---|---|---:|---|---|
| Package/executable | Cargo metadata | independent | unchanged | independent decision |
| Mystery document | `core/src/mystery.rs::FORMAT_VERSION` | `3` | unchanged | bumped to `3` |
| File identity | `core/src/mystery.rs::MAGIC` | `0xDED` | unchanged | unchanged |
| Save format | `core/src/save.rs::SAVE_VERSION` | independent | unaffected | separately evaluated |
| Pack/story/catalog | their own owners | independent | unaffected | unaffected |

The checklist correctly prevents package or save identity from substituting for
the mystery document identity.

## 3. Authoritative representation

- **Schema/model:** `core/src/mystery.rs::Mystery` and nested types
- **Reader:** `core/src/format.rs::{from_ron,from_json,load,import}`
- **Writer:** `core/src/format.rs::{to_ron,to_json}`
- **Identity gate:** `core/src/format.rs::check_identity`
- **Semantic/resource validation:** `core/src/validate.rs::validate`
- **Bounded analysis:** `core/src/checker.rs::analyze`
- **Contract:** `docs/FileContract_V1.md`
- **Tests:** `core/tests/format.rs` plus checker/validation tests

Current code and contract agree that RON is authoritative, JSON is derived,
unknown versions are rejected, and the checker verdict is recomputed locally.

## 4. Change classification

### Additive field

`Character.title` uses `#[serde(default)]` and is skipped when empty. A current
format file may omit it and receive `LocalizedString::default()` without changing
`FORMAT_VERSION`.

### Structural change

`FORMAT_VERSION` 3 represents `Testimony`; the current reader requires exact
version equality. It has no automatic v2-to-v3 migration path in `format::load`.
This is a product contract decision, not something the common checklist should
replace.

Unknown fields are not denied on `Mystery` structs. Same-version downgrade is
therefore expected to discard fields unknown to the older reader, but no explicit
`Character.title` downgrade fixture was found; this remains a verification gap.

## 5. Compatibility matrix

| Data | Reader | Current result | Evidence |
|---|---|---|---|
| Current v3 without `Character.title` | Current v3 | Loads with empty title | `#[serde(default)]` on `Character.title` |
| Current v3 with `Character.title` | Older v3 reader | Expected to load and discard unknown field | Serde default behavior; no explicit downgrade test found |
| Current v3 | Current v3 | Canonical RON and JSON-to-RON round trips | `core/tests/format.rs::{ron_round_trips,json_export_re_imports_equal}` |
| v2 | Current v3 | Rejected as `UnsupportedVersion(2)` | Exact comparison in `format::check_identity`; no v2 migration |
| version 999 | Current v3 | Rejected as `UnsupportedVersion(999)` | `unknown_version_is_rejected` |
| Wrong magic | Current v3 | Rejected as `NotDeductree` | `wrong_magic_is_rejected` |
| Malformed RON/JSON | Current v3 | `FormatError::Parse`, no panic | `garbage_does_not_panic` |
| Structurally oversized v3 | Current v3 | Import returns a report containing validation errors; analysis is skipped | `validate::cap_errors`, `checker::analyze` |

The two directions cannot be summarized as one "backward compatible" flag.

## 6. Migration and rollback

- **Additive field:** no migration; default-on-read and omit-when-empty.
- **Structural version:** no automatic import migration in current format module;
  unsupported versions are refused.
- **Mutation/rollback:** not applicable to immutable import until the app explicitly
  saves or exports a new file.
- **Old bytes retained:** import reads the caller-provided string and does not
  rewrite it.

## 7. Safety and resource bounds

- **Raw input byte cap:** no byte cap is visible in `core/src/format.rs` before
  RON/JSON deserialization. The checklist exposed this as a current evidence gap;
  no code change is authorized by this validation.
- **Archive expansion:** not applicable to bare `.dtree`; `.dtpack` has a separate
  owner and caps.
- **Parsed caps:** leads 256、clues 512、variables 8、domain 64、assignment product
  20,000、text 4,000、day budget 1,000，另有portrait/debate/testimony caps。
- **Search bound:** `MAX_SEARCH_STATES = 5_000`; exceeding it yields
  `solvable_within_budget = None` rather than hanging.
- **Failure surface:** parse/identity failures are `FormatError`; semantic and cap
  failures are returned in `Report.errors`.
- **Determinism:** contract/checker paths use ordered sets/maps where ordering
  affects reproducibility.

## 8. Verification assessment

Existing positive evidence:

- canonical round trip；
- derived JSON re-import；
- locally recomputed solvability；
- missing optional fields through Serde defaults in the model；
- malformed input and identity/version refusal；
- post-parse semantic/resource caps and bounded search。

Gaps surfaced by the common checklist:

- no explicit same-version downgrade test for an unknown additive field；
- no raw serialized-input byte bound visible before deserialization；
- the supplied unknown-version test uses 999 rather than a released old version，
  although exact comparison makes the result mechanically clear。

These are validation findings, not authorization to change Deductree.

## 9. Ownership result

- **Common workflow owns:** asking for identities, both compatibility directions,
  pre/post-parse bounds, fixtures and failure surface.
- **Deductree owns:** exact version gating, canonical RON, JSON convenience export,
  checker certification, limits and any migration promise.
- **Shared runtime change:** none implied.

## 10. Template verdict

The template represents both additive defaulting and exact structural refusal
without forcing a migration policy. Splitting raw, archive and parsed bounds was
necessary; the prototype was updated accordingly.
