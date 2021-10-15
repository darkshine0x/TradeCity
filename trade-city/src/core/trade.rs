use crate::orderutils::Order;
use crate::core::assets::Tradeable;

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

pub enum Response {
    Placement,
    Execution,
    Cancellation,
    Rejection
}
