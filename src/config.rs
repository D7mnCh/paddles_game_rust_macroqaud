pub struct WindowConfig {
    pub screen_width: i32,
    pub screen_height: i32,
    pub window_title: &'static str,
}
pub struct Config {
    pub window_config: WindowConfig,
}
impl Config {
    pub fn init() -> Self {
        Self {
            window_config: WindowConfig {
                screen_width: 1200,
                screen_height: 800,
                window_title: "ping_pong",
            },
        }
    }
}
