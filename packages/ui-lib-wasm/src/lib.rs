use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@forma/ui-lib")]
extern "C" {
    #[wasm_bindgen(js_name = "UIBuilder")]
    type JsUIBuilder;

    fn col() -> JsUIBuilder;
    fn row() -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn col(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "endCol")]
    fn end_col(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn row(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "endRow")]
    fn end_row(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn button(
        this: &JsUIBuilder,
        label: &str,
        disabled: bool,
        variant: &str,
        on_click: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn input(
        this: &JsUIBuilder,
        input_type: &str,
        placeholder: &str,
        value: &str,
        disabled: bool,
        on_change: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn p(this: &JsUIBuilder, text: &str, level: &str) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn alert(this: &JsUIBuilder, text: &str, alert_type: &str, title: &str) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn img(this: &JsUIBuilder, src: &str, alt: &str) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn checkbox(
        this: &JsUIBuilder,
        label: &str,
        checked: bool,
        disabled: bool,
        on_change: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn select(
        this: &JsUIBuilder,
        options: &JsValue,
        value: &str,
        placeholder: &str,
        disabled: bool,
        on_change: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    fn separator(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "renderInto")]
    fn render_into(this: &JsUIBuilder, selector: &str);
}

/// Fluent builder for constructing UI trees from WASM.
///
/// All DOM interaction is delegated to `@forma/ui-lib` --
/// this struct never touches browser APIs directly.
#[wasm_bindgen]
pub struct UiBuilder {
    inner: JsUIBuilder,
}

#[wasm_bindgen]
impl UiBuilder {
    /// Start a new UI tree rooted in a column layout.
    #[wasm_bindgen(constructor)]
    pub fn new_col() -> UiBuilder {
        UiBuilder { inner: col() }
    }

    /// Start a new UI tree rooted in a row layout.
    pub fn new_row() -> UiBuilder {
        UiBuilder { inner: row() }
    }

    /// Open a nested column inside the current container.
    pub fn col(self) -> UiBuilder {
        UiBuilder { inner: self.inner.col() }
    }

    /// Close the current column container.
    pub fn end_col(self) -> UiBuilder {
        UiBuilder { inner: self.inner.end_col() }
    }

    /// Open a nested row inside the current container.
    pub fn row(self) -> UiBuilder {
        UiBuilder { inner: self.inner.row() }
    }

    /// Close the current row container.
    pub fn end_row(self) -> UiBuilder {
        UiBuilder { inner: self.inner.end_row() }
    }

    /// Add a button component.
    ///
    /// `variant` must be one of: `"outlined"`, `"flat"`, `"solid"`.
    pub fn button(
        self,
        label: &str,
        disabled: bool,
        variant: &str,
        on_click: Option<Function>,
    ) -> UiBuilder {
        UiBuilder { inner: self.inner.button(label, disabled, variant, on_click.as_ref()) }
    }

    /// Add a text input component.
    ///
    /// `input_type` must be a valid HTML input type (e.g. `"text"`, `"number"`, `"email"`, `"color"`).
    pub fn input(
        self,
        input_type: &str,
        placeholder: &str,
        value: &str,
        disabled: bool,
        on_change: Option<Function>,
    ) -> UiBuilder {
        UiBuilder {
            inner: self.inner.input(input_type, placeholder, value, disabled, on_change.as_ref()),
        }
    }

    /// Add a text/heading element.
    ///
    /// `level` must be one of: `"h1"`, `"h2"`, `"h3"`, `"p"`, `"code"`.
    pub fn p(self, text: &str, level: &str) -> UiBuilder {
        UiBuilder { inner: self.inner.p(text, level) }
    }

    /// Add an alert/banner component.
    ///
    /// `alert_type` must be one of: `"error"`, `"warning"`, `"info"`.
    /// Pass an empty string for `title` to omit it.
    pub fn alert(self, text: &str, alert_type: &str, title: &str) -> UiBuilder {
        UiBuilder { inner: self.inner.alert(text, alert_type, title) }
    }

    /// Add an image element.
    ///
    /// Pass an empty string for `alt` to omit the alt text.
    pub fn img(self, src: &str, alt: &str) -> UiBuilder {
        UiBuilder { inner: self.inner.img(src, alt) }
    }

    /// Add a checkbox component.
    pub fn checkbox(
        self,
        label: &str,
        checked: bool,
        disabled: bool,
        on_change: Option<Function>,
    ) -> UiBuilder {
        UiBuilder { inner: self.inner.checkbox(label, checked, disabled, on_change.as_ref()) }
    }

    /// Add a select/dropdown component.
    ///
    /// `options_json` must be a JSON array of `{"value":"...","label":"..."}` objects.
    pub fn select(
        self,
        options_json: &str,
        value: &str,
        placeholder: &str,
        disabled: bool,
        on_change: Option<Function>,
    ) -> UiBuilder {
        let options = js_sys::JSON::parse(options_json)
            .expect("select: invalid JSON for options");
        UiBuilder {
            inner: self.inner.select(&options, value, placeholder, disabled, on_change.as_ref()),
        }
    }

    /// Add a horizontal separator line.
    pub fn separator(self) -> UiBuilder {
        UiBuilder { inner: self.inner.separator() }
    }

    /// Render the constructed UI tree into the DOM element matching `selector`.
    /// Consumes the builder.
    pub fn render_into(self, selector: &str) {
        self.inner.render_into(selector);
    }
}
