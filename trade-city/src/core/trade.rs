use crate::orderutils::Order;
use crate::core::assets::Tradeable;
use crate::marketdata::Price;

pub struct Execution<T> where T: Tradeable + 'static {
    sell_order: Order<T>,
    buy_order: Order<T>,
    price: Price
}

impl<T> Execution<T> where T: Tradeable + 'static {
    pub fn new(sell_order: Order<T>, buy_order: Order<T>, price: Price) -> Execution<T> {
        Execution {
            sell_order,
            buy_order,
            price
        }
    }

    pub fn sell_order(&self) -> &Order<T> {
        &self.sell_order
    }

    pub fn buy_order(&self) -> &Order<T> {
        &self.buy_order
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}
