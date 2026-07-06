pub struct App {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub edit_history: Vec<(String, usize)>,
}

pub enum InputMode {
    Normal,
    Editing,
}
