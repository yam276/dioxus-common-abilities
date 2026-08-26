//! Focus lifecycle for Dioxus overlays without modal policy or markup.
//!
//! [`use_focus_scope`] creates a zero-configuration scope. Consumers that need
//! a specific initial target can use [`use_focus_scope_with_options`]. The
//! consumer keeps ownership of its root element, dialog semantics, Escape and
//! close policy.

use dioxus::prelude::{use_drop, use_hook};
use std::sync::atomic::{AtomicU64, Ordering};

const ACTIVATE_JS: &str = include_str!("activate.js");
const DEACTIVATE_JS: &str = include_str!("deactivate.js");
const SCOPE_ID_TOKEN: &str = "__DIOXUS_FOCUS_SCOPE_ID__";
const INITIAL_ID_TOKEN: &str = "__DIOXUS_FOCUS_INITIAL_ID__";
static NEXT_SCOPE_ID: AtomicU64 = AtomicU64::new(1);

/// Optional focus behavior chosen by the consumer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FocusScopeOptions {
    initial_focus_id: Option<String>,
}

impl FocusScopeOptions {
    /// Prefer the descendant with this HTML `id` when the scope activates.
    ///
    /// A missing, hidden or disabled target falls back to the first tabbable
    /// descendant and then to the scope root.
    pub fn with_initial_focus_id(mut self, id: impl Into<String>) -> Self {
        self.initial_focus_id = Some(id.into());
        self
    }
}

/// One focus scope owned by the current Dioxus component lifecycle.
///
/// Apply [`Self::root_id`] and [`Self::root_tab_index`] to the existing scope
/// root, then call [`Self::activate`] from that element's `onmounted` handler.
/// The component that calls the hook must unmount with the root so cleanup and
/// focus restoration run at the same boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusScope {
    root_id: String,
    initial_focus_id: Option<String>,
}

impl FocusScope {
    /// Stable HTML `id` for the consumer-owned scope root.
    pub fn root_id(&self) -> &str {
        &self.root_id
    }

    /// The root's required fallback tab index.
    pub const fn root_tab_index(&self) -> i32 {
        -1
    }

    /// Activates the mounted scope and moves focus inside it.
    ///
    /// Call this once from the root element's `onmounted` handler.
    pub fn activate(&self) {
        let script = activation_script(&self.root_id, self.initial_focus_id.as_deref());
        let _ = dioxus::document::eval(&script);
    }
}

/// Creates a focus scope that initially focuses its first tabbable descendant.
pub fn use_focus_scope() -> FocusScope {
    use_focus_scope_with_options(FocusScopeOptions::default())
}

/// Creates a focus scope with consumer-selected initial-focus options.
pub fn use_focus_scope_with_options(options: FocusScopeOptions) -> FocusScope {
    let root_id = use_hook(next_scope_id);
    let cleanup_id = root_id.clone();
    use_drop(move || {
        let script = deactivation_script(&cleanup_id);
        let _ = dioxus::document::eval(&script);
    });
    FocusScope {
        root_id,
        initial_focus_id: options.initial_focus_id,
    }
}

fn next_scope_id() -> String {
    let value = NEXT_SCOPE_ID.fetch_add(1, Ordering::Relaxed);
    format!("dioxus-focus-scope-{value}")
}

fn activation_script(root_id: &str, initial_focus_id: Option<&str>) -> String {
    ACTIVATE_JS
        .replace(SCOPE_ID_TOKEN, &js_string_literal(root_id))
        .replace(
            INITIAL_ID_TOKEN,
            &initial_focus_id.map_or_else(|| "null".to_string(), js_string_literal),
        )
}

fn deactivation_script(root_id: &str) -> String {
    DEACTIVATE_JS.replace(SCOPE_ID_TOKEN, &js_string_literal(root_id))
}

fn js_string_literal(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{2028}' => output.push_str("\\u2028"),
            '\u{2029}' => output.push_str("\\u2029"),
            control if control <= '\u{1f}' => push_u16_escape(&mut output, control as u16),
            other => output.push(other),
        }
    }
    output.push('"');
    output
}

fn push_u16_escape(output: &mut String, value: u16) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    output.push_str("\\u");
    for shift in [12, 8, 4, 0] {
        output.push(HEX[((value >> shift) & 0x0f) as usize] as char);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_scope_ids_are_distinct_and_safe() {
        let first = next_scope_id();
        let second = next_scope_id();

        assert_ne!(first, second);
        assert!(first.starts_with("dioxus-focus-scope-"));
        assert!(first.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
        }));
    }

    #[test]
    fn javascript_literals_escape_code_and_line_boundaries() {
        let literal = js_string_literal("target\"\\\n\u{0001}\u{2028}");

        assert_eq!(literal, "\"target\\\"\\\\\\n\\u0001\\u2028\"");
    }

    #[test]
    fn activation_uses_optional_initial_target() {
        let script = activation_script("scope", Some("primary-action"));

        assert!(script.contains("const scopeId = \"scope\";"));
        assert!(script.contains("const initialId = \"primary-action\";"));
    }

    #[test]
    fn activation_without_target_uses_default_path() {
        let script = activation_script("scope", None);

        assert!(script.contains("const initialId = null;"));
    }

    #[test]
    fn scripts_do_not_own_escape_policy() {
        assert!(!ACTIVATE_JS.contains("Escape"));
        assert!(!DEACTIVATE_JS.contains("Escape"));
    }
}
