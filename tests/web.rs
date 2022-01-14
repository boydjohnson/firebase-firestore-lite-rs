use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use firebase_firestore_lite::{
    app::{FirebaseApp, FirebaseOptions},
    initialize_app, initialize_firestore,
    store::FirestoreSettings,
};

#[wasm_bindgen_test]
fn test_initialize_app() {
    let options = FirebaseOptions::new("example".into());

    assert!(initialize_app(options, None).is_err());
}

#[wasm_bindgen_test]
fn test_initialize_firestore() {
    let options = FirebaseOptions::new("test".into());
    let app = FirebaseApp {
        automatic_data_collection_enabled: false,
        name: "test".into(),
        options,
    };

    let settings = FirestoreSettings::default();

    assert!(initialize_firestore(app, settings).is_err());
}
