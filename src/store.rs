use crate::app::FirebaseApp;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct FirestoreSettings {
    #[wasm_bindgen(js_name = "cacheSizeBytes")]
    pub cache_size_bytes: Option<u64>,
    #[wasm_bindgen(js_name = "experimentalAutoDetectLongPolling")]
    pub experimental_auto_detect_long_polling: Option<bool>,
    #[wasm_bindgen(js_name = "experimentalForceLongPolling")]
    pub experimental_force_long_polling: Option<bool>,
    pub host: Option<String>,
    #[wasm_bindgen(js_name = "ignoreUndefinedProperties")]
    pub ignore_undefined_properties: Option<bool>,
    pub ssl: Option<bool>,
}

impl FirestoreSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cache_size_bytes(mut self, cache_size_bytes: u64) -> Self {
        self.cache_size_bytes = Some(cache_size_bytes);
        self
    }

    pub fn with_experimental_auto_detect_long_polling(mut self, yes: bool) -> Self {
        self.experimental_auto_detect_long_polling = Some(yes);
        self
    }

    pub fn with_experimental_force_long_polling(mut self, yes: bool) -> Self {
        self.experimental_force_long_polling = Some(yes);
        self
    }

    pub fn with_host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn with_ignore_undefined_properties(mut self, yes: bool) -> Self {
        self.ignore_undefined_properties = Some(yes);
        self
    }

    pub fn with_ssl(mut self, yes: bool) -> Self {
        self.ssl = Some(yes);
        self
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Firestore {
    pub app: FirebaseApp,
    #[wasm_bindgen(js_name = "type")]
    pub kind: String,
}
