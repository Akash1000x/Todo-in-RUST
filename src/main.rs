use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct NewTodo {
    todo: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    todos: String,
    complete: bool,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/getTodos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    if todos.is_empty() {
        HttpResponse::Ok().json(Vec::<Todo>::new())
    } else {
        HttpResponse::Ok().json(todos.clone()) // Return the list of todos as JSON
    }
}

#[post("/addTodo")]
async fn add_todo(req_body: web::Json<NewTodo>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let new_inner_todo = req_body.into_inner();
    let new_todo = Todo {
        id: (todos.len() + 1) as u32,
        todos: new_inner_todo.todo,
        complete: false,
    };
    todos.push(new_todo);
    HttpResponse::Ok().body("Todo Added")
}

#[put("/updateTodo/{id}")]
async fn update_todo(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();
    if let Some(pos) = todos.iter().position(|x| x.id == id) {
        todos[pos].complete = !todos[pos].complete;
        HttpResponse::Ok().body("Todo Updated")
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[delete("/deleteTodo/{id}")]
async fn delete_todo(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();
    // let some_pos = todos.iter().position(|x| x.id == id);
    // match some_pos {
    //     Some(pos) => {
    //         todos.remove(pos);
    //         HttpResponse::Ok().body("Todo Deleted")
    //     }
    //     None => HttpResponse::NotFound().body("Todo not found"),
    // }

    //or

    // let mut some_pos = None;
    // for (i, todo) in todos.iter().enumerate() {
    //     if todo.id == id {
    //         some_pos = Some(i);
    //         break;
    //     }
    // }
    // match some_pos {
    //     Some(pos) => {
    //         todos.remove(pos);
    //         HttpResponse::Ok().body("Todo Deleted")
    //     }
    //     None => HttpResponse::NotFound().body("Todo not found"),
    // }

    //or

    if let Some(pos) = todos.iter().position(|x| x.id == id) {
        todos.remove(pos);
        HttpResponse::Ok().body("Todo Deleted")
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share the state
            .service(index)
            .service(add_todo)
            .service(get_todos)
            .service(delete_todo)
            .service(update_todo)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
