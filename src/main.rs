use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod orderbook;
mod utils;
mod test;

use orderbook::*;

const BASE_ASSET: &str = "BTC";
const QUOTE_ASSET: &str = "USD";
pub const GLOBAL_TRADE_ID: usize = 0;

#[post("/api/v1/order")]
async fn post_order(
    req: web::Json<OrderInputSchema>,
    data: web::Data<Arc<Mutex<BookWithQuantity>>>,
) -> impl Responder {
    let order_data = req.into_inner();
    let base_asset = order_data.base_asset;
    let quote_asset = order_data.quote_asset;
    let price = order_data.price;
    let quantity = order_data.quantity;
    let side = order_data.side;
    let kind = order_data.kind;

    let order_id = get_order_id();

    if base_asset != BASE_ASSET && quote_asset != QUOTE_ASSET {
        return HttpResponse::BadRequest().body("Invalid assets");
    }

    // Access and mutate the shared BookWithQuantity instance
    let mut book = data.lock().unwrap();
    

    HttpResponse::Ok().json(book)
}

fn get_order_id()->usize{
    1
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Instantiate and share the BookWithQuantity instance
    let book = Arc::new(Mutex::new(BookWithQuantity::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(book.clone()))
            .service(hello)
            .service(echo)
            .service(post_order)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
