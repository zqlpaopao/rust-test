https://mp.weixin.qq.com/s/UE8kg9YQQaYqOQnfvgczyg

# 1、安装

```
[dependencies]
chrono = "0.4"
```



# 2、基本时间类型

Chrono 提供了几个基本类型来表示时间：

- `DateTime<Utc>`: UTC（协调世界时）时间日期。
- `DateTime<Local>`: 本地时区时间日期。
- `NaiveDateTime`: 无时区的时间日期。
- `Date<Utc>`: 只有日期的 UTC 时间。
- `Date<Local>`: 只有日期的本地时区时间。
- `NaiveDate`: 无时区的日期。
- `Time<Utc>`: 只有时间的 UTC 时间。
- `Time<Local>`: 只有时间的本地时区。
- `NaiveTime`: 无时区的时间。



# 3、获取当前时间

```
//获取当前时间
let now_utc : DateTime<Utc> = Utc::now();
println!("utc now {}",now_utc);

//获取local time
let now_local :DateTime<Local> = Local::now();
println!("local now {}",now_local);

utc now 2024-05-20 03:52:37.243454 UTC
local now 2024-05-20 11:52:37.243511 +08:00
```



# 4、解析日期时间函数

```

parse_data_time_from_str("2024-05-20 12:00:00", "%Y-%m-%d %H:%M:%S")


fn parse_data_time_from_str(data_str : &str,fmt:&str){

    // 解析 NaiveDateTime
    let naive_utc = NaiveDateTime::parse_from_str(data_str, fmt).unwrap();
    println!("naive_utc date time {}",naive_utc);

    // 创建一个时区偏移量，比如东八区（北京时间）
    let offset = FixedOffset::east_opt(8 * 3600).unwrap(); // 东八区

    // 使用偏移量和 UTC NaiveDateTime 创建 DateTime
    let datetime_with_offset = offset.from_utc_datetime(&naive_utc);

    println!("DateTime with offset: {}", datetime_with_offset);

    // 直接从 UTC NaiveDateTime 创建 DateTime<Utc>
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&naive_utc);

    println!("DateTime in UTC: {}", datetime_utc);

    // 若要转换为本地时间（Local），可以使用 Local 时区
    let datetime_local: DateTime<Local> = Local.from_utc_datetime(&naive_utc);

    println!("DateTime in Local: {}", datetime_local);
}


naive_utc date time 2024-05-20 12:00:00
DateTime with offset: 2024-05-20 20:00:00 +08:00
DateTime in UTC: 2024-05-20 12:00:00 UTC
DateTime in Local: 2024-05-20 20:00:00 +08:00

```



# 5、时间计算比较

Chrono 允许你对日期和时间进行前后计算。你可以使用 `checked_add_signed` 和 `checked_sub_signed` 方法来计算时间的加减。

```
use chrono::{DateTime, Utc, NaiveDateTime, FixedOffset, Local, TimeZone, Duration,TimeDelta,Days};

pub fn test_chrono(){
    //获取当前时间
    // let now_utc : DateTime<Utc> = Utc::now();
    // println!("utc now {}",now_utc);

    //获取local time
    // let now_local :DateTime<Local> = Local::now();
    // println!("local now {}",now_local);

    // parse_data_time_from_str("2024-05-20 12:00:00", "%Y-%m-%d %H:%M:%S")


    //时间比较
    let now = Utc::now();
    let after_ten_day = now.checked_add_signed(TimeDelta::try_days(10).unwrap()).unwrap();
    let before_ten_day = now.checked_sub_signed(TimeDelta::try_days(10).unwrap()).unwrap();
    let before_ten_day1 = now.checked_sub_days(Days::new(10)).unwrap();
    println!("Utc 十天后的时间：{}", after_ten_day);
    println!("Utc 十天前的时间：{}", before_ten_day);
    println!("Utc days 十天前的时间：{}", before_ten_day1);

    let now = Local::now();
    let after_ten_day = now.checked_add_signed(TimeDelta::try_days(10).unwrap()).unwrap();
    let before_ten_day = now.checked_sub_signed(TimeDelta::try_days(10).unwrap()).unwrap();
    let before_ten_day1 = now.checked_sub_days(Days::new(10)).unwrap();

    println!("Local 十天后的时间：{}", after_ten_day);
    println!("Local  days十天前的时间：{}", before_ten_day);
    println!("Local days 十天前的时间：{}", before_ten_day1);
    
 		let now_hour = now.sub(TimeDelta::new(3600, 0).unwrap());
    println!("Local 一小时前：{}", now_hour);

}
Utc 十天后的时间：2024-05-30 06:23:43.433025 UTC
Utc 十天前的时间：2024-05-10 06:23:43.433025 UTC
Utc days 十天前的时间：2024-05-10 06:23:43.433025 UTC
Local 十天后的时间：2024-05-30 14:23:43.433260 +08:00
Local  days十天前的时间：2024-05-10 14:23:43.433260 +08:00
Local days 十天前的时间：2024-05-10 14:23:43.433260 +08:00
Local 一小时前：2024-05-20 13:28:03.971365 +08:00
```

有



同时，Chrono 支持使用 `>`、`<` 和 `==` 操作符来比较时间：

```
let now = Utc::now();
let earlier = now - chrono::Duration::try_seconds(5).unwrap();

println!("现在是：{}", now);
println!("五秒前是：{}", earlier);

现在是：2024-05-20 06:30:24.513713 UTC
五秒前是：2024-05-20 06:30:19.513713 UTC
```





# 6、格式化显示

Chrono 提供了格式化时间日期的功能。你可以利用 `format` 方法输出不同格式的日期时间字符串：

```
//格式化显示
let now = Local::now();
println!("现在的时间是：{}", now.format("%Y年%m月%d日 %H:%M:%S"));

现在的时间是：2024年05月20日 14:32:38
```



# 7、时区处理

Chrono 支持时区转换。你可以使用 `with_timezone` 方法将 UTC 时间转换为任何指定的时区时间：

```
//时区转换
let utc_now: DateTime<Utc> = Utc::now();
let shanghai = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
let shanghai_now = utc_now.with_timezone(&shanghai);
println!("上海时间：{}", shanghai_now);

上海时间：2024-05-20 14:34:49.465576 +08:00
```







































