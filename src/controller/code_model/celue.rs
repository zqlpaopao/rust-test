/// https://mp.weixin.qq.com/s/LktWfKfSU5-CgjRFeTh9iw

/// 策略模式

trait ExecutionStrategy {
    fn execution_order(&self, order_id: u32, quantity: u32);
}

//实现 TWAP策略
struct TWapStrategy;
impl ExecutionStrategy for TWapStrategy {
    fn execution_order(&self, order_id: u32, quantity: u32) {
        println!("使用 TWap 策略执行订单 {},数量：{}", order_id, quantity);
    }
}

//实现VWAP
struct VWapStrategy;
impl ExecutionStrategy for VWapStrategy {
    fn execution_order(&self, order_id: u32, quantity: u32) {
        println!("使用 VWap 策略执行订单 {},数量：{}", order_id, quantity);
    }
}

//实现POV 策略
struct PovStrategy {
    participation_rate: f64,
}

impl ExecutionStrategy for PovStrategy {
    fn execution_order(&self, order_id: u32, quantity: u32) {
        println!(
            "使用 POV 策略执行订单 {}，参与率 {}%，数量：{}",
            order_id,
            self.participation_rate * 100.0,
            quantity
        );
    }
}

/// 订单执行器
struct OrderExecutor {
    strategy: Box<dyn ExecutionStrategy>,
}

impl OrderExecutor {
    fn new(strategy: Box<dyn ExecutionStrategy>) -> OrderExecutor {
        OrderExecutor { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn ExecutionStrategy>) {
        self.strategy = strategy;
    }

    fn execution(&self, order_id: u32, quantity: u32) {
        self.strategy.execution_order(order_id, quantity);
    }
}

pub fn test() {
    let order_id = 101;
    let quantity = 1000;
    // using TWap strategy
    let twap_strategy = Box::new(TWapStrategy);
    let mut executor = OrderExecutor::new(twap_strategy);
    executor.execution(order_id, quantity);

    //switching to vWap strategy
    let vwap_strategy = Box::new(VWapStrategy);
    executor.set_strategy(vwap_strategy);
    executor.execution(order_id + 1, quantity);

    let pov_strategy = Box::new(PovStrategy {
        participation_rate: 0.1,
    });
    executor.set_strategy(pov_strategy);
    executor.execution(order_id + 2, quantity);
}

// 使用 TWap 策略执行订单 101,数量：1000
// 使用 VWap 策略执行订单 102,数量：1000
// 使用 POV 策略执行订单 103，参与率 10%，数量：1000
