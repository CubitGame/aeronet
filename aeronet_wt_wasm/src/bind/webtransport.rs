use wasm_bindgen::prelude::*;
use web_sys::ReadableStream;

use super::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransport , typescript_type = "WebTransport")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransport;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = ready)]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ready(this: &WebTransport) -> ::js_sys::Promise;
    #[cfg(feature = "WebTransportReliabilityMode")]
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = reliability)]
    #[doc = "Getter for the `reliability` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/reliability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportReliabilityMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reliability(this: &WebTransport) -> WebTransportReliabilityMode;
    #[cfg(feature = "WebTransportCongestionControl")]
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = congestionControl)]
    #[doc = "Getter for the `congestionControl` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/congestionControl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportCongestionControl`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn congestion_control(this: &WebTransport) -> WebTransportCongestionControl;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn closed(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = draining)]
    #[doc = "Getter for the `draining` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/draining)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draining(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = datagrams)]
    #[doc = "Getter for the `datagrams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/datagrams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn datagrams(this: &WebTransport) -> WebTransportDatagramDuplexStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = incomingBidirectionalStreams)]
    #[doc = "Getter for the `incomingBidirectionalStreams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/incomingBidirectionalStreams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn incoming_bidirectional_streams(this: &WebTransport) -> ReadableStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = incomingUnidirectionalStreams)]
    #[doc = "Getter for the `incomingUnidirectionalStreams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/incomingUnidirectionalStreams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn incoming_unidirectional_streams(this: &WebTransport) -> ReadableStream;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransport")]
    #[doc = "The `new WebTransport(..)` constructor, creating a new instance of `WebTransport`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(url: &str) -> Result<WebTransport, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransport")]
    #[doc = "The `new WebTransport(..)` constructor, creating a new instance of `WebTransport`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_options(
        url: &str,
        options: &WebTransportOptions,
    ) -> Result<WebTransport, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &WebTransport);
    #[cfg(feature = "WebTransportCloseInfo")]
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close_with_close_info(this: &WebTransport, close_info: &WebTransportCloseInfo);
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createBidirectionalStream)]
    #[doc = "The `createBidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createBidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_bidirectional_stream(this: &WebTransport) -> ::js_sys::Promise;
    #[cfg(feature = "WebTransportSendStreamOptions")]
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createBidirectionalStream)]
    #[doc = "The `createBidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createBidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportSendStreamOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_bidirectional_stream_with_options(
        this: &WebTransport,
        options: &WebTransportSendStreamOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createUnidirectionalStream)]
    #[doc = "The `createUnidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createUnidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_unidirectional_stream(this: &WebTransport) -> ::js_sys::Promise;
    #[cfg(feature = "WebTransportSendStreamOptions")]
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createUnidirectionalStream)]
    #[doc = "The `createUnidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createUnidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`, `WebTransportSendStreamOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_unidirectional_stream_with_options(
        this: &WebTransport,
        options: &WebTransportSendStreamOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_stats(this: &WebTransport) -> ::js_sys::Promise;
}
