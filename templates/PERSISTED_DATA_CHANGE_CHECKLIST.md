# Persisted-data Change Checklist

Catalog workflow：`DCA-011`

Copy this template into the owning repository's active change plan. Replace every
placeholder or mark it `Not applicable` with a reason. The completed copy belongs
to the consumer; this template remains product-neutral.

## 1. Change identity

- **Consumer repository:** `<repo>`
- **Change owner:** `<crate/module/team>`
- **Change summary:** `<observable persisted behavior>`
- **Data crosses:** `<process restart / app upgrade / export / network / community>`
- **Trust boundary:** `<trusted local / untrusted import / secret / public contract>`

## 2. Version identities

List every identity touched or explicitly unaffected.

| Identity | Owner and source of truth | Current | Proposed | Bump trigger | Touched? |
|---|---|---:|---:|---|---|
| Package/executable | `<Cargo metadata or other authority>` | `<value>` | `<value>` | `<rule>` | `<yes/no>` |
| Database schema | `<migration authority>` | `<value>` | `<value>` | `<rule>` | `<yes/no/n/a>` |
| Document/wire format | `<format authority>` | `<value>` | `<value>` | `<rule>` | `<yes/no/n/a>` |
| Backup/save format | `<format authority>` | `<value>` | `<value>` | `<rule>` | `<yes/no/n/a>` |
| Request/catalog identity | `<catalog authority>` | `<value>` | `<value>` | `<rule>` | `<yes/no/n/a>` |
| Other | `<authority>` | `<value>` | `<value>` | `<rule>` | `<yes/no/n/a>` |

Explain why untouched identities do not need to change. Package SemVer alone is
not an explanation for a persisted-format decision.

## 3. Authoritative representation

- **Schema/format source:** `<migration files / Rust types / schema / parser>`
- **Reader entrypoint:** `<path and symbol>`
- **Writer entrypoint:** `<path and symbol>`
- **Migration entrypoint:** `<path and symbol or none>`
- **Validation/checker entrypoint:** `<path and symbol>`
- **Released fixtures:** `<paths>`
- **Documentation contract:** `<paths>`

If code, schema, fixtures and prose disagree, record the disagreement and resolve
the authority before implementing.

## 4. Change classification

- **Shape:** `<additive / rename / removal / type change / nesting / semantic reinterpretation>`
- **Default source:** `<database / Serde / parser / migration / none>`
- **Unknown-field policy:** `<ignore / preserve / warn / reject>`
- **Unknown-version policy:** `<warn / reject / cannot verify / other>`
- **Released migration modified?** `<must normally be no>`
- **Secret material involved?** `<yes/no>`
- **Generated or derived data involved?** `<yes/no>`

Semantic reinterpretation counts as a contract change even when the serialized
shape is unchanged.

## 5. Compatibility matrix

Do not summarize both directions as only "backward compatible."

| Writer/data | Reader/app | Expected result | Warning or refusal | Evidence |
|---|---|---|---|---|
| Old | New | `<result>` | `<UX>` | `<fixture/test>` |
| New | Old | `<result>` | `<UX>` | `<fixture/test>` |
| Current | Current | `<result>` | `<UX>` | `<round-trip/test>` |
| Older same-version shape | Current | `<result>` | `<UX>` | `<missing-field fixture>` |
| Current shape | Older same-version reader | `<result>` | `<UX>` | `<unknown-field fixture or documented gap>` |
| Unknown version | Current | `<result>` | `<UX>` | `<negative fixture>` |
| Malformed/truncated | Current | `<result>` | `<UX>` | `<negative fixture>` |

If downgrade is unsupported, state the bounded refusal behavior rather than
leaving the cell blank.

## 6. Migration and rollback

- **Migration strategy:** `<automatic / explicit tool / lazy / refuse>`
- **Migration is idempotent:** `<evidence>`
- **Partial failure behavior:** `<transaction / recovery / receipt>`
- **Rollback/downgrade strategy:** `<strategy or explicit unsupported boundary>`
- **Backup created before mutation:** `<yes/no/not applicable and why>`
- **Old data retained until success:** `<yes/no/not applicable and why>`
- **Secret migration ordering:** `<new secret write succeeds before old plaintext removal, or n/a>`
- **Write outcome is observable:** `<typed result / receipt / explicit documented gap>`
- **Repeated-open/repeated-import behavior:** `<evidence>`

For multi-backend products, enumerate every backend and prove equivalent intended
behavior.

## 7. Safety and resource bounds

- **Raw input/upload byte cap:** `<bound before parse or documented gap>`
- **Archive entry/decompressed-size cap:** `<bound or not applicable>`
- **Parsed count/depth/string caps:** `<bounds>`
- **Timeout or work bound:** `<bound>`
- **Allocation amplification risk:** `<analysis>`
- **Panic-free malformed-input path:** `<test/code evidence>`
- **Deterministic output required:** `<yes/no and ordering rule>`
- **Secret fields excluded from logs/export/backup:** `<evidence or n/a>`
- **Failure result reaches the user/caller:** `<Result/outcome/UI>`

Use "cannot verify" or a bounded refusal when the product cannot safely establish
the result.

## 8. Verification plan

### Positive cases

- [ ] Existing released fixture opens/restores.
- [ ] New fixture opens/restores.
- [ ] Current writer-reader round trip preserves required data.
- [ ] Migration produces the intended current representation.

### Negative cases

- [ ] Remove or invert the new guard and observe the regression test fail.
- [ ] Unknown version follows the documented policy.
- [ ] Missing new field follows the documented default policy.
- [ ] Unknown field follows the documented preservation/ignore/reject policy.
- [ ] Malformed or truncated input fails without panic.
- [ ] Raw input and archive expansion hit their pre-parse/pre-allocation bounds.
- [ ] Oversized/deep input hits the documented bound.
- [ ] Partial migration failure leaves a recoverable state.
- [ ] Secret material does not appear in logs, exports or ordinary config.
- [ ] Secret migration failure retains a recoverable authority and reaches the caller.
- [ ] Non-secret metadata cannot falsely claim unavailable secret material is persisted.

Mark genuinely irrelevant cases `Not applicable` and explain the invariant that
makes them impossible.

## 9. Product and common ownership

- **Common workflow owns:** questions, evidence structure and lifecycle gate.
- **Product owns:** `<schema, values, policy, code, UX and release decision>`
- **Shared crate/API change required:** `<DCA ID or no>`
- **Consumer adapter required:** `<path or no>`
- **Domain catalog/roadmap update required:** `<path or no>`

Do not move a domain compatibility promise into `dioxus-common-abilities` merely
because this checklist exposed it.

## 10. Decision

- **Compatibility decision:** `<accepted / revise / reject>`
- **Unresolved risks:** `<list or none>`
- **Required quality gate:** `<local exact commands>`
- **Runtime/manual validation:** `<scenario and environment>`
- **Approver/owner:** `<name or authority>`
- **Evidence date and revision:** `<date and commit>`

Implementation may start only after the owning repository accepts this completed
checklist through its local planning and quality-gate rules.
