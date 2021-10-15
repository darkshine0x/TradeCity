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

        pub fn is_fully_executed(&self) -> bool {
            self.is_fully_executed
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
    use crate::orderutils::Order;
    use crate::trade::Response;

    struct OrderQueue<T> where T: Tradeable + 'static {
        pub buy_queue: VecDeque<Order<T>>,
        pub sell_queue: VecDeque<Order<T>>
    }
    
    pub struct Orderbook<T> where T: Tradeable + 'static {
        underlying: &'static T,
        order_queue_map: HashMap<f64, OrderQueue<T>>
    }

    impl<T> Orderbook<T> {
        pub fn underlying(&self) -> &T {
            &self.underlying
        }

        pub fn place_order(order: Order<T>) -> Response {
            // trigger matching
            Response::Placement
        }
    }
}
