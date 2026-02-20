use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@forma/ui-lib")]
extern "C" {
    #[wasm_bindgen(js_name = "UIBuilder")]
    pub(crate) type JsUIBuilder;

    #[wasm_bindgen(js_name = "col")]
    pub(crate) fn new_col() -> JsUIBuilder;

    #[wasm_bindgen(js_name = "col")]
    pub(crate) fn new_col_with_gap(gap_px: i32) -> JsUIBuilder;

    #[wasm_bindgen(js_name = "row")]
    pub(crate) fn new_row() -> JsUIBuilder;

    #[wasm_bindgen(js_name = "row")]
    pub(crate) fn new_row_with_gap(gap_px: i32) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn col(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "col")]
    pub(crate) fn col_with_gap(this: &JsUIBuilder, gap_px: i32) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "endCol")]
    pub(crate) fn end_col(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn row(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "row")]
    pub(crate) fn row_with_gap(this: &JsUIBuilder, gap_px: i32) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "endRow")]
    pub(crate) fn end_row(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn button(
        this: &JsUIBuilder,
        label: &str,
        disabled: bool,
        variant: &str,
        on_click: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn input(
        this: &JsUIBuilder,
        input_type: &str,
        placeholder: &str,
        value: &str,
        disabled: bool,
        on_change: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn p(this: &JsUIBuilder, text: &str, level: &str) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn alert(
        this: &JsUIBuilder,
        text: &str,
        alert_type: &str,
        title: &str,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn img(this: &JsUIBuilder, src: &str, alt: &str) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn checkbox(
        this: &JsUIBuilder,
        label: &str,
        checked: bool,
        disabled: bool,
        on_change: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn select(
        this: &JsUIBuilder,
        options: &JsValue,
        value: &str,
        placeholder: &str,
        disabled: bool,
        on_change: Option<&Function>,
    ) -> JsUIBuilder;

    #[wasm_bindgen(method)]
    pub(crate) fn separator(this: &JsUIBuilder) -> JsUIBuilder;

    #[wasm_bindgen(method, js_name = "renderInto")]
    pub(crate) fn render_into(this: &JsUIBuilder, selector: &str);
}
