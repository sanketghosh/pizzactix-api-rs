use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use std::io::Result;
mod models;
use crate::models::pizza::BuyPizzaRequest;
use validator::Validate;

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
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a pizza")
}

// main
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(post_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}
