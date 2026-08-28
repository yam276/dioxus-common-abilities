# DCA-009 / DCA-010 Quality-gate Interface Validation

Status：feasibility reassessed; `DCA-009` remains viable as a convention and
`DCA-010` is deferred

Evidence date：2026-08-28

## Objective

Determine whether one quality-gate entrypoint (`DCA-009`) and a change-impact
target matrix (`DCA-010`) can share semantics across products without sharing
their commands, path policy or release authority.

This validation deliberately compares opposing consumers before selecting
shell, PowerShell, `just`, `cargo xtask` or a standalone program. The inspected
source revisions were:

- Gentle Cards `54fc8d460298e106b4a945a48d4e7b8525b2df62`;
- NewShiny/ShinyColors `ab5e3e5af70243c0037c1f8c0809f3c6caaf43c6`.

The NewShiny worktree contained unrelated user changes during inspection. Its
gate instructions and hook were unchanged from the recorded revision, and this
validation did not modify that repository.

## Hypotheses

- **Entrypoint convention hypothesis:** independent repositories can expose the
  same explicit `docs`, `code` and `release` intent plus honest pass/fail/pending
  results while retaining different local implementations. The feasibility
  probe accepts this narrow workflow boundary.
- **Shared entrypoint tool hypothesis:** a common executable adds enough behavior
  beyond a local wrapper to justify its dependency and cross-platform process
  surface. The feasibility probe rejects this with the current evidence.
- **Automatic matrix hypothesis:** one neutral model can safely infer target-,
  feature-, runtime- and release-specific checks from a changed-file set. The
  first source comparison showed that the cases can be represented, but the
  feasibility probe rejects completeness and maintenance value.
- **Single-artifact hypothesis:** the entrypoint and matrix should be merged into
  one required tool. The comparison rejects this because the viable entrypoint
  convention does not require an automatic matrix.

## Opposing consumer evidence

### Gentle Cards

Cards has multiple gate shapes rather than one universal command list:

| Change class | Required evidence | Boundary exposed |
|---|---|---|
| Ordinary app code | format, native lint and tests | baseline automated gate |
| Code under `cfg(target_arch = "wasm32")` | baseline plus `wasm32-unknown-unknown` web check | target-specific code is invisible to the host build |
| Core database code | separate SQLite and PostgreSQL feature/service arms | one package needs mutually different environments |
| Release | web release build before desktop release build, then fresh-folder/LAN smoke | check order and delivered-artifact evidence matter |
| Documentation only | no explicit local path today | the prototype must add a cheap class rather than infer the full code gate |

The checked-in CI covers native app and both database arms, but does not cover
the locally required WASM check. Root and nested instructions also present
different subsets of the gate. The repository therefore cannot treat CI, a
pre-commit hook or one prose block as the complete selector authority.

### ShinyColors

ShinyColors begins with a locked Rust gate, then adds checks according to the
surface being changed:

| Change class | Required evidence | Boundary exposed |
|---|---|---|
| Ordinary Rust code | format, locked check, all-target lint/tests and diff check | baseline automated gate |
| Canvas/UI intake | relevant JavaScript tests and mutation/coverage checks | a non-Rust runtime owns failures |
| Feature/profile boundary | feature-specific locked check | default features cannot prove optional code |
| Desktop milestone | locked build | compilation is not a packaged milestone |
| Canvas/WebView2 release behavior | Windows visible smoke with runtime, DPI, viewport, route and motion receipt | required evidence may be unavailable on the invoking host |

Its committed pre-commit hook checks only the private-authoring boundary. It is
valuable, but is neither the full code gate nor evidence that a Windows-visible
smoke passed. This is the strongest counterexample to equating "quality gate"
with "pre-commit" or "run every local command".

## Concrete feasibility probe

The second pass filled the proposed matrix with real selectors rather than only
checking whether the abstract fields could hold them.

| Consumer case | Narrow selector attempted | Completeness result | Safe alternative |
|---|---|---|---|
| Cards WASM | list the Rust files that currently contain `cfg(wasm32)` | unsafe: a new conditional in any other file is invisible until the list is manually updated | run the WASM check for every app Rust change |
| Cards database arms | select by `gentle-cards-core/**` | safe only as a coarse package boundary | run both supported database arms for every core change |
| Cards release order | infer from changed paths | impossible: release is caller intent, not a path property | explicit `release` class owns web-before-desktop and smoke |
| ShinyColors Canvas | select `src/canvas/**` | unsafe: `app.rs`, album models and runtime-content producers also change the Canvas contract | run Canvas checks for the repository's broad code class or let the active plan name them |
| ShinyColors visible smoke | select Canvas-related paths | incorrect: the smoke closes a named milestone and exact commit, not every edit to those paths | explicit release/milestone receipt owned by the active plan |
| ShinyColors targeted verifiers | map paths to the current plan's commands | high churn: the list evolves with each active feature and duplicates its acceptance criteria | keep feature-specific commands in the active plan |
| Pedigoo documentation only | select tracked documentation/instruction paths while rejecting executable inputs | feasible, but it is a change class and safety guard rather than a target matrix | explicit `docs` class plus a local changed-file guard |

The neutral schema was therefore structurally expressive but not operationally
complete. A schema that accepts arbitrary local predicates merely moves the
existing scripts behind another syntax. A schema restricted to paths silently
misses semantic producers. A conservative schema selects broad full gates and
collapses back into the explicit entrypoint convention.

