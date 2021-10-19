pub mod orderutils {
    use crate::core::assets::Tradeable;
    use crate::accounting::marketdata::Price;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::Zero;

    #[derive(Clone, PartialEq, Eq, Hash)]
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
            self.underlying
        }

        pub fn amount(&self) -> f64 {
            self.amount
        }

        pub fn is_fully_executed(&self) -> bool {
            self.is_fully_executed
        }

        pub fn is_sell_order(&self) -> bool {
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
            if l.value() <= Decimal::zero() {
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
    use crate::messaging::Response;

    struct OrderQueue<T> where T: Tradeable + 'static {
        buy_queue: VecDeque<Order<T>>,
        sell_queue: VecDeque<Order<T>>
    }

    impl<T> OrderQueue<T> where T: Tradeable + 'static {
        pub fn new() -> OrderQueue<T> {
            OrderQueue {
                buy_queue: VecDeque::new(),
                sell_queue: VecDeque::new()
            }
        }

        pub fn buy_queue(&mut self) -> &mut VecDeque<Order<T>> {
            &mut self.buy_queue
        }

        pub fn sell_queue(&mut self) -> &mut VecDeque<Order<T>> {
            &mut self.sell_queue
        }
    }

    pub struct Orderbook<T> where T: Tradeable + 'static {
        underlying: &'static T,
        order_queue_map: HashMap<OrderType, OrderQueue<T>>
    }

    impl<T> Orderbook<T> where T: Tradeable + 'static {
        pub fn underlying(&self) -> &T {
            &self.underlying
        }

        pub fn place_order(&mut self, order: Order<T>) -> Response {
            // the order queue determination can be done automatically
            // with a hashmap lookup, therefore it must no be done manually.
            let order_queue = self.order_queue_map.get_mut(order.order_type());

            // determine order queue (sell or buy)
            match order_queue {
                Some(oq) => {
                    if order.is_sell_order() {
                        oq.sell_queue().push_front(order);
                    } else {
                        oq.buy_queue().push_front(order);
                    }
                },
                None => {
                    // new order queue entry if there isn't any
                    let mut new_oq = OrderQueue::new();
                    let order_type_clone = order.order_type().clone();
                    new_oq.sell_queue.push_back(order);
                    self.order_queue_map.insert(order_type_clone, new_oq);
                }
            }

            // trigger matching
            Response::NoOrder
        }
    }
}

pub mod placement {
    use std::collections::{VecDeque, HashMap};
    use crate::orderutils::Order;
    use crate::assets::Tradeable;
    use crate::orderbook::Orderbook;
    use std::hash::Hash;
    use crate::messaging::Response;

    pub struct Placer<T> where T: Tradeable + 'static + Eq + Hash {
        placer_queue: VecDeque<Order<T>>,
        orderbooks: HashMap<T, Orderbook<T>>
    }

    impl<T> Placer<T> where T: Tradeable + 'static + Eq + Hash {
        pub fn push_to_queue(&mut self, order: Order<T>) {
            self.placer_queue.push_back(order);
        }

        fn place_next(&mut self) -> Result<Response, Response> {
            let next_order = self.placer_queue.pop_front();
            if let Some(order) = next_order {
                if let Some(orderbook) = self.orderbooks.get_mut(order.underlying()) {
                    orderbook.place_order(order);
                    // TODO: Implement auto-assignment of order id
                    let order_id = 1;
                    return Ok(Response::Placement(order_id));
                } else {
                    return Err(Response::Rejection(String::from("Orderbook could not been determined.")));
                }
            }
            Err(Response::NoOrder)
        }
    }
 }
