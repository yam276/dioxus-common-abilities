use dioxus::prelude::*;
use dioxus_focus_scope::{use_focus_scope, use_focus_scope_with_options, FocusScopeOptions};

const STYLE: &str = r#"
body {
    margin: 0;
    min-height: 100vh;
    background: #111827;
    color: #f9fafb;
    font-family: system-ui, sans-serif;
}

button {
    border: 1px solid #64748b;
    border-radius: 0.35rem;
    background: #1e293b;
    color: inherit;
    padding: 0.55rem 0.8rem;
}

button:focus-visible,
.dialog-panel:focus-visible {
    outline: 3px solid #38bdf8;
    outline-offset: 2px;
}

.fixture {
    display: grid;
    gap: 1rem;
    max-width: 52rem;
    margin: 0 auto;
    padding: 2rem;
}

.scenario {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 1rem;
}

.dialog-root {
    position: fixed;
    inset: 0;
    z-index: 10;
    display: grid;
    place-items: center;
    background: rgb(0 0 0 / 65%);
}

.dialog-root.nested {
    z-index: 20;
    background: rgb(15 23 42 / 75%);
}

.dialog-panel {
    display: grid;
    gap: 0.75rem;
    width: min(30rem, calc(100vw - 3rem));
    border: 1px solid #94a3b8;
    border-radius: 0.5rem;
    background: #0f172a;
    padding: 1.25rem;
}

.actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
}
"#;

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut basic_open = use_signal(|| false);
    let mut outer_open = use_signal(|| false);
    let mut empty_open = use_signal(|| false);
    let mut dynamic_open = use_signal(|| false);

    rsx! {
        document::Title { "DCA-023 focus lifecycle validation" }
        style { dangerous_inner_html: STYLE }
        main { class: "fixture",
            h1 { "DCA-023 focus lifecycle validation" }
            p { "Shared focus scope with product-owned dialog semantics and close policy." }

            Scenario {
                title: "Basic dialog",
                detail: "Initial focus, Tab wrap, Escape and opener restoration.",
                opener_id: "open-basic",
                on_open: move |_| basic_open.set(true),
            }
            Scenario {
                title: "Nested dialogs",
                detail: "Only the inner scope may react while it is open.",
                opener_id: "open-outer",
                on_open: move |_| outer_open.set(true),
            }
            Scenario {
                title: "No tabbable descendants",
                detail: "Focus must still enter the modal scope.",
                opener_id: "open-empty",
                on_open: move |_| empty_open.set(true),
            }
            Scenario {
                title: "Dynamic controls",
                detail: "Each Tab event must use the currently rendered controls.",
                opener_id: "open-dynamic",
                on_open: move |_| dynamic_open.set(true),
            }
            OrphanScenario {}

            if basic_open() {
                BasicDialog { on_close: move |_| basic_open.set(false) }
            }
            if outer_open() {
                OuterDialog { on_close: move |_| outer_open.set(false) }
            }
            if empty_open() {
                EmptyDialog { on_close: move |_| empty_open.set(false) }
            }
            if dynamic_open() {
                DynamicDialog { on_close: move |_| dynamic_open.set(false) }
            }
        }
    }
}

#[component]
fn Scenario(
    title: &'static str,
    detail: &'static str,
    opener_id: &'static str,
    on_open: EventHandler<()>,
) -> Element {
    rsx! {
        section { class: "scenario",
            div {
                h2 { "{title}" }
                p { "{detail}" }
            }
            button { id: opener_id, onclick: move |_| on_open.call(()), "Open {title}" }
        }
    }
}

#[component]
fn BasicDialog(on_close: EventHandler<()>) -> Element {
    let scope = use_focus_scope();
    rsx! {
        DialogPanel {
            scope,
            class: "dialog-root",
            title_id: "basic-title",
            description_id: "basic-description",
            on_close,
            h2 { id: "basic-title", "Basic dialog" }
            p { id: "basic-description", "The first action should receive focus." }
            div { class: "actions",
                button { id: "basic-first", "First action" }
                button { id: "basic-last", "Last action" }
                button { id: "close-basic", onclick: move |_| on_close.call(()), "Close basic dialog" }
            }
        }
    }
}

#[component]
fn OuterDialog(on_close: EventHandler<()>) -> Element {
    let mut inner_open = use_signal(|| false);
    let scope = use_focus_scope_with_options(
        FocusScopeOptions::default().with_initial_focus_id("open-inner"),
    );
    rsx! {
        DialogPanel {
            scope,
            class: "dialog-root",
            title_id: "outer-title",
            description_id: "outer-description",
            on_close,
            h2 { id: "outer-title", "Outer dialog" }
            p { id: "outer-description", "Opening the child suspends this focus scope." }
            div { class: "actions",
                button { id: "open-inner", onclick: move |_| inner_open.set(true), "Open inner dialog" }
                button { id: "outer-secondary", "Outer secondary action" }
                button { id: "close-outer", onclick: move |_| on_close.call(()), "Close outer dialog" }
            }
            if inner_open() {
                InnerDialog { on_close: move |_| inner_open.set(false) }
            }
        }
    }
}

