#[cfg(test)]
mod tests {
    use crate::{fill_order, Ask, Bid, BookWithQuantity, Kind, Order, OrderBook, Status};

    use super::*;
    use actix_web::web::Data;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[test]
    fn test_fill_order_buy_partial_fill() {
        let book = Arc::new(Mutex::new(OrderBook {
            bids: vec![],
            asks: vec![Ask {
                order: Order {
                    order_id: 1,
                    price: 100,
                    quantity: 5,
                },
                side: Kind::SELL,
            }],
        }));

        let book_qty = Arc::new(Mutex::new(BookWithQuantity {
            bids: HashMap::new(),
            asks: [(100, 5)].iter().cloned().collect(),
        }));

        let order_id = 2;
        let mut quantity = 3;
        let result = fill_order(
            order_id,
            100,
            &mut quantity,
            Kind::BUY,
            Data::new(book_qty.clone()),
            Data::new(book.clone()),
        );

        assert_eq!(result.status, Status::Accepted);
        assert_eq!(result.executedqty, 3);
        assert_eq!(result.fills.len(), 1);
        assert_eq!(result.fills[0].price, 100);
        assert_eq!(result.fills[0].quantity, 3);
    }

    #[test]
    fn test_fill_order_sell_full_fill() {
        let book = Arc::new(Mutex::new(OrderBook {
            bids: vec![Bid {
                order: Order {
                    order_id: 1,
                    price: 110,
                    quantity: 10,
                },
                side: Kind::BUY,
            }],
            asks: vec![],
        }));

        let book_qty = Arc::new(Mutex::new(BookWithQuantity {
            bids: [(110, 10)].iter().cloned().collect(),
            asks: HashMap::new(),
        }));

        let order_id = 3;
        let mut quantity = 10;
        let result = fill_order(
            order_id,
            105,
            &mut quantity,
            Kind::SELL,
            Data::new(book_qty.clone()),
            Data::new(book.clone()),
        );

        assert_eq!(result.status, Status::Accepted);
        assert_eq!(result.executedqty, 10);
        assert_eq!(result.fills.len(), 1);
        assert_eq!(result.fills[0].price, 110);
        assert_eq!(result.fills[0].quantity, 10);
    }

    #[test]
    fn test_fill_order_rejected() {
        let book = Arc::new(Mutex::new(OrderBook {
            bids: vec![],
            asks: vec![Ask {
                order: Order {
                    order_id: 1,
                    price: 120,
                    quantity: 2,
                },
                side: Kind::SELL,
            }],
        }));

        let book_qty = Arc::new(Mutex::new(BookWithQuantity {
            bids: HashMap::new(),
            asks: [(120, 2)].iter().cloned().collect(),
        }));

        let order_id = 4;
        let mut quantity = 5;
        let result = fill_order(
            order_id,
            110,
            &mut quantity,
            Kind::BUY,
            Data::new(book_qty.clone()),
            Data::new(book.clone()),
        );

        assert_eq!(result.status, Status::Rejected);
        assert_eq!(result.executedqty, 0);
        assert_eq!(result.fills.len(), 0);
    }
}
