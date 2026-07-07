pub struct App {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub edit_history: Vec<(String, usize)>,
    pub messages: Vec<Message>,
}

impl App {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
            edit_history: Vec::new(),
        }
    }
}

pub num ApiEvent {
    Delta(String),
    Done,
    Error(String),
}


pub is_streaming:bool,
api_tx: mpsc::Sender<ApiEvent>,
api_rx: mpsc::Receiver<ApiEvent>,
runtime: tokio:runtime::Handle,

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
