use super::model::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodoEntry, TodoStatus};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

#[get("/todos")]
async fn get_all(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todo_entries.lock().unwrap().to_vec())
}

#[get("/todos/{id}")]
async fn get(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner(); // get id from path
    let todo_entries = data.todo_entries.lock().unwrap();
    let todo = todo_entries.iter().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        return HttpResponse::NotFound().json(format!("{} is not found", id));
    }

    let todo = todo.unwrap();

    HttpResponse::Ok().json(todo)
    
}

#[post("/todos")]
async fn create(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateEntryData>,
) -> impl Responder {
    let mut todo_entries = data.todo_entries.lock().unwrap();
    let id = Uuid::new_v4();
    todo_entries.push(TodoEntry {
        id: Some(id.to_string()),
        date: Utc::now(),
        title: param_obj.title.clone(),
        status: TodoStatus::Pending,
    });

    HttpResponse::Ok().json(todo_entries.to_vec())
}

#[put("/todos/{id}")]
async fn update(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateEntryData>,
) -> impl Responder {
    let id = path.into_inner();
    let mut todo_entries = data.todo_entries.lock().unwrap();
    let todo = todo_entries.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        return HttpResponse::NotFound().json(format!("{} is not found", id));
    }

    let todo = todo.unwrap();

    let title = body.title.to_owned().unwrap_or(todo.title.to_owned());
    todo.title = title;
    todo.date = Utc::now();

    HttpResponse::Ok().json(todo_entries.to_vec())
}

#[post("/todos/{id}/status")]
async fn update_status(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let mut todo_entries = data.todo_entries.lock().unwrap();
    let todo = todo_entries.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        return HttpResponse::NotFound().json(format!("{} is not found", id));
    }

    let todo = todo.unwrap();
    match todo.status {
        TodoStatus::Pending => todo.status = TodoStatus::InProgress,
        TodoStatus::InProgress => todo.status = TodoStatus::Done,
        TodoStatus::Done => (),
    }

    HttpResponse::Ok().json(todo_entries.to_vec())
}

#[delete("/todos/{id}")]
async fn delete(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let mut todo_entries = data.todo_entries.lock().unwrap();
    let todo = todo_entries.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        return HttpResponse::NotFound().json(format!("{} is not found", id));
    }

    *todo_entries = todo_entries.to_vec().into_iter().filter(|x| x.id != Some(id.clone())).collect();

    HttpResponse::Ok().json(todo_entries.to_vec())
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_all)
    .service(get)
    .service(create)
    .service(delete)
    .service(update)
    .service(update_status);
}
