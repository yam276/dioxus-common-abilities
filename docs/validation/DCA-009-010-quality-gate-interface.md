# DCA-009 / DCA-010 Quality-gate Interface Validation

Status：boundary accepted; two-consumer implementation validation remains open

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

- **Entrypoint hypothesis:** independent repositories can expose the same
  selection, explanation, execution and failure semantics while retaining
  different local implementations. The source comparison accepts this boundary.
- **Matrix hypothesis:** one neutral model can select target-, feature-, runtime-
  and release-specific checks without fields named after a product technology.
  The source comparison accepts this boundary.
- **Single-artifact hypothesis:** the entrypoint and matrix should be merged into
  one candidate. The comparison rejects this. Invocation semantics and local
  impact knowledge have different owners, although they must be validated
  together.

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
  platform-specific arm was selected.

## Accepted responsibility boundary

`DCA-009` owns only the common invocation contract:

1. accept an explicit change class such as `docs`, `code` or `release` plus a
   repository-local changed-surface input;
2. ask the local `DCA-010` matrix for the required check IDs;
3. show why each check was selected before or while running it;
4. execute available automated checks in dependency order;
5. propagate command failures without rewriting them as success;
6. report required manual or unavailable checks as pending and refuse to call
   the complete gate green.

`DCA-010` owns the repository-local selection data:

- stable local check IDs;
- local commands, working directories and prerequisites;
- rule predicates over change class and changed surfaces;
- dependencies or ordering between checks;
- availability requirements and whether evidence is automated or an owner
  receipt.

The shared schema defines the meanings of those concepts. Each consumer owns
their values. A local ID such as `app-wasm-check` or `canvas-tests` is valid;
adding a shared `wasm` or `canvas` field is not.

## Minimal abstract model

This is a semantic model, not a decision on file syntax:

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

An explain/plan operation may finish after resolution. A run operation is
complete only when every required item passed or has an accepted receipt.
`pending`, `unavailable` and `failed` are all non-green terminal results for that
run. There is no silent host-only fallback.

## Relationship between the candidates

The candidates remain separate but paired:

- `DCA-009` is reusable even if a small repository computes its required checks
  directly;
- `DCA-010` can be inspected or linted without running commands;
- the gate consumes the matrix when a repository has conditional arms;
- neither candidate may advance to implementation independently of the opposing
  Cards and ShinyColors cases in this validation.

This separation also preserves source ownership. The common repository owns
the semantics and future conformance tests; the consumer repository remains the
only authority for its complete gate and release approval.

## Acceptance and next gate

This boundary-validation artifact passes because:

- Cards target/database/release arms and ShinyColors Canvas/feature/manual arms
  fit without product-specific shared fields;
- executable failures, unavailable environments and owner receipts have explicit
  non-green outcomes;
- check dependencies express release ordering without centralizing product
  policy;
- docs-only work remains an explicit selectable class;
- no implementation technology was selected prematurely.

Both candidates advance to `Validating`, not `Planned`. Their next gate is two
small consumer-owned prototypes that expose the same operations and results:

1. Cards rules prove an ordinary app change, a WASM-owned change and ordered
   release checks;
2. ShinyColors rules prove an ordinary Rust change, a Canvas-owned change and a
   required Windows owner receipt;
3. mutation tests show removing or inverting each special rule makes the
   corresponding selection test fail;
4. both entrypoints propagate one deliberate command failure and distinguish a
   planned run from a complete green run;
5. implementation plans are written only after those prototypes confirm that
   the semantic model needs no consumer-specific shared field.
