use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};

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

button:focus-visible {
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
    let mut inner_open = use_signal(|| false);
    let mut empty_open = use_signal(|| false);

    rsx! {
        document::Title { "DCA-023 focus lifecycle validation" }
        style { dangerous_inner_html: STYLE }
        main { class: "fixture",
            h1 { "DCA-023 focus lifecycle validation" }
            p {
                "Pinned upstream dialog behavior only. This is not a shared component."
            }

            section { class: "scenario",
                div {
                    h2 { "Basic dialog" }
                    p { "Initial focus, Tab wrap, Escape and opener restoration." }
                }
                button {
                    id: "open-basic",
                    onclick: move |_| basic_open.set(true),
                    "Open basic dialog"
                }
            }

            section { class: "scenario",
                div {
                    h2 { "Nested dialogs" }
                    p { "Only the inner scope may react while it is open." }
                }
                button {
                    id: "open-outer",
                    onclick: move |_| outer_open.set(true),
                    "Open outer dialog"
                }
            }

            section { class: "scenario",
                div {
                    h2 { "No tabbable descendants" }
                    p { "Focus must still enter the modal scope." }
                }
                button {
                    id: "open-empty",
                    onclick: move |_| empty_open.set(true),
                    "Open empty dialog"
                }
            }

            DialogRoot {
                open: basic_open(),
                on_open_change: move |open| basic_open.set(open),
                class: "dialog-root",
                DialogContent { class: "dialog-panel",
                    DialogTitle { "Basic dialog" }
                    DialogDescription {
                        "The first action should receive focus and Tab should remain inside."
                    }
                    div { class: "actions",
                        button { id: "basic-first", "First action" }
                        button { id: "basic-last", "Last action" }
                        button {
                            id: "close-basic",
                            onclick: move |_| basic_open.set(false),
                            "Close basic dialog"
                        }
                    }
                }
            }

            DialogRoot {
                open: outer_open(),
                on_open_change: move |open: bool| {
                    if !open {
                        inner_open.set(false);
                    }
                    outer_open.set(open);
                },
                class: "dialog-root",
                DialogContent { class: "dialog-panel",
                    DialogTitle { "Outer dialog" }
                    DialogDescription {
                        "Opening the child must suspend the outer focus scope."
                    }
                    div { class: "actions",
                        button {
                            id: "open-inner",
                            onclick: move |_| inner_open.set(true),
                            "Open inner dialog"
                        }
                        button { id: "outer-secondary", "Outer secondary action" }
                        button {
                            id: "close-outer",
                            onclick: move |_| outer_open.set(false),
                            "Close outer dialog"
                        }
                    }

                    DialogRoot {
                        open: inner_open(),
                        on_open_change: move |open| inner_open.set(open),
                        class: "dialog-root nested",
                        DialogContent { class: "dialog-panel",
                            DialogTitle { "Inner dialog" }
                            DialogDescription {
                                "Tab and Escape must affect only this dialog."
                            }
                            div { class: "actions",
                                button { id: "inner-first", "Inner first action" }
                                button { id: "inner-last", "Inner last action" }
                                button {
                                    id: "close-inner",
                                    onclick: move |_| inner_open.set(false),
                                    "Close inner dialog"
                                }
                            }
                        }
                    }
                }
            }

            DialogRoot {
                open: empty_open(),
                on_open_change: move |open| empty_open.set(open),
                class: "dialog-root",
                DialogContent { class: "dialog-panel",
                    DialogTitle { "Empty dialog" }
                    DialogDescription {
                        "There are deliberately no tabbable descendants. Press Escape to close."
                    }
                }
            }
        }
    }
}
