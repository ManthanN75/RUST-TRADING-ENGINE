use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid, // Represents a buy order
    Ask  // Represents a sell order
}   

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    fn add_order(&mut self, order: Order, price: f64) {
        let price_struct = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match(self.bids.get_mut(&price)){
                    Some(limit) => println!("already got a limit"),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Ask => {}
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy )]
struct Price{
    integral: u64,
    fractional: u64,
    scalar: u64
}

impl Price{
    fn new(price: f64) -> Price{
        let integral = price as u64;
        let scalar = 100000;
        let fractional = ((price % 1.0) * scalar as f64) as u64;

        Price{integral, fractional, scalar}
    }
}
#[derive(Debug)]
struct Limit{
    price: Price,
    orders: Vec<Order>
}


impl Limit{
    fn new(price: Price) -> Limit{
        Limit{
            price,
            orders: Vec::new()
        }
    }
    fn add_order(&mut self, order: Order){
        self.orders.push(order);
    }

}
#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order{
    fn new(bid_or_ask: BidOrAsk, size:f64) -> Order{
        Order{size,bid_or_ask}
    }
}

fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let sell_order = Order::new(BidOrAsk::Ask, 2.45);
    let sell_order_from_bob = Order::new(BidOrAsk::Ask, 1.0);

    let mut order_book = OrderBook::new();
    order_book.add_order(buy_order, 4.4);
    println!("{:?}" , order_book);
}
