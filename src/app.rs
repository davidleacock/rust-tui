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


// TODO: Maybe a global editing mode?  

#[derive(Debug)]
pub struct App {
    pub todo_items: Vec<TodoItem>,
    pub current_todo_item: usize,
    pub todo_editing_mode: bool,
    pub current_todo_input: String,
    pub active_window: Window,
    pub notepad_content: String,
    pub notepad_editing_mode: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            todo_items: vec![],
            current_todo_item: 0,
            todo_editing_mode: false,
            current_todo_input: String::new(),
            active_window: Window::Notepad,
            notepad_content: String::new(),
            notepad_editing_mode: true,
        }
    }
}
