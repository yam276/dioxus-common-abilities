# DCA-023 Accessible Modal Focus Validation

Status：shared fixture and Gentle Cards green; Deductree adoption remains open

Evidence date：2026-08-28

Dioxus baseline：`0.7.9`

Upstream revision：`DioxusLabs/dioxus-components@bf007c15d0cf4d04d3181cc46cf12325aa773955`

## Original upstream objective

Determine whether the first-party `dioxus-primitives::dialog` behavior can own
the shared `DCA-023` focus lifecycle without a new common implementation. This
round is validation only: it does not change a consumer, create a production
crate or accept an implementation plan.

The original standalone fixture lived at
`validation/dca-023-focus-lifecycle`, outside the production Cargo workspace,
and pinned the exact upstream revision above. After rejection, the same stable
scenarios became the runtime acceptance fixture for `dioxus-focus-scope`.

## Hypothesis

The upstream dialog is an acceptable common boundary if it provides all of the
following in a real browser:

- dialog semantics and an accessible title/description relationship;
- initial focus inside the active modal;
- forward and reverse Tab containment;
- only the innermost focus scope reacting when dialogs are nested;
- Escape closing only the active dialog;
- restoration to the opener after teardown;
- a safe focus fallback when no tabbable descendant exists.

Product policy remains outside the boundary: whether Escape or outside
dismissal is enabled, destructive-action generations, busy state, default
button choice, styling, copy and layer placement.

## Upstream runtime matrix

The pinned upstream fixture was served with `dx serve --web` and exercised in a
real Chromium browser. Element IDs below remain stable fixture probes.

| Scenario | Expected | Observed | Result |
|---|---|---|---|
| Basic initial focus | Focus enters the first action | `open-basic` to `basic-first` | Pass |
| Basic forward Tab | Focus stays in the dialog and wraps | `basic-first`, `basic-last`, `close-basic`, `basic-first` | Pass |
| Basic reverse Tab | Shift+Tab wraps to the last action | `basic-first` to `close-basic` | Pass |
| Basic Escape and restoration | Dialog closes and focus returns to its opener | zero dialogs; active element `open-basic` | Pass |
| Dialog semantics | `role=dialog`, `aria-modal=true`, labelled and described | all attributes present and both relationships generated | Pass |
| Nested initial focus | Each new scope receives focus | outer `open-inner`; inner `inner-first` | Pass |
| Nested reverse Tab | Focus remains inside the inner scope | `inner-first`, `close-inner`, `inner-last`, `inner-first` | Pass |
| Nested forward Tab | Focus remains inside the inner scope | first Tab moved from `inner-first` to outer `open-inner` | Fail |
| Nested Escape while contained | Only inner closes, then focus returns to its opener | one outer dialog remains; active element `open-inner` | Pass |
| No tabbable descendants | Focus still enters a safe element in the modal scope | focus stayed on outside opener `open-empty`; dialog has no `tabindex` fallback | Fail |
| Browser diagnostics | No unexpected console warning or error | none observed | Pass |

The nested forward-Tab failure is sufficient to reject direct adoption. Once
focus reaches an outer control, outside-focus dismissal and Escape policy can
interact with the wrong scope. That follow-on behavior was not attributed to a
single mechanism in this round because the fixture did not sample dialog count
between every intermediate focus event.

The empty-dialog case is also a real boundary failure. A modal with explanatory
content but no button must not leave keyboard focus behind the active modal.

## Decision

Do not adopt the complete upstream `DialogRoot`/`DialogContent` behavior as the
shared `DCA-023` contract. It is useful source and behavior evidence, and its
single-dialog semantics are a strong baseline, but the tested revision fails
two required cases: nested forward containment and the no-tabbable fallback.

The first-party component remains rejected as the direct shared dependency. The
consumer evidence below now supports planning a narrower shared mechanism,
`dioxus-focus-scope`, rather than another complete modal component.

The likely reusable owner is narrower than a modal component: a focus-scope
mechanism responsible for initial focus, dynamic tabbable discovery, nested
scope activation and safe restoration. ARIA markup can remain a composition
checklist unless consumer validation proves a helper removes meaningful
duplication. Escape, outside dismissal and domain cancellation remain local
policy.

## Shared crate runtime matrix

The fixture now consumes `dioxus-focus-scope` by a local path while remaining
outside the production workspace. It composes `role`, `aria-modal`, accessible
names and Escape callbacks itself. Trusted browser keyboard input produced the
following receipt:

| Scenario | Observed | Result |
|---|---|---|
| Default initial focus | Basic scope focused its first current tabbable, `basic-first` | Pass |
| Preferred initial focus | Dynamic scope focused `dynamic-toggle` even though `close-dynamic` precedes it | Pass |
| Basic forward Tab | `basic-first`, `basic-last`, `close-basic`, `basic-first` | Pass |
| Basic reverse Tab | `basic-first` to `close-basic` | Pass |
| Basic Escape and restoration | Fixture-owned Escape closed the scope and restored `open-basic` | Pass |
| Nested initial focus | Outer focused `open-inner`; inner focused `inner-first` | Pass |
| Nested forward and reverse containment | Inner wrapped between `inner-first` and `close-inner` without entering outer controls | Pass |
| Nested teardown | Inner restored `open-inner`; outer resumed containment and restored `open-outer` | Pass |
| No tabbable descendants | The `div[role=dialog][tabindex=-1]` root retained forward and reverse Tab | Pass |
| Dynamic control added | From `dynamic-toggle`, Tab reached newly mounted `dynamic-extra`, then wrapped to `close-dynamic` | Pass |
| Dynamic control removed | After removal, Shift+Tab from `dynamic-toggle` reached current `close-dynamic` | Pass |
| Disabled preferred target | Disabled `orphan-disabled` fell back to enabled `close-orphan` | Pass |
| Disconnected opener | Closing removed `open-orphan`; focus remained on connected `body`, never the detached node | Pass |
| Browser diagnostics | A fresh page emitted no warning or error from the fixture origin | Pass |

