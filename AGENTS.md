# Shared Development Instructions for Dioxus Products

These instructions are the common behavioral and engineering baseline for the
Dioxus product repositories maintained together with this workspace. This file
is also the root instruction file for `dioxus-common-abilities` itself.

Consumer repositories should include this file, then add only their local
facts and stricter rules. Local instructions own exact quality-gate commands,
domain contracts, release procedures, generated-file policy, active planning
documents, credentials, and product-specific architecture.

When a local rule conflicts with this file because of a concrete product or
target constraint, follow the narrower local rule and make the reason explicit.
Do not silently weaken correctness, safety, or verification requirements.

# Session start

These repositories publish under the `yam276` GitHub identity. At the beginning
of a repository session, run:

```bash
git config user.email
```

The expected value in this workspace is
`59956724+yam276@users.noreply.github.com`. If it differs, stop before creating
commits or pushing and resolve the repository-local identity. Do not allow an
unrelated global identity to write project history.

# Behavioral guidelines

These principles apply to every task. They favor caution over speed; use
judgment for trivial work.

## 1. Think before coding

Do not assume or hide uncertainty. Surface tradeoffs before implementation.

- State assumptions explicitly.
- If several interpretations materially change the result, present them.
- Point out a simpler approach when one exists.
- Stop and ask when missing information would make the change unsafe or send it
  in a materially different direction.

## 2. Simplicity first

Write the minimum code that solves the verified problem.

- Do not add features, configurability, or error cases that were not requested
  and cannot occur.
- Do not create abstractions for hypothetical consumers.
- Prefer a direct implementation over a framework, registry, callback bag, or
  dependency-injection layer.
- If a solution is substantially larger than the behavior it provides,
  simplify it before shipping.

## 3. Surgical changes

Every changed line should trace to the current objective.

- Do not reformat, refactor, rename, or clean adjacent code without need.
- Match the repository's existing style.
- Preserve unrelated user changes in a dirty worktree.
- Remove imports, variables, functions, tests, and files made obsolete by your
  own change.
- Mention unrelated problems instead of fixing or deleting them without scope.

## 4. Goal-driven execution

Turn the request into observable success criteria and loop until they hold.

For multi-step work, state a short plan with a verification step for each item:

```text
1. Change -> verify: focused check
2. Change -> verify: consumer behavior
3. Finish -> verify: repository quality gate
```

Examples:

- Bug fix: reproduce the failure, implement the fix, prove the reproduction is
  now green.
- Validation: show that invalid input fails and valid input succeeds.
- Refactor: establish equivalent tests before and after.

## 5. Small, focused units

Prefer methods and components with one responsibility and explicit ownership of
state and side effects.

- Split unrelated responsibilities into named helpers or child components.
- Lift deeply nested blocks so call sites read as a sequence of named steps.
- A helper's name and signature should make clear what it reads, mutates, saves,
  or spawns.
- Extracting a helper to name a step is useful; speculative generalization is
  not.

## 6. Concrete tripwires

- **Extract before extending a large unit.** If the component or function being
  changed is already difficult to hold in one screen, first separate the part
  being touched. A rough warning is more than 300 lines or more than 10 local
  signals in one component; the unit, not the file, is the concern.
- **Treat a second near-copy as an abstraction decision.** Reuse an existing
  common ability, extract the shared core, or leave a short explanation of why
  the semantics intentionally diverge.
- **Do not swallow meaningful failure.** Surface a save, import, export, network,
  or background-operation result when the user cares whether it succeeded.
- **Do not over-persist view state.** Hover, drag, pan, zoom, focus, composition,
  and other high-frequency transient state do not belong in document history or
  per-frame persistence.
- **Make task lifetime explicit.** A caller must be able to tell whether work is
  component-scoped, cancellable, or intentionally detached.

## 7. Verify current evidence

Code and current measurements are the source of truth. Comments, plans,
changelogs, and prior answers are evidence to check, not facts to repeat from
memory.

- Read the implementation before describing or changing it.
- Re-run probes that a conclusion depends on.
- Correct stale comments or plans when the current task proves them wrong.
- Treat a null result as a valid result; do not keep code whose hypothesis failed.

# Testing and quality gates

The local repository instruction file must define the exact formatter, linter,
test, target, and packaging commands. The following discipline is common:

