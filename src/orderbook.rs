use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Kind {
    BUY,
    SELL,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub enum Market {
    TataInr,
    GoogleDollar,
    NvidiaInr,
    TeslaDollar,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub enum FillStatus {
    Unfilled,
    PartiallyFilled,
    Filled,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Order {
    pub order_id: usize,
    pub price: usize,
    pub quantity: usize,
    // pub order_type: T,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Bid {
    pub order: Order,
    pub side: Kind,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct Ask {
    pub order: Order,
    pub side: Kind,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
}

impl OrderBook {
    pub fn index_of_ask(&self, price: usize) -> Option<usize> {
        self.asks.iter().position(|o| o.order.price == price)
    }

    pub fn index_of_bid(&self, price: usize) -> Option<usize> {
        self.bids.iter().position(|o| o.order.price == price)
    }
}

// pub const ORDERBOOK: OrderBook = OrderBook {
//     bids: Vec::new(),
//     asks: Vec::new(),
// };

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OrderInputSchema {
    pub base_asset: String,
    pub quote_asset: String,
    pub price: usize,
    pub quantity: usize,
    pub side: Kind,
    // pub typ: OrderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookWithQuantity {
    pub bids: HashMap<usize, usize>,
    pub asks: HashMap<usize, usize>,
}

impl BookWithQuantity {
    pub fn new() -> BookWithQuantity {
        BookWithQuantity {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }
}
