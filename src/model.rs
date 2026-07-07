pub struct App {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub edit_history: Vec<(String, usize)>,
    pub messages: Vec<Message>,
}

pub enum Role {
    User,
    Assistant,
}


pub struct Message {
    pub content: String,
    pub role: Role,
}


pub enum InputMode {
    Normal,
    Editing,
}
