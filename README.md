# forma-embedded-view-sdk-bindings-ui

> **Disclaimer:** This library is in no way officially supported, endorsed, or otherwise recommended by Autodesk. It is purely experimental. Use at your own risk.

A strict, sandboxable UI library with a fluent chaining API for building interfaces using [Autodesk Forma Weave](https://app.autodeskforma.eu/design-system/v2) components. Designed to be safely exposed to user-authored code via WASM bindings.

![Example screenshot](./screenshot-example.png)

## Motivation

When embedding user-authored logic (e.g. extensions compiled to WASM), you need a UI layer that:

- Provides a **well-defined, finite set of operations** -- no arbitrary DOM access
- Can be called from **Rust/WASM** without exposing browser APIs to the sandboxed binary
- Renders real, production-quality UI components (Autodesk Forma Weave design system)

This workspace provides exactly that: a TypeScript library that builds a virtual node tree via a fluent builder API, then renders it to the DOM using Weave web components -- and Rust WASM bindings that call into that library without ever touching the DOM directly.

## Packages

### `@forma/ui-lib` (TypeScript)

The core library. Provides a `UIBuilder` class with a fluent chaining interface for declaratively constructing UI trees.

```typescript
import { col } from '@forma/ui-lib';

col()
  .p('Settings', 'h1')
  .row()
    .p('Theme:', 'p')
    .select(
      [{ value: 'light', label: 'Light' }, { value: 'dark', label: 'Dark' }],
      'light', 'Choose...', false,
      (v) => console.log('Theme:', v),
    )
  .endRow()
  .separator()
  .row()
    .button('Save', false, 'solid', () => console.log('Saved'))
    .button('Cancel', false, 'flat', () => console.log('Cancelled'))
  .endRow()
.endCol()
.renderInto('#app');
```

**Supported components:**

| Method | Renders as | Key parameters |
|--------|-----------|----------------|
| `col()` / `endCol()` | Flex column (`align-items: stretch`) | -- |
| `row()` / `endRow()` | Flex row (`align-items: center`) | -- |
| `button()` | `<weave-button>` | label, disabled, variant, onClick |
| `input()` | `<weave-input>` or native `<input>` | type, placeholder, value, disabled, onChange |
| `p()` | `<h1>`/`<h2>`/`<h3>`/`<p>`/`<code>` | text, level |
| `alert()` | `<weave-banner>` | text, type, title |
| `img()` | `<img>` | src, alt |
| `checkbox()` | `<weave-checkbox>` | label, checked, disabled, onChange |
| `select()` | `<weave-select>` | options, value, placeholder, disabled, onChange |
| `separator()` | `<hr>` | -- |

Weave CSS and component JS files are **lazy-loaded** from the Forma CDN on first use.

### `@forma/ui-lib-wasm` (Rust)

Rust bindings for `@forma/ui-lib` via `wasm-bindgen`. The WASM binary imports the TypeScript library's builder API directly -- **no DOM APIs are exposed to the sandboxed code**.

```rust
use forma_ui_lib_wasm::UiBuilder;

UiBuilder::new_col()
    .p("Settings", "h1")
    .row()
        .input("text", "Name...", "", false, Some(on_change))
        .button("Save", false, "solid", Some(on_save))
    .end_row()
    .alert("Saved successfully.", "info", "")
.end_col()
.render_into("#app");
```

Callbacks are passed as `js_sys::Function` (or `Option<Function>` for optional ones). Create them from Rust closures with `wasm_bindgen::closure::Closure`.

## Project Structure

```
packages/
  ui-lib/                    # TypeScript library
    src/
      types.ts               # NodeDesc union type, option types
      loader.ts              # Weave CSS/JS lazy loading
      builder.ts             # UIBuilder fluent API
      render.ts              # NodeDesc tree -> DOM (Weave components)
      index.ts               # Public exports + factory functions
    example/                 # Runnable browser example
  ui-lib-wasm/               # Rust WASM bindings
    src/lib.rs               # UiBuilder struct wrapping JS imports
    example/                 # Runnable Rust WASM browser example
      src/lib.rs
      index.html
```

## Prerequisites

- Node.js >= 18
- Rust toolchain with `wasm32-unknown-unknown` target
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

## Getting Started

```bash
# Install dependencies
npm install

# Build both packages
npm run build

# Run the TypeScript example
npm run example

# Run the Rust WASM example
npm run example:wasm
```

## Build Commands

| Command | Description |
|---------|-------------|
| `npm run build` | Build both packages (TS first, then WASM) |
| `npm run build:ts` | Build only the TypeScript library |
| `npm run build:wasm` | Build only the Rust WASM bindings |
| `npm run example` | Serve the TypeScript example (Vite dev server) |
| `npm run example:wasm` | Build TS lib + WASM example, then serve (Vite) |
