use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::ffi::{self, JsUIBuilder};
use crate::types::*;

fn leak_click(f: Box<dyn FnMut()>) -> Function {
    Closure::<dyn FnMut()>::new(f)
        .into_js_value()
        .unchecked_into()
}

fn leak_string_cb(f: Box<dyn FnMut(String)>) -> Function {
    Closure::<dyn FnMut(String)>::new(f)
        .into_js_value()
        .unchecked_into()
}

fn leak_bool_cb(f: Box<dyn FnMut(bool)>) -> Function {
    Closure::<dyn FnMut(bool)>::new(f)
        .into_js_value()
        .unchecked_into()
}

fn options_to_js(options: &[SelectOption]) -> JsValue {
    let arr = js_sys::Array::new();
    for opt in options {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from_str(&opt.value)).unwrap();
        js_sys::Reflect::set(&obj, &"label".into(), &JsValue::from_str(&opt.label)).unwrap();
        arr.push(&obj);
    }
    arr.into()
}

/// Fluent builder for constructing UI trees from Rust.
///
/// All DOM interaction is delegated to `@forma/ui-lib` --
/// this struct never touches browser APIs directly.
pub struct UiBuilder {
    inner: JsUIBuilder,
}

impl UiBuilder {
    /// Start a new UI tree rooted in a column layout.
    pub fn new_col() -> Self {
        Self {
            inner: ffi::new_col(),
        }
    }

    /// Start a new UI tree rooted in a row layout.
    pub fn new_row() -> Self {
        Self {
            inner: ffi::new_row(),
        }
    }

    /// Open a nested column inside the current container.
    pub fn col(self) -> Self {
        Self {
            inner: self.inner.col(),
        }
    }

    /// Close the current column container.
    pub fn end_col(self) -> Self {
        Self {
            inner: self.inner.end_col(),
        }
    }

    /// Open a nested row inside the current container.
    pub fn row(self) -> Self {
        Self {
            inner: self.inner.row(),
        }
    }

    /// Close the current row container.
    pub fn end_row(self) -> Self {
        Self {
            inner: self.inner.end_row(),
        }
    }

    /// Add a button component.
    pub fn button(
        self,
        label: &str,
        disabled: bool,
        variant: ButtonVariant,
        on_click: Option<Box<dyn FnMut() + 'static>>,
    ) -> Self {
        let js_fn = on_click.map(leak_click);
        Self {
            inner: self
                .inner
                .button(label, disabled, variant.as_str(), js_fn.as_ref()),
        }
    }

    /// Add a text input component.
    pub fn input(
        self,
        input_type: InputType,
        placeholder: &str,
        value: &str,
        disabled: bool,
        on_change: Option<Box<dyn FnMut(String) + 'static>>,
    ) -> Self {
        let js_fn = on_change.map(leak_string_cb);
        Self {
            inner: self.inner.input(
                input_type.as_str(),
                placeholder,
                value,
                disabled,
                js_fn.as_ref(),
            ),
        }
    }

    /// Add a text/heading element.
    pub fn p(self, text: &str, level: TextLevel) -> Self {
        Self {
            inner: self.inner.p(text, level.as_str()),
        }
    }

    /// Add an alert/banner component.
    ///
    /// Pass an empty string for `title` to omit it.
    pub fn alert(self, text: &str, alert_type: AlertType, title: &str) -> Self {
        Self {
            inner: self.inner.alert(text, alert_type.as_str(), title),
        }
    }

    /// Add an image element.
    ///
    /// Pass an empty string for `alt` to omit the alt text.
    pub fn img(self, src: &str, alt: &str) -> Self {
        Self {
            inner: self.inner.img(src, alt),
        }
    }

    /// Add a checkbox component.
    pub fn checkbox(
        self,
        label: &str,
        checked: bool,
        disabled: bool,
        on_change: Option<Box<dyn FnMut(bool) + 'static>>,
    ) -> Self {
        let js_fn = on_change.map(leak_bool_cb);
        Self {
            inner: self
                .inner
                .checkbox(label, checked, disabled, js_fn.as_ref()),
        }
    }

    /// Add a select/dropdown component.
    pub fn select(
        self,
        options: &[SelectOption],
        value: &str,
        placeholder: &str,
        disabled: bool,
        on_change: Option<Box<dyn FnMut(String) + 'static>>,
    ) -> Self {
        let js_opts = options_to_js(options);
        let js_fn = on_change.map(leak_string_cb);
        Self {
            inner: self
                .inner
                .select(&js_opts, value, placeholder, disabled, js_fn.as_ref()),
        }
    }

    /// Add a horizontal separator line.
    pub fn separator(self) -> Self {
        Self {
            inner: self.inner.separator(),
        }
    }

    /// Render the constructed UI tree into the DOM element matching `selector`.
    /// Consumes the builder.
    pub fn render_into(self, selector: &str) {
        self.inner.render_into(selector);
    }
}
