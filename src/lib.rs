use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/dist/main.js")]
extern "C" {}

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct FirebaseOptions {
    api_key: String,
    app_id: Option<String>,
    auth_domain: Option<String>,
    database_url: Option<String>,
    measurement_id: Option<String>,
    messaging_sender_id: Option<String>,
    project_id: Option<String>,
    storage_bucket: Option<String>,
}

#[wasm_bindgen]
impl FirebaseOptions {
    pub fn new(api_key: String) -> Self {
        FirebaseOptions {
            api_key,
            ..Default::default()
        }
    }

    #[wasm_bindgen(js_name = "storageBucket", getter)]
    pub fn storage_bucket(&self) -> Option<String> {
        self.storage_bucket.clone()
    }

    #[wasm_bindgen(js_name = "projectId", getter)]
    pub fn project_id(&self) -> Option<String> {
        self.project_id.clone()
    }

    #[wasm_bindgen(js_name = "messagingSenderId", getter)]
    pub fn messaging_sender_id(&self) -> Option<String> {
        self.messaging_sender_id.clone()
    }

    #[wasm_bindgen(js_name = "measurementId", getter)]
    pub fn measurement_id(&self) -> Option<String> {
        self.measurement_id.clone()
    }

    #[wasm_bindgen(js_name = "databaseUrl", getter)]
    pub fn database_url(&self) -> Option<String> {
        self.database_url.clone()
    }

    #[wasm_bindgen(js_name = "authDomain", getter)]
    pub fn auth_domain(&self) -> Option<String> {
        self.auth_domain.clone()
    }

    #[wasm_bindgen(js_name = "appId", getter)]
    pub fn app_id(&self) -> Option<String> {
        self.app_id.clone()
    }

    #[wasm_bindgen(js_name = "apiKey", getter)]
    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn with_app_id(mut self, app_id: String) -> Self {
        self.app_id = Some(app_id);
        self
    }

    pub fn with_auth_domain(mut self, auth_domain: String) -> Self {
        self.auth_domain = Some(auth_domain);
        self
    }

    pub fn with_database_url(mut self, database_url: String) -> Self {
        self.database_url = Some(database_url);
        self
    }

    pub fn with_measurement_id(mut self, measurement_id: String) -> Self {
        self.measurement_id = Some(measurement_id);
        self
    }

    pub fn with_messaging_sender_id(mut self, messaging_sender_id: String) -> Self {
        self.messaging_sender_id = Some(messaging_sender_id);
        self
    }

    pub fn with_project_id(mut self, project_id: String) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn with_storage_bucket(mut self, storage_bucket: String) -> Self {
        self.storage_bucket = Some(storage_bucket);
        self
    }
}
