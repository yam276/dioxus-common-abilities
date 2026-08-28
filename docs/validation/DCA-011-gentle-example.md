# DCA-011 Validation Example：Gentle Database and Backup Contract

Evidence date：2026-08-25

Consumer：workspace sibling `gentle`

本例重建兩個已存在的 contract decisions，不提出 schema change：

- additive case：migration `0014_work_import_status.sql`；
- structural case：backup format v1 到 v2 的 tag-model transform。

## 1. Change identity

- **Owner:** `gentle-core`
- **Data crosses:** library reopen、app upgrade、backup export/restore、downgrade
- **Trust boundary:** LAN-exposed untrusted multipart backup upload
- **Backends:** PostgreSQL and SQLite migrations must remain behaviorally aligned

## 2. Version identities

| Identity | Owner/source | Current | Additive case | Structural case |
|---|---|---:|---|---|
| App/package | Cargo package metadata | package-owned | unchanged by schema alone | release-owned |
| Database schema | highest SQLx migration plus `LATEST_DB_SCHEMA_VERSION` | `14` | bumped by migration 0014 | independently evolved |
| Backup format | `gentle_core::BACKUP_FORMAT_VERSION` | `2` | unchanged | bumped for v1-to-v2 JSON shape |
| Backup manifest | `BackupManifest` | carries all three identities | records schema 14 | selects transform for version 1 |

The example confirms that database and backup identities must occupy separate
rows in the common template.

## 3. Authoritative representation

- **Database source:** numbered files in `gentle-core/migrations/` and
  `gentle-core/migrations/sqlite/`
- **Static schema identity:** `gentle-core/src/lib.rs::LATEST_DB_SCHEMA_VERSION`
- **Backup identity:** `gentle-core/src/lib.rs::BACKUP_FORMAT_VERSION`
- **Backup model:** `gentle-core/src/model/backup.rs`
- **Writer:** `gentle-core/src/api/backup.rs`
- **Reader/identity gate:** `gentle-core/src/api/restore.rs::restore_handler`
- **Structural migration:** `gentle-core/src/api/restore.rs::transform_v1_to_v2`
- **Database apply:** `gentle-core/src/db/restore.rs::apply_backup`
- **Contract/checklist:** `gentle/gentle-core/CLAUDE.md`

## 4. Change classification

### Additive field

Migration 0014 adds nullable `works.import_status`. The Rust field is
`Option<String>` with `#[serde(default)]`. Old backups omit it and restore as
`None`; the backup format remains 2 because the JSON evolution is additive.

### Structural change

Backup format 1 contained persons/authors/organizations arrays and native work
fields. Format 2 represents them as tags. Restore first reads untyped JSON,
applies `transform_v1_to_v2`, then deserializes `LibraryBackup`.

Released migration files are append-only; PostgreSQL and SQLite each own a 0014
file with intended equivalent behavior.

## 5. Compatibility matrix

| Data | Reader | Current result | Evidence |
|---|---|---|---|
| Pre-0014 backup without `import_status` | Current | Deserializes `None` and restores into migrated DB | `work_deserialises_without_import_status_field`, `restore_fills_import_status_none_for_legacy_work` |
| Current backup with `import_status` | Older app | Expected unknown field drop; schema warning when manifest is newer | default Serde unknown-field behavior plus restore warning; no explicit old-binary test |
| Backup format 1 | Current format 2 | Transformed to tag model before typed deserialization | `transform_v1_to_v2` and transform tests |
| Backup format 2 | Current | Restores and reports counts/warnings | restore/apply tests |
| Backup format greater than 2 or below 1 | Current | Rejected with bad-request error | range gate in `restore_handler` |
| Newer DB schema manifest | Older current schema | Continues with warning; unknown tables/columns may be dropped | schema comparison in `restore_handler` |
| Malformed/missing manifest or library JSON | Current | Bounded bad-request error | explicit parse/missing handling |

The product intentionally promises more downgrade tolerance than Deductree's
exact document-version gate. The shared workflow preserves both policies.

## 6. Migration and rollback

- **Database migration:** SQLx applies numbered migrations when opening a library.
- **Pre-upgrade backup:** `db/upgrade_backup.rs` creates a DB-only safety archive
  before schema upgrade.
- **Additive restore:** missing field defaults and migration-provided nullable
  column allow old backup to new DB.
- **Structural restore:** explicit v1-to-v2 JSON transform before typed apply.
- **Partial media failure:** restore records warnings and orphan cleanup rather
  than silently presenting full success.
- **Concurrency:** backup/restore share an app-level write-operations guard.

Idempotence and rollback of migration 0014 were not independently executed for
this documentation validation; local SQLx migration behavior and upgrade-backup
tests remain the product authority.

## 7. Safety and resource bounds

- **Raw upload:** streamed to a tempfile with `MAX_BACKUP_BYTES = 5 GiB`.
- **Archive entry:** `MAX_BACKUP_ENTRY_BYTES = 100 MiB`.
- **Library JSON:** `MAX_BACKUP_LIBRARY_JSON_BYTES = 100 MiB` before allocation.
- **Parsed cardinality:** explicit caps for works, tags, categories, links, images,
  progress and icons.
- **Strings:** title 4,096、tag 512、config value 4,096 bytes.
- **Failure surface:** `AppError::bad_request` for invalid/untrusted input;
  `RestoreReport` carries user-relevant warnings and counts.
- **Secret material:** not part of this backup-contract example; must be marked
  not applicable rather than assumed safe.

The three separate bound layers justify the template revision made during this
validation.

## 8. Verification assessment

Existing positive evidence:

- static constant matches actual migration version；
- old JSON missing additive fields deserializes；
- legacy work restores end-to-end into schema 14；
- v1 structural JSON transform has focused tests；
- replace/merge restore behavior and multiple additive tables have tests；
- untrusted archive and parsed collections are bounded。

Gaps surfaced by the common checklist:

- no real old binary is run against a current backup；downgrade behavior is
  inferred from Serde and warning logic；
- this validation did not deliberately remove `#[serde(default)]` to observe the
  existing regression test fail；
- migration 0014 rollback/idempotence was not separately exercised here。

These are validation findings, not authorization to change Gentle.

## 9. Ownership result

- **Common workflow owns:** distinct identities, directional compatibility,
  migration/rollback questions, multi-layer bounds and evidence slots.
- **Gentle owns:** dual SQL migrations, defaults, downgrade promise, transform,
  size values, restore report and release decision.
- **Shared runtime change:** none implied.

## 10. Template verdict

The template represents additive schema evolution and structural backup migration
without imposing Deductree's exact-version refusal or a universal serializer. The
same version map and compatibility matrix remained usable in both examples.
