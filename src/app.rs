#[derive(Debug)]
pub struct TodoItem {
    pub description: String,
    pub checked: bool,
}

#[derive(PartialEq, Debug)]
pub enum Window {
    TodoList,
    Notepad,
}

#[derive(Debug)]
pub struct App {
    pub items: Vec<TodoItem>,
    pub selected: usize,
    pub todo_editing: bool,
    pub current_input: String,
    pub active_window: Window,
    pub notepad_content: String,
    pub notepad_editing: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            items: vec![],
            selected: 0,
            todo_editing: false,
            current_input: String::new(),
            active_window: Window::Notepad,
            notepad_content: String::new(),
            notepad_editing: true,
        }
    }
}