- Run the complete local quality gate before declaring executable work done.
- Warnings are errors unless a local toolchain documents a justified exception.
- For a regression guard, demonstrate that the test fails without the fix when
  practical. A test never seen fail may be vacuous.
- Check positive and negative cases, including empty inputs and boundary values.
- GUI behavior such as IME composition, focus, keyboard commands, window state,
  and mobile lifecycle needs proportionate runtime verification; compilation
  alone is insufficient.
- Documentation-only changes that do not alter code, dependencies, build
  configuration, generated artifacts, or executable fixtures may skip the code
  gate. Verify their diff, formatting, paths, and links instead.
- Run checks from an explicit absolute working directory. Do not rely on a shell
  session's remembered directory.

# Change and commit discipline

- Treat each independently verifiable objective as one target change.
- Do not combine unrelated features, fixes, refactors, or cleanup.
- Keep implementation, regression coverage, and documentation of the same
  behavior together.
- Do not commit a knowingly partial or failing target merely to create a small
  commit.
- Follow the user's request and local repository policy on whether to create a
  commit. When committing, use the local commit format and complete its gate
  first.

# Rust principles

- Avoid `unwrap()` and `expect()` in non-test code. Prefer `?`, `match`, `if let`,
  or an explicit fallback. An exception requires a locally proven invariant and
  a comment explaining why it cannot fail. Tests and examples are exempt.
- Avoid `Rc<RefCell<T>>` as a default. Prefer ownership, `&mut`, or message
  passing; use runtime borrow checking only when the state model genuinely
  requires it, and keep borrows short.
- Avoid `unsafe`. If no safe alternative exists, isolate it behind a safe API
  and document the invariant.
- Treat files, imported data, network responses, and persisted state as untrusted
  input. Parsing and validation paths must fail gracefully rather than panic.
- Use deterministic collections and ordering where output, persisted contracts,
  replay, or analysis must be reproducible.

# Dioxus principles

- Prefer composition and small components over a root component with many
  branches and local signals.
- Extract child components and helpers when `rsx!` becomes long or deeply nested.
- Keep component structure and prop flow shallow. Name intermediate pieces
  instead of hiding behavior in nested closures.
- Make signal ownership clear. A component should not mutate unrelated global
  state as a side effect of rendering.
- Controlled text input must account for IME composition. Do not let intermediate
  composition events submit, cancel, trigger shortcuts, or overwrite the final
  committed text.
- Async UI work must surface failure and state its lifetime. Do not use detached
  tasks merely to avoid borrow or scope errors.
- Domain cores remain independent of Dioxus. Dioxus adapters may depend on
  domain cores, never the reverse.

# Shared ability boundaries

Before writing a second implementation of cross-product Dioxus infrastructure,
search `dioxus-common-abilities` for an existing crate or documented candidate.

- A common crate owns a small reusable mechanism and its correctness invariant.
- The product owns domain types, policy, persistence schema, paths, router, root
  context, styling, copy, and UX composition.
- Do not distort a common API to eliminate a small product adapter.
- Do not make an unrelated product depend on a domain platform such as Diolama
  merely to obtain a generic-looking helper.
- Apps depend directly on the small abilities they use. Do not create an
  umbrella app framework until several independently validated crates are
  repeatedly composed in the same way.

# Generic base and default preset

Every highly customizable shared surface should provide both:

1. A generic, overridable base API.
2. A ready-made, zero-configuration preset for the common case.

The preset must be built from the public base API and remain optional. This rule
does not require a preset for a tiny ability that has only one correct use.

# UI symbols

Do not introduce emoji or ad-hoc dingbat glyphs in product UI, source comments,
documentation, commit messages, or sample content. Use the repository's icon
system, normally `dioxus-free-icons`, and use CSS color or shape for state.

# Exact values that remain local

The exact values below remain local even when their surrounding workflow later
becomes shared. Candidate workflows are tracked in
`SHARED_WORKFLOW_CANDIDATES.md`.

- Exact Cargo, `dx`, feature-matrix, platform, audit, or packaging commands.
- Branch, release, signing, and publishing rules.
- Active roadmap locations and required domain reference documents.
- Database, file-format, migration, backup, wire, and version contracts.
- Product-specific architecture, terminology, security, credentials, and
  destructive-action policy.
- CSS pipelines, generated assets, icons, localization keys, and repository
  layout facts.
- Known product or dependency quirks.

Each consumer repository must keep these facts in its own root or nearest-scope
instruction file.
