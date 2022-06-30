#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
use mycelium::BaseApi;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen]
    fn alert(s: &str);
}

#[wasm_bindgen_test]
async fn wasm_test1() {
    log("hello!");
    BaseApi::new("http://localhost:9933")
        .fetch_metadata()
        .await
        .expect("must not error");
    alert("done!..");
}

#[wasm_bindgen_test]
async fn wasm_test2() {
    BaseApi::new("http://localhost:9933")
        .fetch_rpc_methods()
        .await
        .expect("must not error");
}
