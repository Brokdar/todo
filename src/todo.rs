pub enum Status {
    Open,
    InProgress,
    Closed,
    Canceled,
    OnHold,
}

pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: Status,
}

pub struct TodoService {
    id: u32,
    items: Vec<Todo>,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            id: 0,
            items: vec![],
        }
    }

    pub fn list(&self) -> &Vec<Todo> {
        &self.items
    }

    pub fn add(&mut self, title: String, description: String) {
        let todo = Todo {
            id: self.id,
            title,
            description,
            status: Status::Open,
        };
        self.items.push(todo);
        self.id += 1;
    }

    pub fn get(&self, id: u32) -> Option<&Todo> {
        self.items.iter().find(|&x| x.id == id)
    }

    pub fn update(&mut self, id: u32, title: String, description: String) {
        let todo = self.items.iter_mut().find(|x| x.id == id);
        match todo {
            Some(todo) => {
                todo.title = title;
                todo.description = description;
            }
            None => println!("Warning: cannot update the todo with the id={}", id),
        }
    }
}
