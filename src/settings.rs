pub struct Settings {
    pub installed_only: bool,
}

impl Settings {
    pub fn defaults() -> Self {
        Self {
            installed_only: false,
        }
    }
}
