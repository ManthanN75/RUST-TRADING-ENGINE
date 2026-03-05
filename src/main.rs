enum BidOrAsk {
    Bid(Order),
    Ask(Order)
}   

struct Price{
    integral
}

struct Limit{
    price: Price,
    orders: Vec<Order>
}
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
    println!("Hello, world!");
}
