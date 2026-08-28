# DCA-011 Validation Example：OxDM Config and Keyring Split

Evidence date：2026-08-25

Consumer：workspace sibling `oxdm`

本例重建已存在的 migration boundary：舊版 `config.toml` plaintext credentials
遷移到 system keychain，同時讓普通 preferences、device metadata 與 secret values
維持不同 authority。

## 1. Change identity

- **Owner:** OxDM `persist` module
- **Data crosses:** process restart and app upgrade
- **Trust boundary:** local preferences plus OS-protected credentials
- **Ordinary files:** `~/.oxdm/config.toml`、`devices.toml`、`healthcheck.toml`
- **Secret store:** keychain service `com.oxdm`、user `credentials`

## 2. Version identities

| Identity | Owner/source | Current behavior | Migration behavior |
|---|---|---|---|
| App/package | Cargo metadata | independent | no automatic package-format substitution |
| Ordinary config shape | `persist.rs::ConfigFile` and TOML readers | unversioned, default-on-read | legacy fields remain readable |
| Device metadata shape | `DevicesFile`/`DeviceRecord` | unversioned, default-on-read | `has_credentials` defaults false |
| Health-group metadata | `HealthGroupsFile`/`HealthGroup` | unversioned, credentials skipped | credentials hydrate from keychain |
| Credential blob | keychain JSON `CredsMap` | reserved keys inside one entry | legacy global plaintext copied into blob |

The checklist can represent several persisted identities without inventing a
single OxDM format version. It also exposes that these local formats currently
have no explicit version field or formal compatibility map.

## 3. Authoritative representation

- **Ordinary model/readers/writers:** `src/persist.rs`
- **Startup composition:** `src/main.rs` loads config, credentials, devices and
  groups in that order
- **Credential source:** `keyring_load_all`／`keyring_save_all`
- **Complete credential projection:** `build_creds_map`
- **Legacy migration:** `load_all_credentials`
- **Config writer:** `save_config` deliberately serializes a `ConfigOut` without
  username/password
- **Product contract:** `CLAUDE.md` section `Credentials + persistence`

## 4. Change classification

- **Shape:** secret extraction from ordinary config into a separate store
- **Ordinary defaults:** every config field uses `#[serde(default)]`
- **Unknown fields:** TOML/Serde default behavior; no deny-unknown policy found
- **Legacy fields:** username/password are read for migration but never written by
  current `save_config`
- **Device metadata:** persists only name、address and `has_credentials`; actual
  values are joined from the keychain map
- **Health-group metadata:** credential fields are `#[serde(skip)]` and hydrate
  from reserved key prefixes

This is not an ordinary additive settings field. Secret write success and
plaintext removal order are load-bearing.

## 5. Compatibility matrix

| Persisted state | Current app | Result | Evidence |
|---|---|---|---|
| No files/keychain | Current | Defaults and empty credentials/devices/groups | load fallbacks |
| Current TOML plus valid keychain | Current | Keychain wins; metadata hydrates credentials | `load_all_credentials`, `load_devices`, `load_health_groups` |
| Legacy config with plaintext, empty keychain | Current | Attempts keychain migration and returns legacy credentials in memory | legacy branch in `load_all_credentials` |
| Current config read by legacy app | Older | Product promise not documented in inspected authority | explicit validation gap |
| Malformed config/devices/groups TOML | Current | Logs error and falls back to defaults/empty collections | load functions |
| Malformed keychain JSON | Current | Warns and returns an empty credential map | `keyring_load_all` |
| Metadata says credentials exist but keychain key is missing | Current | Device loads with `credentials = None` | join logic in `load_devices` |

The current system prioritizes startup availability over strict refusal. That
policy remains OxDM-owned.

## 6. Migration, rollback and write outcomes

- **Migration:** legacy username/password are inserted into the in-memory map and
  passed to `keyring_save_all`.
- **Plaintext retirement:** subsequent `save_config` writes a separate `ConfigOut`
  that omits legacy fields.
- **Complete keychain writes:** both device and health-group save paths rebuild the
  entire credential blob to avoid clobbering other credential tiers.
- **Ordinary write failures:** logged; save functions return no result.
- **Keychain write failures:** logged; `keyring_save_all` returns no result.

Potential failure-path gap surfaced by the checklist:

`load_all_credentials` cannot observe whether the legacy-to-keychain write
succeeded, yet later `save_config` omits plaintext credentials. If keychain
persistence fails, credentials remain available in memory for the current run
but may lose their persisted authority for the next launch. Exact Dioxus effect
timing and user-visible consequences were not runtime-tested here. This finding
does not authorize an OxDM code change, but it demonstrates why shared workflow
must require an observable secret-migration receipt before old plaintext removal.

## 7. Safety and resource bounds

- **Raw input caps:** ordinary local TOML and keychain JSON readers use
  `read_to_string`/`get_password` with no explicit size cap in the inspected path.
- **Archive expansion:** not applicable.
- **Parsed cardinality/string caps:** no explicit bounds found for config devices,
  health groups or credential map.
- **Secret exclusion:** current writers omit credentials from ordinary TOML;
  HealthGroup secret fields use `serde(skip)`.
- **Logs:** inspected persistence logs report error text and paths/counts, not
  credential values.
- **Failure surface:** load/save failures are logs plus fallbacks; they are not
  returned to the caller as typed outcomes.

The common template correctly treats missing bounds and non-observable writes as
documented gaps instead of inventing a universal cap or error type.

## 8. Verification assessment

Existing structural evidence:

- ordinary config writer cannot serialize legacy credentials；
- health-group credential fields are excluded from TOML；
- one complete keychain projection prevents device/group save-path clobbering；
- missing keychain entries degrade to absent credentials rather than panic；
- legacy config remains readable for migration。

Gaps surfaced by the common checklist:

- no focused tests were found for keychain projection, legacy migration ordering,
  missing keychain values or plaintext exclusion；
- persistence write failures do not reach callers；
- the local formats have no explicit version identity or stated old/new direction；
- no input/cardinality bounds are visible in the inspected persistence path。

These are validation findings, not authorization to modify OxDM.

## 9. Ownership result

- **Common workflow owns:** separating secret and ordinary identities, asking for
  write receipts, migration ordering, downgrade direction, bounds and exclusion
  evidence.
- **OxDM owns:** keychain service/keys, TOML shape, fallback policy, error UX,
  actual bounds and any compatibility promise.
- **Shared runtime change:** none implied; `DCA-003` preferences and `DCA-006`
  diagnostics remain separate candidates.

## 10. Template verdict

The template can represent a secret split without treating credentials as normal
preferences. Validation required two additions: explicit secret-migration ordering
and observable write outcome. Those fields were added to the prototype.
