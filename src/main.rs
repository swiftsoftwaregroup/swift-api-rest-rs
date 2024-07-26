mod db;
mod models;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use models::NewBook;

async fn create_book(pool: web::Data<db::DbPool>, new_book: web::Json<NewBook>) -> impl Responder {
    match db::create_book(&pool, new_book.into_inner()) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_all_books(pool: web::Data<db::DbPool>) -> impl Responder {
    match db::get_all_books(&pool) {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_book(pool: web::Data<db::DbPool>, book_id: web::Path<i32>) -> impl Responder {
    match db::get_book(&pool, book_id.into_inner()) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn update_book(
    pool: web::Data<db::DbPool>,
    book_id: web::Path<i32>,
    updated_book: web::Json<NewBook>,
) -> impl Responder {
    match db::update_book(&pool, book_id.into_inner(), updated_book.into_inner()) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn delete_book(pool: web::Data<db::DbPool>, book_id: web::Path<i32>) -> impl Responder {
    match db::delete_book(&pool, book_id.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/books", web::post().to(create_book))
            .route("/books", web::get().to(get_all_books))
            .route("/books/{id}", web::get().to(get_book))
            .route("/books/{id}", web::put().to(update_book))
            .route("/books/{id}", web::delete().to(delete_book))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
