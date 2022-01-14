use crate::app::{FirebaseApp, FirebaseOptions};
use wasm_bindgen::prelude::*;

pub mod app;

#[wasm_bindgen(module = "/dist/index.js")]
extern "C" {

    #[wasm_bindgen(js_name = "initializeApp", catch)]
    pub fn initialize_app(
        options: FirebaseOptions,
        name: Option<String>,
    ) -> Result<FirebaseApp, JsValue>;

}
