use crate::{orderbook::*, GLOBAL_TRADE_ID};
use std::sync::{Arc, Mutex};
use actix_web::web;

use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub enum Status {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Fills {
    price: usize,
    quantity: usize,
    tradeid: usize,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Fillresult {
    pub status: Status,
    pub executedqty: usize,
    pub fills: Vec<Fills>,
}

pub fn fill_order(
    order_id: usize,
    price: usize,
    quantity: &mut usize,
    kind: Kind,
    bookwithquantity:  web::Data<Arc<Mutex<BookWithQuantity>>>,
    orderbook:  web::Data<Arc<Mutex<OrderBook>>>,
) -> Fillresult {
    let mut fills: Vec<Fills> = Vec::new();

    let max_fill_quantity = get_fill_amount(price, *quantity, kind, &orderbook);

    let mut executedqty = 0;

    if max_fill_quantity < *quantity {
        Fillresult {
            status: Status::Rejected,
            executedqty: max_fill_quantity,
            fills: vec![],
        }
    } else {
        let mut book = orderbook.lock().unwrap();
        let mut bookwithquantity = bookwithquantity.lock().unwrap();

        let mut to_remove = Vec::new();
        let mut to_update = Vec::new();

        match kind {
            Kind::BUY => {
                for o in book.asks.iter_mut() {
                    if o.order.price <= price {
                        let filledqty = std::cmp::min(*quantity, o.order.quantity);
                        o.order.quantity -= filledqty;

                        let bookedqty = bookwithquantity.asks[&price];
                        to_update.push((price, bookedqty - filledqty));

                        fills.push(Fills {
                            price: o.order.price,
                            quantity: filledqty,
                            tradeid: GLOBAL_TRADE_ID + 1,
                        });
                        executedqty += filledqty;
                        *quantity -= filledqty;

                        if o.order.quantity == 0 {
                            to_remove.push(o.order.price);
                        }
                    }
                }

                for price in to_remove {
                    let index = book.index_of_ask(price).expect("ask not found");
                    book.asks.remove(index);
                }

                for (price, qty) in to_update {
                    if qty == 0 {
                        bookwithquantity.asks.remove(&price);
                    } else {
                        bookwithquantity.asks.insert(price, qty);
                    }
                }

                if *quantity != 0 {
                    book.bids.push(Bid {
                        order: Order {
                            price,
                            quantity: *quantity,
                            order_id,
                        },
                        side: Kind::BUY,
                    });
                    *bookwithquantity.bids.entry(price).or_insert(0) += *quantity;
                }
            }

            Kind::SELL => {
                for o in book.bids.iter_mut() {
                    if o.order.price >= price {
                        let filledqty = std::cmp::min(*quantity, o.order.quantity);
                        o.order.quantity -= filledqty;

                        let bookedqty = bookwithquantity.bids[&price];
                        to_update.push((price, bookedqty - filledqty));

                        fills.push(Fills {
                            price: o.order.price,
                            quantity: filledqty,
                            tradeid: GLOBAL_TRADE_ID + 1,
                        });
                        executedqty += filledqty;
                        *quantity -= filledqty;

                        if o.order.quantity == 0 {
                            to_remove.push(o.order.price);
                        }
                    }
                }

                for price in to_remove {
                    let index = book.index_of_bid(price).expect("bid not found");
                    book.bids.remove(index);
                }

                for (price, qty) in to_update {
                    if qty == 0 {
                        bookwithquantity.bids.remove(&price);
                    } else {
                        bookwithquantity.bids.insert(price, qty);
                    }
                }

                if *quantity != 0 {
                    book.asks.push(Ask {
                        order: Order {
                            price,
                            quantity: *quantity,
                            order_id,
                        },
                        side: Kind::SELL,
                    });
                    *bookwithquantity.asks.entry(price).or_insert(0) += *quantity;
                }
            }
        }

        Fillresult {
            status: Status::Accepted,
            executedqty,
            fills,
        }
    }
}

pub fn get_fill_amount(
    price: usize,
    quantity: usize,
    kind: Kind,
    orderbook: &Arc<Mutex<OrderBook>>,
) -> usize {
    let mut filled = 0;
    let book = orderbook.lock().unwrap();

    match kind {
        Kind::BUY => {
            for o in book.asks.iter() {
                if o.order.price < price {
                    filled = std::cmp::min(quantity, o.order.quantity);
                }
            }
        }
        Kind::SELL => {
            for o in book.asks.iter() {
                if o.order.price > price {
                    filled = std::cmp::min(quantity, o.order.quantity);
                }
            }
        }
    }

    filled
}

// fn get_order_id() -> usize {
//     1
// }
