use crate::{orderbook::*, GLOBAL_TRADE_ID};
use std::sync::{Arc, Mutex};

pub enum Status {
    Accepted,
    Rejected,
}

pub struct Fills {
    price: usize,
    quantity: usize,
    tradeid: usize,
}
pub struct Fillresult {
    status: Status,
    executedqty: usize,
    fills: Vec<Fills>,
}

pub fn fill_order(
    order_id: usize,
    price: usize,
    quantity: usize,
    kind: Kind,
    bookwithquantity: &mut Arc<Mutex<BookWithQuantity >>,
    orderbook: &mut Arc<Mutex<OrderBook>>

) -> Fillresult {
    let fills: Vec<Fills> = Vec::new();

    let max_fill_quantity = get_fill_amount(price, quantity, kind, &book);

    let executedqty = 0;

    if max_fill_quantity < quantity {

        Fillresult {
            status: Status::Rejected,
            executedqty: max_fill_quantity,
            fills: vec![],
        }
    }else{

    let mut book = orderbook.lock().unwrap();
    let mut bookwithquantity = bookwithquantity.lock().unwrap();

    match kind {
        Kind::BUY => {
            book.asks.iter().map(|o|{

                if o.order.price >= price {
                    let filledqty = std::cmp::min(quantity,o.order.quantity);
                    o.order.quantity -= filledqty;
                    bookwithquantity.asks[&price] = bookwithquantity.asks[&price] - filledqty;
                    let fill = Fills{
                        price:o.order.price,
                        quantity:filledqty,
                        tradeid: GLOBAL_TRADE_ID + 1
                    };
                    fills.push(fill);
                    executedqty += filledqty;
                    quantity -= filledqty;

                    if o.order.quantity == 0{
                        book.asks.splice(book.index_of_ask(o.order.price), 1);
                    }

                    if bookwithquantity.asks[&price] == 0{
                        bookwithquantity.asks.remove(&price);
                    }
                }
            });
            if quantity !=0 {
                book.bids.push(Bid{
                    order: Order{
                        price:price,
                        quantity: quantity-executedqty,
                        order_id
                    },
                    side: Kind::SELL
                });
                bookwithquantity.bids[&price] = bookwithquantity.bids[&price] + quantity-executedqty;
            }
        },

        Kind::SELL => {
            book.bids.iter().map(|o|{

                if o.order.price >= price {
                    let filledqty = std::cmp::min(quantity,o.order.quantity);
                    o.order.quantity -= filledqty;
                    bookwithquantity.bids[&price] = bookwithquantity.bids[&price] - filledqty;
                    let fill = Fills{
                        price:o.order.price,
                        quantity:filledqty,
                        tradeid: GLOBAL_TRADE_ID + 1
                    };
                    fills.push(fill);
                    executedqty += filledqty;
                    quantity -= filledqty;

                    if o.order.quantity == 0{
                        book.bids.splice(book.index_of_bid(o.order.price), 1);
                    }

                    if bookwithquantity.bids[&price] == 0{
                        bookwithquantity.bids.remove(&price);
                    }
                }
            });
            if quantity !=0 {
                book.asks.push(Ask{
                    order: Order{
                        price:price,
                        quantity: quantity-executedqty,
                        order_id
                    },
                    side: Kind::SELL
                });
                bookwithquantity.asks[&price] = bookwithquantity.asks[&price] + quantity-executedqty;
            }
            
        }
        
    }

    Fillresult{
        status:Status::Accepted,
        executedqty,
        fills
    }

    }

    
}

pub fn get_fill_amount(
    price: usize,
    quantity: usize,
    kind: Kind,
    orderbook: &Arc<Mutex<OrderBook>>
) -> usize {
    let mut filled = 0;
    let book = orderbook.lock().unwrap();

    match kind {
        Kind::BUY => {
            for o in book.asks.iter() {
                if o.0 < &price {
                    filled = std::cmp::min(quantity, *o.1);
                }
            }
        }
        Kind::SELL => {
            for o in book.asks.iter() {
                if o.0 > &price {
                    filled = std::cmp::min(quantity, *o.1);
                }
            }
        }
    }

    filled
}

fn get_order_id()->usize{
    1
}