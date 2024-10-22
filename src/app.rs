pub struct TodoItem {
    pub description: String,
    pub checked: bool,
}

pub struct TodoApp {
    pub items: Vec<TodoItem>,
    pub selected: usize,
    pub editing: bool,
    pub current_input: String,
}

impl TodoApp {
    pub fn new() -> Self {
        TodoApp {
            items: vec![],
            selected: 0,
            editing: false,
            current_input: String::new(),
        }
    }
}