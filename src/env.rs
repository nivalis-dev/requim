pub struct AppEnv {
    pub dummy: String,
}

impl AppEnv {
    pub fn new() -> Self {
        Self {
            dummy: "dummy".to_string(),
        }
    }
}
