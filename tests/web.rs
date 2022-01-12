use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use firebase_firestore_lite::initialize_app;
use firebase_firestore_lite::FirebaseOptions;

#[wasm_bindgen_test]
fn test_intialize_app() {
    let options = FirebaseOptions::new("example".into());

    assert!(initialize_app(options, None).is_err());
}