#[component]
fn InnerDialog(on_close: EventHandler<()>) -> Element {
    let scope = use_focus_scope_with_options(
        FocusScopeOptions::default().with_initial_focus_id("inner-first"),
    );
    rsx! {
        DialogPanel {
            scope,
            class: "dialog-root nested",
            title_id: "inner-title",
            description_id: "inner-description",
            on_close,
            h2 { id: "inner-title", "Inner dialog" }
            p { id: "inner-description", "Only this scope handles Tab while mounted." }
            div { class: "actions",
                button { id: "inner-first", "Inner first action" }
                button { id: "inner-last", "Inner last action" }
                button { id: "close-inner", onclick: move |_| on_close.call(()), "Close inner dialog" }
            }
        }
    }
}

#[component]
fn EmptyDialog(on_close: EventHandler<()>) -> Element {
    let scope = use_focus_scope();
    rsx! {
        DialogPanel {
            scope,
            class: "dialog-root",
            title_id: "empty-title",
            description_id: "empty-description",
            on_close,
            h2 { id: "empty-title", "Empty dialog" }
            p { id: "empty-description", "There are deliberately no tabbable descendants." }
        }
    }
}

#[component]
fn DynamicDialog(on_close: EventHandler<()>) -> Element {
    let mut show_extra = use_signal(|| false);
    let scope = use_focus_scope_with_options(
        FocusScopeOptions::default().with_initial_focus_id("dynamic-toggle"),
    );
    rsx! {
        DialogPanel {
            scope,
            class: "dialog-root",
            title_id: "dynamic-title",
            description_id: "dynamic-description",
            on_close,
            h2 { id: "dynamic-title", "Dynamic controls" }
            p { id: "dynamic-description", "The extra action enters the live tab order." }
            div { class: "actions",
                button { id: "close-dynamic", onclick: move |_| on_close.call(()), "Close dynamic dialog" }
                button {
                    id: "dynamic-toggle",
                    onclick: move |_| show_extra.set(!show_extra()),
                    if show_extra() { "Remove extra action" } else { "Add extra action" }
                }
                if show_extra() {
                    button { id: "dynamic-extra", "Dynamic extra action" }
                }
            }
        }
    }
}

#[component]
fn OrphanScenario() -> Element {
    let mut opener_visible = use_signal(|| true);
    let mut dialog_open = use_signal(|| false);
    rsx! {
        section { class: "scenario",
            div {
                h2 { "Removed opener" }
                p { "Teardown must not focus a detached opener." }
            }
            if opener_visible() {
                button {
                    id: "open-orphan",
                    onclick: move |_| dialog_open.set(true),
                    "Open Removed opener"
                }
            } else {
                button {
                    id: "reset-orphan",
                    onclick: move |_| opener_visible.set(true),
                    "Reset removed opener"
                }
            }
        }
        if dialog_open() {
            OrphanDialog {
                on_close: move |_| {
                    dialog_open.set(false);
                    opener_visible.set(false);
                }
            }
        }
    }
}

#[component]
fn OrphanDialog(on_close: EventHandler<()>) -> Element {
    let scope = use_focus_scope_with_options(
        FocusScopeOptions::default().with_initial_focus_id("orphan-disabled"),
    );
    rsx! {
        DialogPanel {
            scope,
            class: "dialog-root",
            title_id: "orphan-title",
            description_id: "orphan-description",
            on_close,
            h2 { id: "orphan-title", "Removed opener" }
            p { id: "orphan-description", "The preferred target is disabled and the opener disappears on close." }
            div { class: "actions",
                button { id: "orphan-disabled", disabled: true, "Disabled preferred target" }
                button { id: "close-orphan", onclick: move |_| on_close.call(()), "Close removed-opener dialog" }
            }
        }
    }
}

#[component]
fn DialogPanel(
    scope: dioxus_focus_scope::FocusScope,
    class: &'static str,
    title_id: &'static str,
    description_id: &'static str,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    let root_id = scope.root_id().to_string();
    let root_tab_index = scope.root_tab_index();
    let mounted_scope = scope.clone();
    rsx! {
        div { class,
            section {
                id: root_id,
                class: "dialog-panel",
                tabindex: root_tab_index,
                role: "dialog",
                aria_modal: true,
                aria_labelledby: title_id,
                aria_describedby: description_id,
                onmounted: move |_| mounted_scope.activate(),
                onkeydown: move |event| {
                    if event.key() == Key::Escape {
                        event.stop_propagation();
                        on_close.call(());
                    }
                },
                {children}
            }
        }
    }
}
