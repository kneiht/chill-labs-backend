use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
}

impl AppState {
    pub fn new(settings: &Settings) -> Self {
        Self {
            settings: settings.clone(),
        }
    }
}
