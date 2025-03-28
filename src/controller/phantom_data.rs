// https://mp.weixin.qq.com/s/tWKpc1b6VOIacbAxFkLURg

use anyhow::ensure;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum UOM {
    Piece,
    Litre,
}

#[derive(Debug)]
pub struct BilledLineQuantity {
    pub quantity: f32,
    pub unit_of_measure: UOM,
}

impl BilledLineQuantity {
    pub fn new(quantity: f32, uom: UOM) -> anyhow::Result<Self> {
        ensure!(quantity > 0_f32, "quantity cannot be negative");
        Ok(Self {
            quantity,
            unit_of_measure: uom,
        })
    }
}

pub struct FreeLineQuantity {
    pub quantity: f32,
    pub unit_of_measure: UOM,
}

impl FreeLineQuantity {
    pub fn new(quantity: f32, uom: UOM) -> anyhow::Result<Self> {
        ensure!(quantity >= 0_f32, "quantity cannot be zero or negative");
        Ok(Self {
            quantity,
            unit_of_measure: uom,
        })
    }
}

//PhantomData 用于那些需要在编译时知道泛型参数的存在，但在运行时不需要实际类型信息的场景。

#[derive(Debug)]
pub struct Any<T> {
    pub quantity: f32,
    pub unit_of_measure: UOM,
    _phantom: PhantomData<T>,
}

impl<T> Any<T> {
    pub fn create_type(
        quantity: f32,
        uom: UOM,
        f: Box<dyn Fn(f32) -> anyhow::Result<()>>,
    ) -> anyhow::Result<Self> {
        f(quantity)?;
        Ok(Self {
            quantity,
            unit_of_measure: uom,
            _phantom: PhantomData,
        })
    }
}
#[derive(Debug)]
pub struct BilledLineQuantity1 {}
#[derive(Debug)]
pub struct FreeLineQuantity1 {}

pub fn test_phan_tom_data() {
    let b = BilledLineQuantity::new(-12_f32, UOM::Litre);
    println!("{:#?}", b);

    let b = Any::<BilledLineQuantity1>::create_type(
        -12_f32,
        UOM::Litre,
        Box::new(|quantity: f32| {
            //  这里的问题是由于ensure!宏的使用方式造成的。ensure!当条件失败时，
            // 宏返回封装在类型Err中的所提供的错误消息anyhow::Error。当条件为 true 时，
            // 它不会返回，Ok(())而是什么也不返回 ( ())，这就是与?运算符混淆的地方。
            // 要解决此问题，您不需要在宏?后面直接使用运算符，ensure!因为如果不满足条件，ensure!则会自动返回。
            // Err如果满足条件，则继续执行下一行。
            // ensure!(quantity >= 0_f32, "quantity cannot be zero or negative")?; //错误
            ensure!(quantity >= 0_f32, "quantity cannot be zero or negative");
            Ok(())
        }),
    );
    println!("{:#?}", b);

    let f = Any::<FreeLineQuantity1>::create_type(
        14f32,
        UOM::Litre,
        Box::new(|quantity: f32| {
            //  这里的问题是由于ensure!宏的使用方式造成的。ensure!当条件失败时，
            // 宏返回封装在类型Err中的所提供的错误消息anyhow::Error。当条件为 true 时，
            // 它不会返回，Ok(())而是什么也不返回 ( ())，这就是与?运算符混淆的地方。
            // 要解决此问题，您不需要在宏?后面直接使用运算符，ensure!因为如果不满足条件，ensure!则会自动返回。
            // Err如果满足条件，则继续执行下一行。
            // ensure!(quantity >= 0_f32, "quantity cannot be zero or negative")?; //错误
            ensure!(quantity >= 0_f32, "quantity cannot be zero or negative");
            Ok(())
        }),
    );
    println!("{:#?}", f)
}
