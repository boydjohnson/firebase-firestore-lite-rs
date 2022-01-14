use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct FirebaseApp {
    #[wasm_bindgen(js_name = "automaticDataCollectionEnabled")]
    pub automatic_data_collection_enabled: bool,

    pub name: String,

    pub options: FirebaseOptions,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
pub struct FirebaseOptions {
    pub api_key: String,
    pub app_id: Option<String>,
    pub auth_domain: Option<String>,
    pub database_url: Option<String>,
    pub measurement_id: Option<String>,
    pub messaging_sender_id: Option<String>,
    pub project_id: Option<String>,
    pub storage_bucket: Option<String>,
}

#[wasm_bindgen]
impl FirebaseOptions {
    pub fn new(api_key: String) -> Self {
        FirebaseOptions {
            api_key,
            ..Default::default()
        }
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
