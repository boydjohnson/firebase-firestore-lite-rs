use crate::app::{FirebaseApp, FirebaseOptions};
use store::{Firestore, FirestoreSettings};
use wasm_bindgen::prelude::*;

pub mod app;
pub mod store;

#[wasm_bindgen(module = "/dist/index.js")]
extern "C" {

    #[wasm_bindgen(js_name = "initializeApp", catch)]
    pub fn initialize_app(
        options: FirebaseOptions,
        name: Option<String>,
    ) -> Result<FirebaseApp, JsValue>;

    #[wasm_bindgen(js_name = "initializeFirestore", catch)]
    pub fn initialize_firestore(
        app: FirebaseApp,
        settings: FirestoreSettings,
    ) -> Result<Firestore, JsValue>;

}
