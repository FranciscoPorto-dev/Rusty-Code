use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;

pub struct App {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub edit_history: Vec<(String, usize)>,
    pub messages: Vec<Message>,
    pub is_streaming: bool,
    api_tx: mpsc::Sender<ApiEvent>,
    api_rx: mpsc::Receiver<ApiEvent>,
    _runtime: Runtime,
    runtime: Handle,
}

impl App {
    pub fn new() -> Self {
        let runtime = Runtime::new().expect("failed to create Tokio runtime");
        let runtime_handle = runtime.handle().clone();
        let (api_tx, api_rx) = mpsc::channel(32);

        Self {
            messages: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
            edit_history: Vec::new(),
            is_streaming: false,
            api_tx,
            api_rx,
            _runtime: runtime,
            runtime: runtime_handle,
        }
    }
}

pub enum ApiEvent {
    Delta(String),
    Done,
    Error(String),
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