The browser environment emitted unrelated extension warnings, and an earlier
page recorded the expected dev-server disconnect when the fixture server was
restarted. Neither came from the fixture origin; the final fresh-page receipt
was clean.

## Consumer validation

### Gentle Cards

Gentle Cards `9649afcdcdaf3cf365f55d4313ec6be998abbf23` was built as its real web
application and tested on the Settings recycle-bin confirmation dialog. No
source or local data was changed.

| Check | Observed | Result |
|---|---|---|
| Initial focus | The confirm body receives focus as a `div[tabindex="0"]` | Pass |
| Dialog semantics | No `role=dialog`, `aria-modal` or accessible-name relationship exists | Fail |
| Escape dismissal | Escape removes the modal | Pass |
| Focus restoration | Active focus becomes `body`, not the recycle-bin button | Fail |
| Console diagnostics | No warning or error was emitted | Pass |
| Tab containment | Browser automation did not produce default Tab movement even on an outside settings tab control | Not measured |

The Tab result is deliberately not inferred from an invalid control path.
Source inspection still establishes that the Cards `Modal` contains no Tab
trap, and that focusable settings controls remain mounted behind the overlay.
The current `ConfirmDialog` also handles Escape inside its focusable body while
the outer `Modal` handles the same bubbling key, so cancellation needs one
explicit owner before a shared focus scope is introduced.

This consumer run confirms that Cards is a good first adopter: it already has
an internal initial-focus target, but semantics, containment and opener
restoration are separate missing capabilities.

#### Adoption receipt

Gentle Cards commit `54fc8d460298e106b4a945a48d4e7b8525b2df62`
pins all three common Dioxus abilities to reviewed shared revision
`acd3e513fab7ec370c4e7a241ed5585770a7a75a`. Its existing modal root now owns
the shared scope lifecycle while Cards retains backdrop dismissal, Escape,
confirmation actions, styling and dialog markup.

| Check | Observed | Result |
|---|---|---|
| Initial target | The existing focusable confirmation body remains the preferred target and first scope control | Pass |
| Forward and reverse containment | Manual desktop Tab and Shift+Tab traversal remained inside the dialog and wrapped in both directions | Pass |
| Dialog semantics | The consumer emits `role=dialog`, `aria-modal=true` and an accessible label; all three are present in the final web build | Pass |
| Escape ownership | The duplicate confirmation-body Escape branch was removed; one Escape closed the dialog once | Pass |
| Focus restoration | Closing returned focus to the button that opened the confirmation | Pass |
| Backdrop policy | Pressing inside and releasing outside still did not dismiss the dialog | Pass |
| Automated gate | Format, warning-free Clippy and five test suites with 109 passing tests completed | Pass |
| Web target | A complete Dioxus `0.7.9` web build completed with the shared activation and cleanup scripts in the final WASM | Pass |

The interactive rows are the user's foreground desktop receipt from
2026-08-28. They supplement rather than replace the standalone trusted-browser
matrix, which already covers dynamic, empty, disabled and disconnected cases.

### Deductree Cast Library

Deductree `7aecf3705b47f7699076a10a522c110004cc76ed` was inspected and exercised at
the actual Story Editor nested surface. The runtime probe launched the desktop
WebView hidden, with macOS configured not to activate over the current app. It
added one in-memory character, did not mark the story edited, wrote only the
receipt below to stderr and was removed after the run.

The outer Cast Library carries `role=dialog`, `aria-modal` and a labelled title.
The nested asset picker carries `role=dialog` and `aria-modal`, but no accessible
name. Neither panel is a focus target, moves initial focus, traps Tab, scopes
Escape or restores focus to its opener. Both layers remain simultaneously
mounted while the picker is open.

| Check | Observed | Result |
|---|---|---|
| Outer initial focus | `document.activeElement` remained `body`; it was outside `.cast-library` | Fail |
| Outer fallback target | `.cast-library` had no `tabindex` | Fail |
| Nested initial focus | Focus remained on the outer asset-picker opener, outside `.cast-picker` | Fail |
| Nested accessible name | The picker had neither `aria-label` nor `aria-labelledby` | Fail |
| Tab interception | A cancelable bubbling Tab event was not prevented and dispatch returned normally | Fail |
| Nested restoration | After focusing the picker close button and closing the picker, focus became `body`, not the stored outer opener | Fail |

The synthetic Tab dispatch is not treated as a native traversal trace; browsers
do not perform trusted default Tab navigation for a synthetic event. It does
prove that the real surface installs no containment handler. Together with the
active-element observations before and after the nested lifecycle, this is the
required failing-before-fix desktop receipt.

## Next gate

Pin the same reviewed shared revision in Deductree and replace only the outer
Cast Library and nested asset picker's focus lifecycles. Preserve their existing
backdrop and catalog-edit policy, add the nested picker's product-owned
accessible name, and record the hidden desktop-WebView nested receipt. OxDM
remains explicitly out of scope.
