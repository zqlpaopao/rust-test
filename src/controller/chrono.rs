#![allow(unused)]

use chrono::{
    DateTime, Days, Duration, FixedOffset, Local, NaiveDateTime, Offset, TimeDelta, TimeZone, Utc,
};
use std::ops::Sub;
// https://mp.weixin.qq.com/s/wK8wQEnPLrzMNy2DP2UYuA
pub fn test_chrono() {
    //基本处理
    // basic_operations()

    //时区处理
    // timezone_handling()

    //时间计算和比较
    // time_calculations()

    //5. 自定义格式化
    custom_formatting()
}

///7. 实战应用示例
fn scheduler_example() {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    struct Task {
        execution_time: DateTime<Local>,
        name: String,
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.execution_time.cmp(&other.execution_time)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Task {
        fn eq(&self, other: &Self) -> bool {
            self.execution_time == other.execution_time
        }
    }

    impl Eq for Task {}

    struct Scheduler {
        tasks: BinaryHeap<Reverse<Task>>,
    }

    impl Scheduler {
        fn new() -> Self {
            Scheduler {
                tasks: BinaryHeap::new(),
            }
        }

        fn schedule(&mut self, task: Task) {
            self.tasks.push(Reverse(task));
        }

        fn next_task(&mut self) -> Option<Task> {
            if let Some(Reverse(task)) = self.tasks.pop() {
                if task.execution_time <= Local::now() {
                    Some(task)
                } else {
                    self.tasks.push(Reverse(task));
                    None
                }
            } else {
                None
            }
        }
    }
}

///6. 错误处理最佳实践
// 定义 TimeRange 结构体
#[derive(Debug)]
struct TimeRange {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

fn error_handling() {
    use chrono::ParseError;
    // 自定义错误类型
    #[derive(Debug)]
    enum TimeError {
        Parse(ParseError),
        InvalidRange,
        TimezoneError,
    }
    // 安全的时间解析
    fn safe_parse_datetime(input: &str) -> Result<DateTime<Local>, TimeError> {
        let parsed = DateTime::parse_from_rfc3339(input).map_err(TimeError::Parse)?;
        Ok(parsed.with_timezone(&Local))
    }
    // 带验证的时间范围创建
    fn create_time_range(
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> Result<TimeRange, TimeError> {
        if end <= start {
            return Err(TimeError::InvalidRange);
        }
        Ok(TimeRange { start, end })
    }
}

/// 5. 自定义格式化
fn custom_formatting() {
    use chrono::format::{Item, Numeric, Pad};

    // 自定义格式化项
    let format = vec![
        Item::Numeric(Numeric::Year, Pad::Zero),
        Item::Literal("年"),
        Item::Numeric(Numeric::Month, Pad::Zero),
        Item::Literal("月"),
        Item::Numeric(Numeric::Day, Pad::Zero),
        Item::Literal("日"),
    ];
    let now = Local::now();
    // 实现自定义Display trait
    struct CustomDateTime(DateTime<Local>);
    impl std::fmt::Display for CustomDateTime {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0.format("%Y年%m月%d日 %H时%M分%S秒"))
        }
    }
    println!("{}", CustomDateTime(now));
    //2024年12月09日 15时21分23秒
}

///4、性能优化技巧
fn performance_tips() {
    // 使用缓存的时区信息
    thread_local! {
        static CACHED_TZ: chrono_tz::Tz = chrono_tz::Asia::Shanghai;
    }

    // 高效的时间比较
    fn efficient_time_comparison() {
        let now = Local::now();
        let timestamp = now.timestamp();

        // 使用时间戳比较而不是DateTime比较
        if timestamp > 1640995200 {
            // 2022-01-01 00:00:00
            println!("在2022年之后");
        }
    }

    // 批量时间处理
    fn batch_process_dates(dates: Vec<DateTime<Utc>>) -> Vec<DateTime<Local>> {
        let local_tz = Local::now().timezone();
        dates
            .into_iter()
            .map(|dt| dt.with_timezone(&local_tz))
            .collect()
    }
}

/// 3、时间计算和比较
use chrono::{Datelike, Weekday};
fn time_calculations() {
    use chrono::Duration;
    let now = Local::now();
    // 时间加减
    let tomorrow = now + Duration::days(1);
    println!("+1d {}", tomorrow.format("%Y-%m-%d"));
    let last_week = now - Duration::weeks(1);
    println!("-1week {}", last_week);

    // 时间间隔计算
    let duration = tomorrow - now;
    println!("相差秒数: {}", duration.num_seconds());
    // 复杂的时间计算
    fn next_business_day(dt: DateTime<Local>) -> DateTime<Local> {
        let mut next_day = dt + Duration::days(1);
        while next_day.weekday() == Weekday::Sat || next_day.weekday() == Weekday::Sun {
            next_day = next_day + Duration::days(1);
        }
        next_day
    }
    // 自定义时间比较器
    #[derive(Debug)]
    struct TimeRange {
        start: DateTime<Local>,
        end: DateTime<Local>,
    }
    impl TimeRange {
        fn contains(&self, dt: DateTime<Local>) -> bool {
            dt >= self.start && dt <= self.end
        }
        fn overlaps(&self, other: &TimeRange) -> bool {
            self.start <= other.end && self.end >= other.start
        }
    }
    //+1d 2024-12-10
    // -1week 2024-12-02 15:18:35.580906 +08:00
    // 相差秒数: 86400
}

///2、时区处理进阶
fn timezone_handling() {
    use chrono_tz::Tz;
    // 使用固定时区
    let pacific_time = Local::now().with_timezone(&chrono_tz::America::Los_Angeles);

    println!("使用固定时区:{}", pacific_time);
    // 时区转换
    let utc_time = Utc::now();
    println!("时区转换:{}", utc_time);

    let shanghai_time = utc_time.with_timezone(&chrono_tz::Asia::Shanghai);
    // 处理夏令时
    let ny_time = utc_time.with_timezone(&chrono_tz::America::New_York);
    println!(
        "是否是夏令时: {}",
        ny_time.offset().fix().local_minus_utc() == 4 * 3600
    );
    // 安全的时区转换函数
    fn safe_timezone_convert<T: TimeZone>(dt: DateTime<T>, target_tz: Tz) -> DateTime<Tz> {
        dt.with_timezone(&target_tz)
    }

    //使用固定时区:2024-12-08 23:14:55.466503 PST
    // 时区转换:2024-12-09 07:14:55.468486 UTC
    // 是否是夏令时: false
}

///1、基础时间操作
fn basic_operations() {
    // 获取当前时间
    let now = Local::now();
    let utc_now = Utc::now();

    // 创建特定时间
    let date = Local.with_ymd_and_hms(2024, 3, 14, 0, 0, 0);
    let datetime = Local.with_ymd_and_hms(2024, 3, 14, 15, 30, 0);

    // 格式化输出
    println!("ISO 8601: {}", now.to_rfc3339());
    println!("自定义格式: {}", now.format("%Y-%m-%d %H:%M:%S"));

    // 时间戳转换
    let timestamp = now.timestamp();
    let datetime_from_timestamp = Local.timestamp_opt(timestamp, 0);

    //ISO 8601: 2024-12-09T15:10:43.197977+08:00
    // 自定义格式: 2024-12-09 15:10:43
}

fn parse_data_time_from_str(data_str: &str, fmt: &str) {
    // 解析 NaiveDateTime
    let naive_utc = NaiveDateTime::parse_from_str(data_str, fmt).unwrap();
    println!("naive_utc date time {}", naive_utc);

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
