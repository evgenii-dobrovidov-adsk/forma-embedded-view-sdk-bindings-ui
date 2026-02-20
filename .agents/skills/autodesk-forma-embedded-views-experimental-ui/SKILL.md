---
name: autodesk-forma-embedded-views-experimental-ui
description: Generate Rust WASM code that renders browser UIs using the forma-ui-lib-wasm crate. Use when the user asks to build a UI, create a form, render components, or write Rust code targeting forma-ui-lib-wasm, UiBuilder, or the Forma embedded view SDK UI bindings.
---

# Generate Rust UI code using `forma-ui-lib-wasm`

Generate Rust code that renders UIs in a web browser via WebAssembly. The code uses the `forma_ui_lib_wasm` crate — a fluent builder API that delegates all DOM work to a JS library. Generated code MUST NOT use `web_sys` or any DOM APIs directly.

## Dependencies

### Rust crate

The `forma-ui-lib-wasm` crate is **not published on crates.io**. A local Cargo patch is expected to be present that resolves it automatically from git. If that patch is missing or doesn't work, add the dependency directly from the git repository:

```toml
[dependencies]
forma-ui-lib-wasm = { git = "https://github.com/evgenii-dobrovidov-adsk/forma-embedded-view-sdk-bindings-ui" }
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

### JS package (`@forma/ui-lib`)

The Rust crate's generated wasm-bindgen JS glue contains `import ... from "@forma/ui-lib"`. The browser must be able to resolve this bare module specifier at runtime. The `@forma/ui-lib` package is **not published to NPM**, so it must be provided locally. There are two approaches:

#### Option A: Import map in `index.html` (no bundler needed)

Build `@forma/ui-lib` first (`npm run build` in its directory), then point to its ES module output using a `<script type="importmap">` **before** any `<script type="module">` tags:

```html
<script type="importmap">
  {
    "imports": {
      "@forma/ui-lib": "./path/to/packages/ui-lib/dist/index.js"
    }
  }
</script>
<script type="module">
  import init from './pkg/my_app.js';
  await init();
</script>
```

The path in the import map is relative to the HTML file and must point to the built `dist/index.js` (ES module output) of `@forma/ui-lib`.

#### Option B: Local `file:` dependency + bundler (Vite, etc.)

Add `@forma/ui-lib` as a local dependency in the app's `package.json`:

```json
{
  "dependencies": {
    "@forma/ui-lib": "file:../path/to/packages/ui-lib"
  }
}
```

Or, if both packages live in the same monorepo, use npm/pnpm/yarn workspaces so that `@forma/ui-lib` is resolved automatically. Then use a bundler like Vite to serve the app — the bundler resolves the bare specifier during development and build.

This is how the existing example in `packages/ui-lib-wasm/example/` works: the root `package.json` declares both packages as workspaces, and `vite serve` resolves `@forma/ui-lib` via the workspace link.

## API reference

All methods consume `self` and return `UiBuilder` for fluent chaining.

### Constructors

| Constructor | Description |
|---|---|
| `UiBuilder::new_col()` | Start a tree rooted in a vertical column layout |
| `UiBuilder::new_row()` | Start a tree rooted in a horizontal row layout |

### Layout containers

| Method | Description |
|---|---|
| `.col()` | Open a nested column (flex column, stretch cross-axis, start main-axis) |
| `.end_col()` | Close the current column |
| `.row()` | Open a nested row (flex row, center both axes, 8px gap) |
| `.end_row()` | Close the current row |

Unclosed containers are auto-closed by `.render_into()`.

### Components

#### Text

```rust
.p(text: &str, level: &str) -> UiBuilder
```

`level`: `"h1"`, `"h2"`, `"h3"`, `"p"`, `"code"`

#### Button

```rust
.button(label: &str, disabled: bool, variant: &str, on_click: Option<Function>) -> UiBuilder
```

`variant`: `"outlined"`, `"flat"`, `"solid"`

#### Input

```rust
.input(input_type: &str, placeholder: &str, value: &str, disabled: bool, on_change: Option<Function>) -> UiBuilder
```

`input_type`: `"text"`, `"number"`, `"email"`, `"password"`, `"tel"`, `"url"`, `"search"`, `"date"`, `"time"`, `"datetime-local"`, `"month"`, `"week"`, `"color"`, `"range"`, `"hidden"`

Callback receives: `String` (new value).

#### Checkbox

```rust
.checkbox(label: &str, checked: bool, disabled: bool, on_change: Option<Function>) -> UiBuilder
```

Callback receives: `bool` (new checked state).

#### Select / Dropdown

```rust
.select(options_json: &str, value: &str, placeholder: &str, disabled: bool, on_change: Option<Function>) -> UiBuilder
```

`options_json` must be a JSON array of `{"value":"...","label":"..."}` objects:

```rust
r#"[{"value":"a","label":"Option A"},{"value":"b","label":"Option B"}]"#
```

Callback receives: `String` (selected value).

#### Alert / Banner

```rust
.alert(text: &str, alert_type: &str, title: &str) -> UiBuilder
```

`alert_type`: `"error"`, `"warning"`, `"info"`. Pass `""` for `title` to omit.

#### Image

```rust
.img(src: &str, alt: &str) -> UiBuilder
```

Pass `""` for `alt` to omit.

#### Separator

```rust
.separator() -> UiBuilder
```

### Rendering

```rust
.render_into(selector: &str)
```

Consumes the builder. Replaces target element contents. Auto-closes unclosed containers.

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

`Closure::into_js_value()` consumes the closure and leaks memory so the JS function stays valid. Do NOT use `Closure::forget()`.

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

## Rules

1. NEVER import `web_sys` or use DOM APIs directly.
2. ALWAYS use `UiBuilder` for all UI construction.
3. ALWAYS call `.render_into(selector)` as the final chain step.
4. ALWAYS use `Closure::into_js_value().unchecked_into()` for callbacks.
5. ALWAYS use `thread_local!` for mutable state, never `static mut`.
6. ALWAYS snapshot state into locals before building the UI tree.
7. For `select()`, pass options as a JSON string.
8. For optional strings (`title` in `alert`, `alt` in `img`), pass `""` to omit.
