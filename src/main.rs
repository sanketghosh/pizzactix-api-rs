use actix_web::{
    get, patch, post, web::Data, web::Json, web::Path, App, HttpResponse, HttpServer, Responder,
};
use std::io::Result;
// use surrealdb::sql::Data;
mod models;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL};
use validator::Validate;
mod db;
use crate::db::Database;

// handlers
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available")
}

#[post("/buypizza")]
async fn post_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Pizza name required"),
    }
}

#[patch("/updatepizza/{id}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let id = update_pizza_url.into_inner().id;
    HttpResponse::Ok().body(format!("Updating a pizza {id}"))
}

// main
#[actix_web::main]
async fn main() -> Result<()> {
    let db = Database::init()
        .await
        .expect("ERROR: cannot connect to the database");

    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(post_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}
