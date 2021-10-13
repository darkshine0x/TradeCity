use crate::core::trade::Tradeable;
use std::collections::{VecDeque, HashMap};
use crate::accounting::marketdata::Price;

#[derive(Debug, Copy, Clone)]
pub enum OrderType {
    AtMarket,
    Limit
}

pub struct Order<T> where T: Tradeable + 'static {
    underlying: &'static T,
    amount: f64,
    limit: Option<Price>,
    is_fully_executed: bool,
    order_type: OrderType
}

impl<T> Order<T> where T: Tradeable + 'static  {
    pub fn new(underlying: &'static T, amount: f64, limit: Option<Price>) -> Result<Order<T>, String> {
        let validation_result = validate_order(amount, &limit);

        if let Err(message) = validation_result {
            return Err(message);
        }

        // Order is valid

        let new_order = Order {
            underlying,
            amount,
            order_type: match limit {
                Some(_) => OrderType::Limit,
                None => OrderType::AtMarket
            },
            limit,
            is_fully_executed: false
        };

        Ok(new_order)
    }

    pub fn underlying(&self) -> &T {
        &self.underlying
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn limit(&self) -> &Option<Price> {
        &self.limit
    }

    pub fn is_fully_executed(&self) -> bool {
        self.is_fully_executed
    } 

    pub fn order_type(&self) -> OrderType {
        self.order_type
    }
}

fn validate_order(amount: f64, limit: &Option<Price>) -> Result<bool, String> {
    if amount <= 0.0 {
        return Err(String::from("Amount must be greater than zero."));
    }

    if let Some(l) = limit {
        if l.value() <= 0.0 {
            return Err(String::from("Limit must be greater than zero"))
        }
    }

    return Ok(true);
}

pub struct Trade<T> where T: Tradeable + 'static {
    sell_order: Order<T>,
    buy_order: Order<T>
}

impl<T> Trade<T> where T: Tradeable + 'static {
    pub fn new(sell_order: Order<T>, buy_order: Order<T>) -> Trade<T> {
        Trade {
            sell_order,
            buy_order
        }
    }

    pub fn sell_order(&self) -> &Order<T> {
        &self.sell_order
    }

    pub fn buy_order(&self) -> &Order<T> {
        &self.buy_order
    }
}

struct OrderQueue<T> where T: Tradeable + 'static {
    buy_queue: VecDeque<Order<T>>,
    sell_queue: VecDeque<Order<T>>
}

pub struct Orderbook<T> where T: Tradeable + 'static {
    t: T
}

impl<T> Orderbook<T> where T: Tradeable + 'static {
    pub fn t(&self) -> &T {
        &self.t
    }
}

pub struct Confirmation {

}