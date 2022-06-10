use js_sys::eval;
use wasm_bindgen::JsValue;

pub struct JsRuntime {}

impl JsRuntime {
    pub fn new() -> Self {
        JsRuntime {}
    }

    pub fn eval(&self, code: &str) -> Result<JsValue, JsValue> {
        unsafe { eval(code) }
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new()
    }
}
