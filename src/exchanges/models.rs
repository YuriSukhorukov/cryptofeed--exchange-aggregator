pub struct OrderBook {}
pub struct Ticker {}
pub struct Trade {
    symbol: Option<String>,
    timestamp: Option<u64>,
    volume: Option<u64>,
    price: Option<u64>,
    direction: Option<Direction>,
    id: Option<String>,
}
pub enum Direction {
    PlusTick,
    ZeroPlusTick,
    MinusTick,
    ZeroMinusTick,
}
