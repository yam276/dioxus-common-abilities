//! Pointer-origin state for reliable backdrop dismissal.
//!
//! [`BackdropDismissState`] emits a dismiss intent only when the same pointer
//! begins and ends on a backdrop. Rendering, close policy, focus, keyboard
//! behavior and styling remain with the consumer.

use std::collections::BTreeSet;

/// Tracks pointer gestures that began on a backdrop.
///
/// Create one state value per mounted overlay. Feed both backdrop and content
/// pointer events into it, then close only when [`Self::pointer_up_on_backdrop`]
/// returns `true`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BackdropDismissState {
    backdrop_pointers: BTreeSet<i32>,
}

impl BackdropDismissState {
    /// Records that `pointer_id` began on the backdrop.
    pub fn pointer_down_on_backdrop(&mut self, pointer_id: i32) {
        self.backdrop_pointers.insert(pointer_id);
    }

    /// Records that `pointer_id` began inside the overlay content.
    ///
    /// Clearing the same ID also makes a reused pointer safe after a previous
    /// gesture ended outside the application without a terminal event.
    pub fn pointer_down_on_content(&mut self, pointer_id: i32) {
        self.backdrop_pointers.remove(&pointer_id);
    }

    /// Finishes `pointer_id` on the backdrop.
    ///
    /// Returns `true` exactly once when that pointer also began on the
    /// backdrop. A different pointer's active gesture is not changed.
    pub fn pointer_up_on_backdrop(&mut self, pointer_id: i32) -> bool {
        self.backdrop_pointers.remove(&pointer_id)
    }

    /// Finishes `pointer_id` inside the overlay content without dismissing.
    pub fn pointer_up_on_content(&mut self, pointer_id: i32) {
        self.backdrop_pointers.remove(&pointer_id);
    }

    /// Cancels `pointer_id` without dismissing.
    pub fn pointer_cancel(&mut self, pointer_id: i32) {
        self.backdrop_pointers.remove(&pointer_id);
    }
}

#[cfg(test)]
mod tests {
    use super::BackdropDismissState;

    #[test]
    fn matching_backdrop_gesture_dismisses_once() {
        let mut state = BackdropDismissState::default();
        state.pointer_down_on_backdrop(1);

        assert!(state.pointer_up_on_backdrop(1));
        assert!(!state.pointer_up_on_backdrop(1));
    }

    #[test]
    fn content_start_never_dismisses_on_backdrop_release() {
        let mut state = BackdropDismissState::default();
        state.pointer_down_on_content(1);

        assert!(!state.pointer_up_on_backdrop(1));
    }

    #[test]
    fn content_release_clears_backdrop_start_without_dismissing() {
        let mut state = BackdropDismissState::default();
        state.pointer_down_on_backdrop(1);

        state.pointer_up_on_content(1);

        assert!(!state.pointer_up_on_backdrop(1));
    }

    #[test]
    fn cancellation_clears_only_the_cancelled_pointer() {
        let mut state = BackdropDismissState::default();
        state.pointer_down_on_backdrop(1);
        state.pointer_down_on_backdrop(2);

        state.pointer_cancel(1);

        assert!(!state.pointer_up_on_backdrop(1));
        assert!(state.pointer_up_on_backdrop(2));
    }

    #[test]
    fn mismatched_release_does_not_dismiss_or_erase_another_pointer() {
        let mut state = BackdropDismissState::default();
        state.pointer_down_on_backdrop(1);

        assert!(!state.pointer_up_on_backdrop(2));
        assert!(state.pointer_up_on_backdrop(1));
    }

    #[test]
    fn content_start_clears_only_its_reused_pointer_id() {
        let mut state = BackdropDismissState::default();
        state.pointer_down_on_backdrop(1);
        state.pointer_down_on_backdrop(2);

        state.pointer_down_on_content(1);

        assert!(!state.pointer_up_on_backdrop(1));
        assert!(state.pointer_up_on_backdrop(2));
    }
}
