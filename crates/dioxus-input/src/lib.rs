//! Dioxus input-correctness primitives.
//!
//! [`ImeGuard`] protects controlled text inputs from intermediate IME writes,
//! recovers the final committed fragment from `compositionend`, and suppresses
//! local keyboard commands while composition is active.

use dioxus::prelude::{
    use_signal, CompositionEvent, KeyboardEvent, ReadableExt, Signal, WritableExt,
};

/// Composition state for a controlled input.
///
/// Create one guard per input with [`use_ime_guard`]. State reads are
/// non-reactive, so starting or ending composition does not itself re-render
/// the controlled input and overwrite the browser's in-flight IME buffer.
#[derive(Clone, Copy)]
pub struct ImeGuard {
    state: Signal<ImeState>,
}

impl ImeGuard {
    /// Returns whether an IME composition session is active.
    pub fn is_composing(&self) -> bool {
        self.state.peek().is_composing()
    }

    /// Returns whether an `input` event may update the controlled value.
    pub fn allows_input(&self) -> bool {
        self.state.peek().allows_input()
    }

    /// Returns whether a local keyboard handler may run product commands.
    ///
    /// Both the guard state and Dioxus' event-level composition flag must be
    /// clear. Checking both covers event ordering where one source changes
    /// before the other.
    pub fn allows_keyboard_event(&self, event: &KeyboardEvent) -> bool {
        self.state
            .peek()
            .allows_keyboard_event(event.is_composing())
    }

    /// Marks the beginning of an IME composition session.
    ///
    /// Wire this to `oncompositionstart`.
    pub fn handle_composition_start(&mut self) {
        self.state.write().start();
    }

    /// Clears composition state and returns the committed text fragment.
    ///
    /// Wire this to `oncompositionend`. Returning the event data directly
    /// preserves the final text on WebView2 versions that omit the follow-up
    /// `input` event. The caller owns append, replace, parsing and validation
    /// semantics.
    pub fn handle_composition_end(&mut self, event: CompositionEvent) -> Option<String> {
        self.state.write().finish(event.data().data())
    }
}

/// Creates one composition guard owned by the current Dioxus component.
pub fn use_ime_guard() -> ImeGuard {
    ImeGuard {
        state: use_signal(ImeState::default),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ImeState {
    composing: bool,
}

impl ImeState {
    fn is_composing(self) -> bool {
        self.composing
    }

    fn allows_input(self) -> bool {
        !self.composing
    }

    fn allows_keyboard_event(self, event_is_composing: bool) -> bool {
        !self.composing && !event_is_composing
    }

    fn start(&mut self) {
        self.composing = true;
    }

    fn finish(&mut self, committed: String) -> Option<String> {
        self.composing = false;
        (!committed.is_empty()).then_some(committed)
    }
}

#[cfg(test)]
mod tests {
    use super::ImeState;

    #[test]
    fn idle_state_allows_input_and_non_composing_keys() {
        let state = ImeState::default();

        assert!(!state.is_composing());
        assert!(state.allows_input());
        assert!(state.allows_keyboard_event(false));
    }

    #[test]
    fn start_blocks_input_and_keys() {
        let mut state = ImeState::default();

        state.start();

        assert!(state.is_composing());
        assert!(!state.allows_input());
        assert!(!state.allows_keyboard_event(false));
    }

    #[test]
    fn finish_returns_exact_fragment_and_clears_state() {
        let mut state = ImeState::default();
        state.start();

        let committed = state.finish("攻擊".to_string());

        assert_eq!(committed.as_deref(), Some("攻擊"));
        assert!(!state.is_composing());
        assert!(state.allows_input());
    }

    #[test]
    fn empty_finish_clears_state_without_committing() {
        let mut state = ImeState::default();
        state.start();

        let committed = state.finish(String::new());

        assert_eq!(committed, None);
        assert!(!state.is_composing());
        assert!(state.allows_input());
    }

    #[test]
    fn event_composition_flag_blocks_keys_after_guard_clears() {
        let state = ImeState::default();

        assert!(!state.allows_keyboard_event(true));
        assert!(state.allows_keyboard_event(false));
    }
}
