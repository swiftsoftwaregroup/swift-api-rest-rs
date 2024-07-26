use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod models;
mod schema;

use models::{Book, NewBook};

#[utoipa::path(
    post,
    path = "/books",
    request_body = NewBook,
    responses(
        (status = 200, description = "Book created successfully", body = Book),
        (status = 500, description = "Internal server error")
    ),
    tag = "Books"
)]
async fn create_book(pool: web::Data<db::DbPool>, new_book: web::Json<NewBook>) -> impl Responder {
    match db::create_book(&pool, new_book.into_inner()) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/books",
    responses(
        (status = 200, description = "List of all books", body = Vec<Book>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Books"
)]
async fn get_all_books(pool: web::Data<db::DbPool>) -> impl Responder {
    match db::get_all_books(&pool) {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/books/{id}",
    responses(
        (status = 200, description = "Book found", body = Book),
        (status = 404, description = "Book not found")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
async fn get_book(pool: web::Data<db::DbPool>, book_id: web::Path<i32>) -> impl Responder {
    match db::get_book(&pool, book_id.into_inner()) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
    put,
    path = "/books/{id}",
    request_body = NewBook,
    responses(
        (status = 200, description = "Book updated successfully", body = Book),
        (status = 404, description = "Book not found")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
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

#[utoipa::path(
    delete,
    path = "/books/{id}",
    responses(
        (status = 204, description = "Book deleted successfully"),
        (status = 404, description = "Book not found")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
async fn delete_book(pool: web::Data<db::DbPool>, book_id: web::Path<i32>) -> impl Responder {
    match db::delete_book(&pool, book_id.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_book,
        get_all_books,
        get_book,
        update_book,
        delete_book
    ),
    components(
        schemas(Book, NewBook)
    ),
    tags(
        (name = "Books", description = "Book management operations")
    ),
    info(
        title = "Book Management API",
        version = "1.0.0",
        description = "A simple API for managing books"
    )    
)]
struct ApiDocs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::redirect("/docs", "/docs/"))
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", ApiDocs::openapi()),
            )
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