use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportHash)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportHash` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportHash`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportHash;
}

impl WebTransportHash {
    #[doc = "Construct a new `WebTransportHash`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportHash`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `algorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportHash`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn algorithm(&mut self, val: &str) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("algorithm"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportHash`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn value(&mut self, val: &::js_sys::Object) -> &mut Self {
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}

impl Default for WebTransportHash {
    fn default() -> Self {
        Self::new()
    }
}
