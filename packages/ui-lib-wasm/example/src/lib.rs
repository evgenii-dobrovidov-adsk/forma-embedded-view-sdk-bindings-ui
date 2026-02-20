use std::cell::{Cell, RefCell};

use forma_ui_lib_wasm::{
    AlertType, ButtonVariant, InputType, SelectOption, TextLevel, UiBuilder,
};

thread_local! {
    static NAME_VALUE: RefCell<String> = RefCell::new(String::new());
    static COLOR_VALUE: RefCell<String> = RefCell::new("#4a90d9".to_string());
    static AGREE_CHECKED: Cell<bool> = Cell::new(false);
    static SELECTED_FONT: RefCell<String> = RefCell::new("sans".to_string());
    static LOG_BUFFER: RefCell<String> = RefCell::new(String::new());
}

fn log(msg: &str) {
    LOG_BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();
        if !buf.is_empty() {
            buf.push('\n');
        }
        buf.push_str(msg);
    });
    render();
}

#[no_mangle]
pub extern "C" fn start() {
    render();
}

fn render() {
    let name = NAME_VALUE.with(|n| n.borrow().clone());
    let color = COLOR_VALUE.with(|c| c.borrow().clone());
    let agree = AGREE_CHECKED.with(|a| a.get());
    let font = SELECTED_FONT.with(|f| f.borrow().clone());
    let log_buf = LOG_BUFFER.with(|b| b.borrow().clone());

    UiBuilder::new_col()
        .p("UI Library Demo (Rust WASM)", TextLevel::H1)
        .p(
            "The same demo as the TypeScript version, but driven entirely from Rust compiled to WASM.",
            TextLevel::P,
        )

        .separator()

        .p("Text Input", TextLevel::H3)
        .row()
            .input(
                InputType::Text, "Enter your name...", &name, false,
                Some(Box::new(|v: String| {
                    NAME_VALUE.with(|n| *n.borrow_mut() = v.clone());
                    log(&format!("Name changed: \"{}\"", v));
                })),
            )
            .button("Greet", false, ButtonVariant::Solid, Some(Box::new(|| {
                let name = NAME_VALUE.with(|n| n.borrow().clone());
                if name.is_empty() {
                    log("Hello, World!");
                } else {
                    log(&format!("Hello, {}!", name));
                }
            })))
        .end_row()

        .separator()

        .p("Color Picker", TextLevel::H3)
        .row()
            .p("Pick a color:", TextLevel::P)
            .input(
                InputType::Color, "", &color, false,
                Some(Box::new(|v: String| {
                    COLOR_VALUE.with(|c| *c.borrow_mut() = v.clone());
                    log(&format!("Color changed: {}", v));
                })),
            )
            .button("Reset Color", false, ButtonVariant::Outlined, Some(Box::new(|| {
                COLOR_VALUE.with(|c| *c.borrow_mut() = "#4a90d9".to_string());
                log("Color reset to default");
            })))
        .end_row()

        .separator()

        .p("Select Dropdown", TextLevel::H3)
        .row()
            .p("Font family:", TextLevel::P)
            .select(
                &[
                    SelectOption::new("sans", "Sans-serif"),
                    SelectOption::new("serif", "Serif"),
                    SelectOption::new("mono", "Monospace"),
                ],
                &font,
                "Choose a font...",
                false,
                Some(Box::new(|v: String| {
                    SELECTED_FONT.with(|f| *f.borrow_mut() = v.clone());
                    log(&format!("Font changed: {}", v));
                })),
            )
        .end_row()

        .separator()

        .p("Checkbox & Disabled States", TextLevel::H3)
        .row()
            .checkbox(
                "I agree to the terms", agree, false,
                Some(Box::new(|checked: bool| {
                    AGREE_CHECKED.with(|a| a.set(checked));
                    let msg = if checked { "accepted" } else { "declined" };
                    log(&format!("Agreement: {}", msg));
                })),
            )
        .end_row()
        .row()
            .button("Submit", !agree, ButtonVariant::Solid, Some(Box::new(|| {
                log("Form submitted!");
            })))
            .button("Cancel", false, ButtonVariant::Flat, Some(Box::new(|| {
                log("Cancelled");
            })))
        .end_row()

        .separator()

        .p("Alerts", TextLevel::H3)
        .alert("Operation completed successfully.", AlertType::Info, "")
        .alert("Your session will expire in 5 minutes.", AlertType::Warning, "Heads up")
        .alert("Failed to save changes. Please try again.", AlertType::Error, "Error")

        .separator()

        .p("Image", TextLevel::H3)
        .img("https://placehold.co/300x100/4a90d9/ffffff?text=Rust+WASM", "Placeholder image")

        .separator()

        .p("Disabled Input", TextLevel::H3)
        .input(InputType::Text, "", "This input is disabled", true, None)

        .separator()

        .p("Event Log", TextLevel::H3)
        .p(if log_buf.is_empty() { "Event log will appear here..." } else { &log_buf }, TextLevel::Code)

    .render_into("#app");
}
