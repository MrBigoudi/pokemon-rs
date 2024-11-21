#[derive(Debug, Clone)]
pub struct ApplicationParameters {
    pub window_title: String,
    pub window_width: u16,
    pub window_height: u16,
}

impl Default for ApplicationParameters {
    fn default() -> Self {
        Self {
            window_title: String::from("untitled"),
            window_width: 800,
            window_height: 600,
        }
    }
}