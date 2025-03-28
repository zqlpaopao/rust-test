// 观察者模式：实时市场数据推送
// 观察者模式非常适合处理市场数据推送场景，让多个交易策略能够订阅并响应市场数据的变化。

use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, instrument_id: &str, price: f64);
}

//主题特征
trait Subject {
    fn attach(&mut self, observer: Rc<dyn Observer>);
    fn detach(&mut self, observer: Rc<dyn Observer>);
    fn notify(&self);
}

//动量策略
struct MomentumStrategy {
    name: String,
    threshold: f64,
}

impl Observer for MomentumStrategy {
    fn update(&self, instrument_id: &str, price: f64) {
        if price > self.threshold {
            println!(
                "{}:[{}] 价格突破阀值！当前价格：{}",
                self.name, instrument_id, price
            )
        }
    }
}

// 均值回归策略
struct MeanReversionStrategy {
    name: String,
    average_price: RefCell<f64>,
}

impl Observer for MeanReversionStrategy {
    fn update(&self, instrument_id: &str, price: f64) {
        let mut avg = self.average_price.borrow_mut();
        *avg = (*avg * 0.9) + (price * 0.1);

        if price < *avg {
            println!(
                "{}: [{}] 价格低于均值！价格：{}，均值：{:.2}",
                self.name, instrument_id, price, *avg
            );
            //实现买入逻辑
        }
    }
}

//市场数据源
struct MarketDataFeed {
    instrument_id: String,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
    price: RefCell<f64>,
}

impl MarketDataFeed {
    fn new(instrument_id: &str) -> Self {
        MarketDataFeed {
            instrument_id: instrument_id.to_string(),
            observers: RefCell::new(vec![]),
            price: RefCell::new(0.0),
        }
    }
    fn set_price(&mut self, price: f64) {
        *self.price.borrow_mut() = price;
        self.notify();
    }
}

impl Subject for MarketDataFeed {
    fn attach(&mut self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn detach(&mut self, observer: Rc<dyn Observer>) {
        let mut observers = self.observers.borrow_mut();
        if let Some(pos) = observers.iter().position(|x| Rc::ptr_eq(x, &observer)) {
            observers.remove(pos);
        }
    }

    fn notify(&self) {
        let price = *self.price.borrow();
        for observer in self.observers.borrow_mut().iter() {
            observer.update(&self.instrument_id, price);
        }
    }
}

pub fn test() {
    //create market data for AAPL
    let mut market_data_feed = MarketDataFeed::new("AAPL");

    //create observers 动量策略
    let momentum_strategy: Rc<dyn Observer> = Rc::new(MomentumStrategy {
        name: String::from("MomentumStrategy"),
        threshold: 150.0,
    });

    //均值回归
    let mean_reversion_strategy: Rc<dyn Observer> = Rc::new(MeanReversionStrategy {
        name: "MeanReversionStrategy".to_string(),
        average_price: RefCell::new(145.0),
    });

    //Attach observers
    market_data_feed.attach(momentum_strategy.clone());
    market_data_feed.attach(mean_reversion_strategy);

    //simulate market data updates
    let price_updates = vec![148.0, 151.0, 149.5, 152.5, 147.0];

    for price in price_updates {
        println!("\nMarketDataFeed [{}]: New price is {}", "AAPL", price);
        market_data_feed.set_price(price);
    }

    //detach momentum strategy
    market_data_feed.detach(momentum_strategy);

    //more updates
    let more_price_updates = vec![153.0, 146.5];

    for price in more_price_updates {
        println!("\nMarketDataFeed [{}]: New price is {}", "AAPL", price);
        market_data_feed.set_price(price);
    }
}

// MarketDataFeed [AAPL]: New price is 148
//
// MarketDataFeed [AAPL]: New price is 151
// MomentumStrategy:[AAPL] 价格突破阀值！当前价格：151
//
// MarketDataFeed [AAPL]: New price is 149.5
//
// MarketDataFeed [AAPL]: New price is 152.5
// MomentumStrategy:[AAPL] 价格突破阀值！当前价格：152.5
//
// MarketDataFeed [AAPL]: New price is 147
//
// MarketDataFeed [AAPL]: New price is 153
//
// MarketDataFeed [AAPL]: New price is 146.5
// MeanReversionStrategy: [AAPL] 价格低于均值！价格：146.5，均值：147.39
