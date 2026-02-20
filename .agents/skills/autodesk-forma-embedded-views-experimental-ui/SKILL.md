---
name: autodesk-forma-embedded-views-experimental-ui
description: Generate Rust WASM code that renders browser UIs using the forma-ui-lib-wasm crate. Use when the user asks to build a UI, create a form, render components, or write Rust code targeting forma-ui-lib-wasm, UiBuilder, or the Forma embedded view SDK UI bindings.
---

# Generate Rust UI code using `forma-ui-lib-wasm`

Generate Rust code that renders UIs in a web browser via WebAssembly. The code uses the `forma_ui_lib_wasm` crate — a fluent builder API that delegates all DOM work to an internal JS library. The public API is pure Rust: all input/output types are concrete Rust enums and structs, callbacks are native Rust closures, and no WASM or DOM types (`js_sys`, `web_sys`, `wasm_bindgen`, `JsValue`, `Function`, `Closure`, `JsCast`) are exposed to consumers.

## Dependencies

### Rust crate

The `forma-ui-lib-wasm` crate is **not published on crates.io**. A local Cargo patch is expected to be present that resolves it automatically from git. If that patch is missing or doesn't work, add the dependency directly from the git repository:

```toml
[dependencies]
forma-ui-lib-wasm = { git = "https://github.com/evgenii-dobrovidov-adsk/forma-embedded-view-sdk-bindings-ui" }
```

This is the **only** dependency needed. Do NOT add `wasm-bindgen`, `js-sys`, or `web-sys` — the crate handles all WASM/JS interop internally.

Standard imports:

