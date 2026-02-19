use std::cell::{Cell, RefCell};

use forma_ui_lib_wasm::UiBuilder;
use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "logToPanel")]
    fn log_to_panel(msg: &str);
}

thread_local! {
    static NAME_VALUE: RefCell<String> = RefCell::new(String::new());
    static COLOR_VALUE: RefCell<String> = RefCell::new("#4a90d9".to_string());
    static AGREE_CHECKED: Cell<bool> = Cell::new(false);
    static SELECTED_FONT: RefCell<String> = RefCell::new("sans".to_string());
}

fn log(msg: &str) {
    log_to_panel(msg);
}

fn cb(f: impl FnMut() + 'static) -> Function {
    Closure::<dyn FnMut()>::new(f)
        .into_js_value()
        .unchecked_into()
}

fn cb_str(f: impl FnMut(String) + 'static) -> Function {
    Closure::<dyn FnMut(String)>::new(f)
        .into_js_value()
        .unchecked_into()
}

fn cb_bool(f: impl FnMut(bool) + 'static) -> Function {
    Closure::<dyn FnMut(bool)>::new(f)
        .into_js_value()
        .unchecked_into()
}

#[wasm_bindgen(start)]
pub fn start() {
    render();
}

fn render() {
    let name = NAME_VALUE.with(|n| n.borrow().clone());
    let color = COLOR_VALUE.with(|c| c.borrow().clone());
    let agree = AGREE_CHECKED.with(|a| a.get());
    let font = SELECTED_FONT.with(|f| f.borrow().clone());

    UiBuilder::new_col()
        .p("UI Library Demo (Rust WASM)", "h1")
        .p(
            "The same demo as the TypeScript version, but driven entirely from Rust compiled to WASM.",
            "p",
        )

        .separator()

        .p("Text Input", "h3")
        .row()
            .input(
                "text", "Enter your name...", &name, false,
                Some(cb_str(|v| {
                    NAME_VALUE.with(|n| *n.borrow_mut() = v.clone());
                    log(&format!("Name changed: \"{}\"", v));
                })),
            )
            .button("Greet", false, "solid", Some(cb(|| {
                let name = NAME_VALUE.with(|n| n.borrow().clone());
                if name.is_empty() {
                    log("Hello, World!");
                } else {
                    log(&format!("Hello, {}!", name));
                }
            })))
        .end_row()

        .separator()

        .p("Color Picker", "h3")
        .row()
            .p("Pick a color:", "p")
            .input(
                "color", "", &color, false,
                Some(cb_str(|v| {
                    COLOR_VALUE.with(|c| *c.borrow_mut() = v.clone());
                    log(&format!("Color changed: {}", v));
                    render();
                })),
            )
            .button("Reset Color", false, "outlined", Some(cb(|| {
                COLOR_VALUE.with(|c| *c.borrow_mut() = "#4a90d9".to_string());
                log("Color reset to default");
                render();
            })))
        .end_row()

        .separator()

        .p("Select Dropdown", "h3")
        .row()
            .p("Font family:", "p")
            .select(
                r#"[{"value":"sans","label":"Sans-serif"},{"value":"serif","label":"Serif"},{"value":"mono","label":"Monospace"}]"#,
                &font,
                "Choose a font...",
                false,
                Some(cb_str(|v| {
                    SELECTED_FONT.with(|f| *f.borrow_mut() = v.clone());
                    log(&format!("Font changed: {}", v));
                })),
            )
        .end_row()

        .separator()

        .p("Checkbox & Disabled States", "h3")
        .row()
            .checkbox(
                "I agree to the terms", agree, false,
                Some(cb_bool(|checked| {
                    AGREE_CHECKED.with(|a| a.set(checked));
                    let msg = if checked { "accepted" } else { "declined" };
                    log(&format!("Agreement: {}", msg));
                    render();
                })),
            )
        .end_row()
        .row()
            .button("Submit", !agree, "solid", Some(cb(|| {
                log("Form submitted!");
            })))
            .button("Cancel", false, "flat", Some(cb(|| {
                log("Cancelled");
            })))
        .end_row()

        .separator()

        .p("Alerts", "h3")
        .alert("Operation completed successfully.", "info", "")
        .alert("Your session will expire in 5 minutes.", "warning", "Heads up")
        .alert("Failed to save changes. Please try again.", "error", "Error")

        .separator()

        .p("Code Block", "h3")
        .p(
            r##"UiBuilder::new_col().p("Hello", "h1").button("Click", false, "solid", Some(f)).end_col().render_into("#app")"##,
            "code",
        )

        .separator()

        .p("Image", "h3")
        .img("https://placehold.co/300x100/4a90d9/ffffff?text=Rust+WASM", "Placeholder image")

        .separator()

        .p("Disabled Input", "h3")
        .input("text", "", "This input is disabled", true, None)

    .end_col()
    .render_into("#app");
}
