use std::{clone, collections::HashMap};
use serde::{Serialize,Deserialize};

// #[derive(PartialEq,PartialOrd,Debug)]
// pub enum OrderType {
//     LIMIT,
//     MARKET,
// }

#[derive(PartialEq,PartialOrd,Debug)]
pub enum Kind {
    BUY,
    SELL,
}
pub enum Market {
    TataInr,
    GoogleDollar,
    NvidiaInr,
    TeslaDollar,
}
pub enum FillStatus {
    Unfilled,
    PartiallyFilled,
    Filled,
}
pub struct Order {
    pub order_id: usize,
    pub price: usize,
    pub quantity: usize,
    // pub order_type: T,
}
pub struct Bid {
    pub order: Order,
    pub side: Kind,
}
pub struct Ask {
    pub order: Order,
    pub side: Kind,
}
pub struct OrderBook {
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
}
impl OrderBook{
    pub fn index_of_ask(&self, price:usize)-> Option<usize>{
        self.asks.iter().position(|o| o.order.price == price)
    }
    pub fn index_of_bid(&self, price:usize)-> Option<usize>{
        self.bids.iter().position(|o| o.order.price == price)
    }
    
}

pub const ORDERBOOK: OrderBook = OrderBook {
    bids: Vec::new(),
    asks: Vec::new(),
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct OrderInputSchema{
    pub base_asset: String,
    pub quote_asset: String,
    pub price:usize,
    pub quantity:usize,
    pub side: Kind,
    // pub typ: OrderType,
}


#[derive(Debug)]
pub struct BookWithQuantity{
    pub bids :HashMap<usize,usize>,
    pub asks: HashMap<usize,usize>
}

impl BookWithQuantity{
    pub fn new()-> BookWithQuantity{
        BookWithQuantity{
            bids: HashMap::new(),
            asks: HashMap::new()
        }
    }
}

