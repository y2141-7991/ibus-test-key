#[derive(Debug, Clone, Copy)]
pub struct InputMode {
    pub prop_name: &'static str,
    pub mode: i32,
}

impl PartialEq for InputMode {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
    }
}

impl InputMode {
    const fn new(prop_name: &'static str, mode: i32) -> InputMode {
        InputMode { prop_name, mode }
    }
}

pub const INPUT_MODE_TELEX: InputMode = InputMode::new("prop_name", 0);
