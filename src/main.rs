mod template;
mod todo;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::Form;
use template::{HtmlTemplate, IndexTemplate, TodosTemplate};
use todo::{Todo, TodoForm};
use tokio::net::TcpListener;

const TODO_FILE_PATH: &str = "./src/todos.json";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = axum::Router::new()
        .route("/", get(index))
        .route("/todo", get(get_todos).post(create_todo))
        //:id defines path parameters for our route
        .route("/todo/:id", delete(delete_todo));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

//read our todos.json file
async fn read_todos() -> Vec<Todo> {
    let file = std::fs::read_to_string(TODO_FILE_PATH).expect("could not read file");
    let todos = serde_json::from_str(&file).expect("error parsing json");
    todos
}

//get todos handler
async fn get_todos() -> impl IntoResponse {
    let template = TodosTemplate {
        todos: read_todos().await,
    };
    HtmlTemplate(template)
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

//delete todo handler
async fn delete_todo(Path(id): Path<u32>) -> impl IntoResponse {
    let mut todos = read_todos().await;
    // removes todo matching id from the routes path
    todos.retain(|todo| todo.id != id as usize);

    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(TODO_FILE_PATH)
        .unwrap();

    //writes over json with updated todo list
    serde_json::to_writer(file, &todos).unwrap();

    HtmlTemplate(TodosTemplate { todos })
}

pub async fn create_todo(form: Form<TodoForm>) -> impl IntoResponse {
    let mut todos = read_todos().await;

    //create an id for our todos, a random or uuid would be better
    //but this works fine
    let id = todos.len() as u32 + 1;

    //add new todo using data from our form
    todos.push(Todo {
        id: id as usize,
        text: form.text.clone(),
    });

    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(TODO_FILE_PATH)
        .unwrap();
    //writes over json with updated todo list
    serde_json::to_writer(file, &todos).unwrap();

    HtmlTemplate(TodosTemplate { todos })
}
