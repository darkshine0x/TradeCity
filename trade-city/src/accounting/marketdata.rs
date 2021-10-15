use chrono::{Date, Local, DateTime};
use crate::core::assets::Tradeable;

pub struct Currency {
    pub iso_code: String,
    pub name: String
}

#[derive(Copy, Clone)]
pub struct Price {
    currency: &'static Currency,
    value: f64
}

impl Price {
    pub fn new(currency: &'static Currency, value: f64) -> Price {
        Price {
            currency,
            value
        }
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub struct MarketDataPrice<T> where T: Tradeable + 'static {
    date: Date<Local>,
    underlying: &'static T,
    price: Price,
    insertion_date: DateTime<Local>
}

impl<T> MarketDataPrice<T> where T: Tradeable + 'static {
    pub fn new(date: Option<Date<Local>>, underlying: &'static T, price: Price) -> MarketDataPrice<T> {
        MarketDataPrice {
            date: match date {
                Some(d) => d,
                None => Local::today()
            },
            underlying,
            price,
            insertion_date: Local::now()
        }
    }

    pub fn date(&self) -> &Date<Local> {
        &self.date
    }

    pub fn underlying(&self) -> &T {
        &self.underlying
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn insertion_date(&self) -> &DateTime<Local> {
        &self.insertion_date
    }
}

