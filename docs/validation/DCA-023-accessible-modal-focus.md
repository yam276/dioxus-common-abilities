# DCA-023 Accessible Modal Focus Validation

Status：validation complete; narrow focus-scope boundary accepted for planning

Evidence date：2026-08-26

Dioxus baseline：`0.7.9`

Upstream revision：`DioxusLabs/dioxus-components@bf007c15d0cf4d04d3181cc46cf12325aa773955`

## Objective

Determine whether the first-party `dioxus-primitives::dialog` behavior can own
the shared `DCA-023` focus lifecycle without a new common implementation. This
round is validation only: it does not change a consumer, create a production
crate or accept an implementation plan.

The standalone fixture is
`validation/dca-023-focus-lifecycle`. It is outside the production Cargo
workspace and pins the exact upstream revision above.

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

## Runtime matrix

The fixture was served with `dx serve --web` and exercised through the in-app
Chromium browser. Element IDs below are stable fixture probes.

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

Proceed with the implementation plan in `docs/active/DCA-023-focus-scope.md`.
The crate must first pass the standalone browser fixture's trusted forward and
reverse Tab matrix, nested-scope checks, no-tabbable fallback and restoration
checks. Cards and Deductree are the required consumers; OxDM is explicitly out
of scope. Consumer probes may be temporary, but each adoption must preserve its
repository's complete quality gate.
