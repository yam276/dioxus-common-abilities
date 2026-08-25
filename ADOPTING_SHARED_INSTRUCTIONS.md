# Adopting the Shared Agent Instructions

`AGENTS.md` is the single source of truth for behavioral, Rust, Dioxus, testing,
and shared-ability rules that apply across the related product repositories.
Consumer repositories keep their product facts and exact commands locally.
Potential shared workflows discovered in those local rules are tracked in
`SHARED_WORKFLOW_CANDIDATES.md`; "local today" does not mean "never shared."
Unresolved shared capabilities and their lifecycle are governed only by
`COMMON_CAPABILITY_WISHLIST.md`.

## Include pattern

For repositories cloned as siblings of `dioxus-common-abilities`, place this at
the beginning of the consumer's root `AGENTS.md`:

```text
@../dioxus-common-abilities/AGENTS.md

# Project-specific instructions

...
```

For `CLAUDE.md`, use the same include pattern:

```text
@../dioxus-common-abilities/AGENTS.md

# Project-specific instructions

...
```

Adjust the relative path for repositories below another directory. For example,
`NewShiny/ShinyColors_diolama` uses:

```text
@../../dioxus-common-abilities/AGENTS.md
```

An absolute local path may be used on a single workstation, but a relative path
is preferable when the sibling clone layout is part of the workspace contract.

## Consumer file contents

After the include, the local instruction file should contain only facts and
rules that cannot be shared safely:

- Repository identity and structure.
- Exact quality-gate and target-matrix commands.
- Domain contracts and required reference documents.
- Persistence, schema, migration, backup, and compatibility rules.
- Credentials, security, destructive actions, and redaction policy.
- Generated files, CSS pipeline, asset rules, and platform quirks.
- Active plan locations, release flow, and versioning rules.

Local files may make a common rule stricter. If a product constraint requires a
different rule, state the conflict and reason explicitly next to the local rule.

## Migration procedure

1. Add the shared include to the repository root instruction file.
2. Compare existing `AGENTS.md` and `CLAUDE.md` content with the shared file.
3. Remove duplicated behavioral, Rust, and Dioxus guidance from the local file.
4. Keep all product-specific commands and contracts locally.
5. Confirm that both Codex and Claude instruction entrypoints still expose the
   local rules.
6. Test one small repository task and inspect the loaded instructions before
   migrating the next repository.

Do not replace every repository instruction file in one mechanical rewrite.
Migrate and verify one product at a time because instruction precedence and
required reference documents differ.

## Current repository mapping

| Repository | Current instruction shape | Recommended first migration |
|---|---|---|
| `Deductree` | Rich root `AGENTS.md` plus `CLAUDE.md` | Include shared base; retain gates, Diolama versioning, Story Editor governance, reference docs, and file contract locally |
| `gentle` | Rich root and crate-level `CLAUDE.md`; stock Dioxus `gentle-app/AGENTS.md` | Add a root `AGENTS.md`; keep identity, feature matrix, backup/schema, assets, and generated CSS rules locally |
| `gentle-cards` | Rich root and crate-level `CLAUDE.md`; stock Dioxus `gentle-cards-app/AGENTS.md` | Add a root `AGENTS.md`; keep fork roadmap, target matrix, card contracts, identity, and release rules locally |
| `oxdm` | Product rules in root `CLAUDE.md` | Add root `AGENTS.md`; retain ONVIF architecture, session reuse, credentials, i18n parity, and exact gate locally |
| `Pedigoo` | Behavioral and product rules combined in root `CLAUDE.md` | Add root `AGENTS.md`; retain simulation contracts, measurements, exact gates, and domain references locally |
| `ShinyColors_diolama` | Short root `CLAUDE.md` delegating to authoritative docs | Add root `AGENTS.md`; preserve authority order, Diolama boundary, portability, and release gate locally |

The stock Dioxus tutorial content currently duplicated in the two Gentle app
`AGENTS.md` files should not become part of the shared policy. Replace it with a
short shared include plus app-specific facts; rely on version-matched Dioxus
documentation for general framework tutorials.

## Updating the common layer

A proposed common rule should satisfy all of these conditions:

1. It applies to at least two independent product repositories, not only fork
   siblings.
2. It does not contain product paths, domain names, identities, or exact build
   commands.
3. A local repository can make it stricter without copying the whole section.
4. Its wording produces an observable development decision or verification
   behavior.

Change the central file first, then validate its effect in one consumer before
rolling it out across the remaining repositories.
