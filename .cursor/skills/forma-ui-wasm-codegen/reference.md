# API Reference: `forma-ui-lib-wasm`

## Constructor methods

| Constructor | Description |
|---|---|
| `UiBuilder::new_col()` | Start a tree rooted in a vertical column layout |
| `UiBuilder::new_row()` | Start a tree rooted in a horizontal row layout |

## Layout containers

| Method | Description |
|---|---|
| `.col()` | Open a nested column (flex column, stretch cross-axis, start main-axis) |
| `.end_col()` | Close the current column |
| `.row()` | Open a nested row (flex row, center both axes, 8px gap) |
| `.end_row()` | Close the current row |

Unclosed containers are auto-closed by `.render_into()`.

## Component signatures

### Text

```rust
.p(text: &str, level: &str) -> UiBuilder
```

`level`: `"h1"`, `"h2"`, `"h3"`, `"p"`, `"code"`

### Button

```rust
.button(label: &str, disabled: bool, variant: &str, on_click: Option<Function>) -> UiBuilder
```

`variant`: `"outlined"`, `"flat"`, `"solid"`

### Input

```rust
.input(input_type: &str, placeholder: &str, value: &str, disabled: bool, on_change: Option<Function>) -> UiBuilder
```

`input_type`: `"text"`, `"number"`, `"email"`, `"password"`, `"tel"`, `"url"`, `"search"`, `"date"`, `"time"`, `"datetime-local"`, `"month"`, `"week"`, `"color"`, `"range"`, `"hidden"`

Callback receives: `String` (new value).

### Checkbox

```rust
.checkbox(label: &str, checked: bool, disabled: bool, on_change: Option<Function>) -> UiBuilder
```

Callback receives: `bool` (new checked state).

### Select / Dropdown

```rust
.select(options_json: &str, value: &str, placeholder: &str, disabled: bool, on_change: Option<Function>) -> UiBuilder
```

`options_json` must be a JSON array of `{"value":"...","label":"..."}` objects:

```rust
r#"[{"value":"a","label":"Option A"},{"value":"b","label":"Option B"}]"#
```

Callback receives: `String` (selected value).

### Alert / Banner

```rust
.alert(text: &str, alert_type: &str, title: &str) -> UiBuilder
```

`alert_type`: `"error"`, `"warning"`, `"info"`. Pass `""` for `title` to omit.

### Image

```rust
.img(src: &str, alt: &str) -> UiBuilder
```

Pass `""` for `alt` to omit.

### Separator

```rust
.separator() -> UiBuilder
```

### Render

```rust
.render_into(selector: &str)
```

Consumes the builder. Replaces target element contents. Auto-closes unclosed containers.

## Callback helpers

```rust
// No-argument callback (button click)
fn cb(f: impl FnMut() + 'static) -> Function {
    Closure::<dyn FnMut()>::new(f).into_js_value().unchecked_into()
}

// String-argument callback (input/select change)
fn cb_str(f: impl FnMut(String) + 'static) -> Function {
    Closure::<dyn FnMut(String)>::new(f).into_js_value().unchecked_into()
}

// Bool-argument callback (checkbox change)
fn cb_bool(f: impl FnMut(bool) + 'static) -> Function {
    Closure::<dyn FnMut(bool)>::new(f).into_js_value().unchecked_into()
}
```

`Closure::into_js_value()` consumes the closure and leaks memory so the JS function stays valid. Do NOT use `Closure::forget()`.

## Complete example

Counter with name input:

```rust
use std::cell::{Cell, RefCell};
use forma_ui_lib_wasm::UiBuilder;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

thread_local! {
    static COUNT: Cell<u32> = Cell::new(0);
    static NAME: RefCell<String> = RefCell::new(String::new());
}

fn cb(f: impl FnMut() + 'static) -> Function {
    Closure::<dyn FnMut()>::new(f).into_js_value().unchecked_into()
}

fn cb_str(f: impl FnMut(String) + 'static) -> Function {
    Closure::<dyn FnMut(String)>::new(f).into_js_value().unchecked_into()
}

#[wasm_bindgen(start)]
pub fn start() {
    render();
}

fn render() {
    let count = COUNT.with(|c| c.get());
    let name = NAME.with(|n| n.borrow().clone());
    let greeting = if name.is_empty() {
        format!("Count: {}", count)
    } else {
        format!("Hello {}, count: {}", name, count)
    };

    UiBuilder::new_col()
        .p(&greeting, "h1")
        .separator()
        .row()
            .input("text", "Your name...", &name, false, Some(cb_str(|v| {
                NAME.with(|n| *n.borrow_mut() = v);
                render();
            })))
        .end_row()
        .row()
            .button("+1", false, "solid", Some(cb(|| {
                COUNT.with(|c| c.set(c.get() + 1));
                render();
            })))
            .button("Reset", false, "outlined", Some(cb(|| {
                COUNT.with(|c| c.set(0));
                render();
            })))
        .end_row()
        .separator()
        .alert(&format!("Current count is {}.", count), "info", "")
    .render_into("#app");
}
```