```rust
use forma_ui_lib_wasm::{
    AlertType, ButtonVariant, InputType, SelectOption, TextLevel, UiBuilder,
};
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
<script type="module" src="glue.js"></script>
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

## Types

All variant/level/type parameters use concrete Rust enums. No raw strings.

### `TextLevel`

`TextLevel::H1`, `TextLevel::H2`, `TextLevel::H3`, `TextLevel::P`, `TextLevel::Code`

### `AlertType`

`AlertType::Error`, `AlertType::Warning`, `AlertType::Info`

### `InputType`

`InputType::Text`, `InputType::Number`, `InputType::Email`, `InputType::Password`, `InputType::Tel`, `InputType::Url`, `InputType::Search`, `InputType::Date`, `InputType::Time`, `InputType::DateTimeLocal`, `InputType::Month`, `InputType::Week`, `InputType::Color`, `InputType::Range`, `InputType::Hidden`

### `ButtonVariant`

`ButtonVariant::Outlined`, `ButtonVariant::Flat`, `ButtonVariant::Solid`

### `SelectOption`

```rust
SelectOption::new("value", "Label")
```

Or construct directly: `SelectOption { value: String, label: String }`.

## API reference

All methods consume `self` and return `UiBuilder` for fluent chaining.

### Constructors

| Constructor | Description |
|---|---|
| `UiBuilder::new_col()` | Start a tree rooted in a vertical column layout (default 8px gap) |
| `UiBuilder::new_col_with_gap(gap_px: i32)` | Start a tree rooted in a vertical column layout with a custom gap |
| `UiBuilder::new_row()` | Start a tree rooted in a horizontal row layout (default 8px gap) |
| `UiBuilder::new_row_with_gap(gap_px: i32)` | Start a tree rooted in a horizontal row layout with a custom gap |

### Layout containers

| Method | Description |
|---|---|
| `.col()` | Open a nested column (flex column, stretch cross-axis, start main-axis, default 8px gap) |
| `.col_with_gap(gap_px: i32)` | Open a nested column with a custom gap |
| `.end_col()` | Close the current column |
| `.row()` | Open a nested row (flex row, center both axes, default 8px gap) |
| `.row_with_gap(gap_px: i32)` | Open a nested row with a custom gap |
| `.end_row()` | Close the current row |

Unclosed containers are auto-closed by `.render_into()`.

### Components

#### Text

```rust
.p(text: &str, level: TextLevel) -> UiBuilder
```

`TextLevel::Code` renders inside a `<pre><code>` block that preserves newlines and whitespace.

#### Button

```rust
.button(label: &str, disabled: bool, variant: ButtonVariant, on_click: Option<Box<dyn FnMut() + 'static>>) -> UiBuilder
```

#### Input

```rust
.input(input_type: InputType, placeholder: &str, value: &str, disabled: bool, on_change: Option<Box<dyn FnMut(String) + 'static>>) -> UiBuilder
```

Callback receives: `String` (new value).

#### Checkbox

```rust
.checkbox(label: &str, checked: bool, disabled: bool, on_change: Option<Box<dyn FnMut(bool) + 'static>>) -> UiBuilder
```

Callback receives: `bool` (new checked state).

#### Select / Dropdown

```rust
.select(options: &[SelectOption], value: &str, placeholder: &str, disabled: bool, on_change: Option<Box<dyn FnMut(String) + 'static>>) -> UiBuilder
```

Callback receives: `String` (selected value).

#### Alert / Banner

```rust
.alert(text: &str, alert_type: AlertType, title: &str) -> UiBuilder
```

Pass `""` for `title` to omit.

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

Callbacks are native Rust closures boxed in `Option<Box<dyn FnMut(...) + 'static>>`. The crate converts them to JS functions internally — consumer code never touches `Closure`, `Function`, or `JsCast`.

Provide a callback: `Some(Box::new(|v: String| { /* ... */ }))`
Omit a callback: `None`

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

## Entry point and JS glue

The entry point is a raw WASM export — no `#[wasm_bindgen]` attribute needed:

```rust
#[no_mangle]
pub extern "C" fn start() {
    render();
}
```

A small JS glue file loads the WASM module and calls the export:

```js
import init from './pkg/my_app.js';

const wasm = await init();
wasm.start();
```

The wasm-bindgen generated `init` handles all internal FFI bindings. The glue file simply calls the raw WASM `start` export afterwards.

## Complete example

Counter with name input:

```rust
use std::cell::{Cell, RefCell};
use forma_ui_lib_wasm::{
    AlertType, ButtonVariant, InputType, TextLevel, UiBuilder,
};

thread_local! {
    static COUNT: Cell<u32> = Cell::new(0);
    static NAME: RefCell<String> = RefCell::new(String::new());
}

#[no_mangle]
pub extern "C" fn start() {
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
        .p(&greeting, TextLevel::H1)
        .separator()
        .row()
            .input(InputType::Text, "Your name...", &name, false, Some(Box::new(|v: String| {
                NAME.with(|n| *n.borrow_mut() = v);
                render();
            })))
        .end_row()
        .row()
            .button("+1", false, ButtonVariant::Solid, Some(Box::new(|| {
                COUNT.with(|c| c.set(c.get() + 1));
                render();
            })))
            .button("Reset", false, ButtonVariant::Outlined, Some(Box::new(|| {
                COUNT.with(|c| c.set(0));
                render();
            })))
        .end_row()
        .separator()
        .alert(&format!("Current count is {}.", count), AlertType::Info, "")
    .render_into("#app");
}
```

## Rules

1. NEVER import `web_sys`, `js_sys`, `wasm_bindgen`, or use DOM/WASM types directly. The only dependency is `forma-ui-lib-wasm`.
2. ALWAYS use `UiBuilder` for all UI construction.
3. ALWAYS call `.render_into(selector)` as the final chain step.
4. ALWAYS use `Some(Box::new(|| { ... }))` for callbacks and `None` to omit.
5. ALWAYS use `thread_local!` for mutable state, never `static mut`.
6. ALWAYS snapshot state into locals before building the UI tree.
7. ALWAYS use typed enums (`TextLevel`, `InputType`, `ButtonVariant`, `AlertType`) — never raw strings.
8. For `select()`, pass a `&[SelectOption]` slice.
9. For optional strings (`title` in `alert`, `alt` in `img`), pass `""` to omit.
10. Use `#[no_mangle] pub extern "C" fn start()` for the entry point, not `#[wasm_bindgen(start)]`.
