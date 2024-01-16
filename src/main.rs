mod todo;

use todo::Todo;
use todo::TodoService;

fn print(todo: &Todo) {
    println!("#{} {}: {}", todo.id, todo.title, todo.description)
}

fn main() {
    let mut service = TodoService::new();

    service.add(String::from("First Todo"), String::from("Some description"));
    service.add("Second Todo".to_string(), "Some description".to_string());
    service.add("Third Todo".to_string(), "Some description".to_string());

    let todos = service.list();
    for todo in todos {
        print(todo)
    }

    let id = 1;
    service.update(
        id,
        String::from("New title"),
        String::from("New description"),
    );

    let todo = service.get(id).unwrap();
    print(todo)
}
