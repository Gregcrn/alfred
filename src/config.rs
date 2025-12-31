#[derive(Clone)]
pub struct Config {
    pub enabled: bool,
}

pub fn load_config() -> Config {
    Config { enabled: true }
}