## Rejected models

The comparison rejects these designs:

- a shared list of raw commands, because working directories, services,
  features and platform availability remain product-owned;
- product-specific schema fields such as `wasm`, `canvas` or `webview2`, because
  those names describe local checks rather than common semantics;
- path matching as the whole model, because release mode and environment/manual
  evidence are not inferred from paths alone;
- silently skipping an unavailable or manual check, because the result would
  claim completion without required evidence;
- treating a pre-commit hook or hosted CI as the sole gate authority;
- one opaque `full` mode, because callers cannot explain why a costly or
  platform-specific arm was selected;
- a mandatory shared automatic matrix, because safe selection in both studied
  repositories is either deliberately coarse or still requires product/plan
  judgment.

## Feasible `DCA-009` boundary

`DCA-009` remains feasible as a workflow/interface convention, not yet as a
shared executable. The common artifact may standardize only this contract:

1. accept an explicit `docs`, `code` or `release` class;
2. invoke the consumer-owned complete gate for that class;
3. propagate command failures without rewriting them as success;
4. report required manual or unavailable evidence as pending and refuse to call
   the complete gate green.

The repository owns its wrapper and all commands, order, services, features,
platforms, changed-file guards and receipts. The common layer does not parse
Rust, JavaScript, active plans or Git diffs to choose a smaller gate.

This still removes a real coordination cost: agents and humans get one
discoverable local entrypoint and do not reconstruct commands from several
instruction files. It does not claim to remove the repository's gate logic.

## `DCA-010` feasibility failure

The invariant behind `DCA-010` remains correct: a default host build does not
prove other targets, features or runtimes. The proposed common machine-readable
selector is not justified, however:

- exact path lists create a new stale authority;
- broad path rules are safe but add no selection value over `code`;
- semantic rules require arbitrary local programs or language-aware parsing;
- plan/milestone receipts cannot be derived from a diff;
- maintaining commands in both plans/instructions and a manifest increases
  drift until the manifest becomes the sole local authority, which neither
  consumer has demonstrated.

`DCA-010` is therefore deferred rather than promoted. It may reopen only after
two independent repositories already use local machine-readable matrices and
can show that those matrices safely avoid meaningful work without missing a
target-owning failure.

## Why the abstract model was insufficient

The first pass produced this neutral model:

```text
check:
  id: local stable name
  execution: automated command | owner receipt
  depends_on: local check IDs
  available_when: local environment predicate

rule:
  when: requested class plus local changed-surface predicate
  require: local check IDs

resolution result:
  ordered checks with reason and state
  state: runnable | passed | failed | pending | unavailable
```

Resolution must include transitive dependencies. For example, Cards can declare
that its desktop release build depends on its web release build without making
"web before desktop" a shared policy. ShinyColors can require a Windows owner
receipt without teaching the shared layer what WebView2 or DPI means.

That proves representability only. It does not prove how a rule knows that
`album/root.rs` affects Canvas, when a stage needs visible smoke or which active
plan verifier applies. Adding an arbitrary `when` hook makes the schema a task
runner around product code; restricting `when` makes it incomplete. The model
is retained here as a rejected spike, not an accepted implementation contract.

The result semantics remain useful for `DCA-009`: a run is green only when all
required automated work passed and all required receipts exist. `pending`,
`unavailable` and `failed` must never be silently converted to success.

## Cost/value result

| Candidate shape | Shared value | New cost | Feasibility result |
|---|---|---|---|
| Entrypoint convention and checklist | one discoverable name, explicit class and honest result vocabulary | a small local wrapper and brief repository declaration | positive; continue validation |
| Shared process-running executable | central command execution, ordering and receipt states | cross-platform quoting, environment/service setup, tool installation and another dependency | negative with current evidence |
| Automatic impact matrix | may avoid some expensive checks | stale selectors, semantic gaps, duplicate authorities and matrix tests | negative with current evidence |
| Consumer-owned full gates | conservative correctness and simple ownership | deliberate over-selection and longer runs | acceptable safe baseline |

The only demonstrated cross-repository saving is discoverability and result
discipline. Command orchestration and impact judgment remain larger and more
volatile than the shared portion.

## Decision

The candidates no longer advance as a mandatory pair:

- keep `DCA-009` at `Validating`, narrowed to a shared convention with
  consumer-owned wrappers;
- defer `DCA-010`; it is neither required by `DCA-009` nor ready for a shared
  schema/tool;
- prefer safe over-selection inside a local `code` gate until a repository has
  measured evidence that selection complexity pays for itself;
- keep plan-specific verification and release authority in the active plan.

This preserves source ownership: the common repository owns only the convention
and a future conformance checklist; each consumer remains the sole authority for
what its classes run and whether a release is approved.

## Acceptance and next gate

The feasibility reassessment passes because it attempted concrete selectors and
recorded the null result rather than treating structural expressiveness as proof
of reuse.

`DCA-009` has one remaining validation gate:

1. add two small consumer-owned wrappers in independent repositories;
2. both expose explicit `docs` and `code` behavior, clean failure propagation
   and no silent skip;
3. at least one exposes `release` with a required owner/platform receipt;
4. measure whether the convention removes duplicated instructions or caller
   mistakes without introducing a second command authority;
5. if it does, plan a template/conformance artifact; do not build a shared
   process runner from the current evidence.
