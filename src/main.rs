use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use rand::Rng;

mod orderbook;
mod utils;
mod test;

use orderbook::*;
use utils::*;

const BASE_ASSET: &str = "BTC";
const QUOTE_ASSET: &str = "USD";
pub const GLOBAL_TRADE_ID: usize = 0;

#[post("/api/v1/order")]
async fn post_order(
    req: web::Json<OrderInputSchema>,
    bookwithqty: web::Data<Arc<Mutex<BookWithQuantity>>>,
    orderbook: web::Data<Arc<Mutex<OrderBook>>>,
) -> impl Responder {
    let order_data = req.into_inner();
    let base_asset = order_data.base_asset;
    let quote_asset = order_data.quote_asset;
    let price = order_data.price;
    let mut quantity = order_data.quantity;
    let kind = order_data.side;

    let order_id = get_order_id();

    if base_asset != BASE_ASSET || quote_asset != QUOTE_ASSET {
        return HttpResponse::BadRequest().body("Invalid assets");
    }

    // let mut bookwithqty = bookwithqty.lock().unwrap();
    // let mut orderbook = orderbook.lock().unwrap();

    // let bookwithquantity_arc = Arc::new(Mutex::new(bookwithquantity));
    // let orderbook_arc = Arc::new(Mutex::new(orderbook));

    // Call fill_order and get the result
    let fill_result = fill_order(
        order_id,
        price,
        &mut quantity,
        kind,
        bookwithqty,
        orderbook
    );

    // Prepare the response
    HttpResponse::Ok().json(serde_json::json!({
        "orderId": order_id,
        "executedQty": fill_result.executedqty,
        "fills": fill_result.fills
    }))
}

fn get_order_id() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>() as usize  // Generate a random u64 and cast it to usize
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
    // Instantiate and share the BookWithQuantity and OrderBook instances
    let bookwithquantity = Arc::new(Mutex::new(BookWithQuantity::new()));
    let orderbook = Arc::new(Mutex::new(OrderBook {
        bids: Vec::new(),
        asks: Vec::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bookwithquantity.clone()))
            .app_data(web::Data::new(orderbook.clone()))
            .service(hello)
            .service(echo)
            .service(post_order)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
