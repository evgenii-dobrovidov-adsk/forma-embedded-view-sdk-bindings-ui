---
name: forma-ui-wasm-codegen
description: Generate Rust WASM code that renders browser UIs using the forma-ui-lib-wasm crate. Use when the user asks to build a UI, create a form, render components, or write Rust code targeting forma-ui-lib-wasm, UiBuilder, or the Forma embedded view SDK UI bindings.
---

# Generate Rust UI code using `forma-ui-lib-wasm`

Generate Rust code that renders UIs in a web browser via WebAssembly. The code uses the `forma_ui_lib_wasm` crate — a fluent builder API that delegates all DOM work to a JS library. Generated code MUST NOT use `web_sys` or any DOM APIs directly.

## Dependencies

```toml
[dependencies]
forma-ui-lib-wasm = { path = "..." }
wasm-bindgen = "0.2"
js-sys = "0.3"
```

Standard imports:

```rust
use forma_ui_lib_wasm::UiBuilder;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
```

## API quick reference

For the complete API with all parameters and allowed values, see [reference.md](reference.md).

### Constructors

- `UiBuilder::new_col()` — column layout root
- `UiBuilder::new_row()` — row layout root

All methods consume `self` and return `UiBuilder` for fluent chaining.

### Layout

- `.col()` / `.end_col()` — nested column (stretch cross-axis, start main-axis)
- `.row()` / `.end_row()` — nested row (center both axes, 8px gap)

Unclosed containers are auto-closed by `.render_into()`.

### Components

- `.p(text, level)` — text: `"h1"` `"h2"` `"h3"` `"p"` `"code"`
- `.button(label, disabled, variant, on_click)` — variant: `"outlined"` `"flat"` `"solid"`
- `.input(type, placeholder, value, disabled, on_change)` — type: `"text"` `"number"` `"email"` `"color"` etc.
- `.checkbox(label, checked, disabled, on_change)`
- `.select(options_json, value, placeholder, disabled, on_change)` — options as JSON string
- `.alert(text, alert_type, title)` — type: `"error"` `"warning"` `"info"`, `""` to omit title
- `.img(src, alt)` — `""` to omit alt
- `.separator()`

### Rendering

`.render_into(selector)` — consumes builder, replaces target element contents.

## Callback pattern

```rust
fn cb(f: impl FnMut() + 'static) -> Function {
    Closure::<dyn FnMut()>::new(f).into_js_value().unchecked_into()
}

fn cb_str(f: impl FnMut(String) + 'static) -> Function {
    Closure::<dyn FnMut(String)>::new(f).into_js_value().unchecked_into()
}

fn cb_bool(f: impl FnMut(bool) + 'static) -> Function {
    Closure::<dyn FnMut(bool)>::new(f).into_js_value().unchecked_into()
}
```

Use `Some(cb(...))` to provide, `None` to omit.

## State management

```rust
use std::cell::{Cell, RefCell};

thread_local! {
    static COUNT: Cell<u32> = Cell::new(0);
    static NAME: RefCell<String> = RefCell::new(String::new());
}
```

Read: `COUNT.with(|c| c.get())`, `NAME.with(|n| n.borrow().clone())`
Write: `COUNT.with(|c| c.set(val))`, `NAME.with(|n| *n.borrow_mut() = val)`

## Re-rendering

Snapshot state into locals at the top of `render()`, build the UI, call `render()` again from callbacks that mutate state.

## Entry point

```rust
#[wasm_bindgen(start)]
pub fn start() {
    render();
}
```

## Host function imports

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "myGlobalFn")]
    fn my_global_fn(arg: &str);
}
```

## Rules

1. NEVER import `web_sys` or use DOM APIs directly.
2. ALWAYS use `UiBuilder` for all UI construction.
3. ALWAYS call `.render_into(selector)` as the final chain step.
4. ALWAYS use `Closure::into_js_value().unchecked_into()` for callbacks.
5. ALWAYS use `thread_local!` for mutable state, never `static mut`.
6. ALWAYS snapshot state into locals before building the UI tree.
7. For `select()`, pass options as a JSON string.
8. For optional strings (`title` in `alert`, `alt` in `img`), pass `""` to omit.
