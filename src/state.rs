pub struct AppState {
    pub project: Option<String>,
}

impl AppState {
    pub fn init() -> Self {
        AppState { project: None }
    }
}
