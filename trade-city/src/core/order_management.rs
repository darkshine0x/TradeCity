pub mod orderutils {
    use crate::core::assets::Tradeable;
    use crate::accounting::marketdata::Price;

    #[derive(Copy, Clone)]
    pub enum OrderType {
        AtMarket,
        Limit(Price)
    }

    pub struct Order<T> where T: Tradeable + 'static {
        underlying: &'static T,
        amount: f64,
        is_fully_executed: bool,
        is_sell_order: bool,
        order_type: OrderType
    }

    impl<T> Order<T> where T: Tradeable + 'static  {
        pub fn new(underlying: &'static T, amount: f64, order_type: OrderType) -> Result<Order<T>, String> {
            let validation_result = validate_order(amount, &order_type);

            if let Err(message) = validation_result {
                return Err(message);
            }

            // Order is valid

            let new_order = Order {
                underlying,
                amount,
                order_type,
                is_fully_executed: false,
                is_sell_order: amount < 0.0
            };

            Ok(new_order)
        }

        pub fn underlying(&self) -> &T {
            &self.underlying
        }

        pub fn amount(self) -> f64 {
            self.amount
        }

        pub fn is_fully_executed(self) -> bool {
            self.is_fully_executed
        }

        pub fn is_sell_order(self) -> bool {
            self.is_sell_order
        }

        pub fn order_type(&self) -> &OrderType {
            &self.order_type
        }
    }

    fn validate_order(amount: f64, order_type: &OrderType) -> Result<bool, String> {
        // amount validation
        if amount <= 0.0 {
            return Err(String::from("Amount must be greater than zero."));
        }

        // limit order validation
        if let OrderType::Limit(l) = order_type {
            if l.value() <= 0.0 {
                return Err(String::from("Limit must be greater than zero."))
            }
        }

        return Ok(true);
    }
}

pub mod orderbook {
    use crate::assets::Tradeable;
    use std::collections::{VecDeque, HashMap};
    use crate::orderutils::{Order, OrderType};
    use crate::trade::Response;

    struct OrderQueue<T> where T: Tradeable + 'static {
        pub buy_queue: VecDeque<Order<T>>,
        pub sell_queue: VecDeque<Order<T>>
    }
    
    pub struct Orderbook<T> where T: Tradeable + 'static {
        underlying: &'static T,
        order_queue_map: HashMap<f64, OrderQueue<T>>
    }

    impl<T> Orderbook<T> where T: Tradeable + 'static {
        pub fn underlying(&self) -> &T {
            &self.underlying
        }

        pub fn place_order(&self, order: Order<T>) -> Response {
            // determine order queue
            match order.order_type() {
                OrderType::AtMarket => {
                    // TODO: Adjust key type so that an "at market" key is possible
                },
                OrderType::Limit(price) => {

                }
            }

            // trigger matching
            Response::Placement
        }
    }
}

pub mod placement {
    use std::collections::{VecDeque, HashMap};
    use crate::orderutils::Order;
    use crate::assets::Tradeable;
    use crate::orderbook::Orderbook;
    use std::hash::Hash;

    pub struct Placer<T> where T: Tradeable + 'static + Eq + Hash {
        placer_queue: VecDeque<Order<T>>,
        orderbooks: HashMap<T, Orderbook<T>>
    }

    impl<T> Placer<T> where T: Tradeable + 'static + Eq + Hash {
        pub fn push_to_queue(&mut self, order: Order<T>) {
            self.placer_queue.push_back(order);
        }

        fn place(&mut self) {
            let next_order = self.placer_queue.pop_front();
            if let Some(order) = next_order {
                if let Some(orderbook) = self.orderbooks.get(order.underlying()) {
                    orderbook.place_order(order);
                }
            }
        }
    }
 }
