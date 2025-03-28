use fast_log::appender::{FastLogRecord, LogAppender};
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use fast_log::Config;
use log::info;
use std::thread;

/// https://github.com/rbatis/fast_log

/// 自定义实现log
pub struct CustomLog {}
impl LogAppender for CustomLog {
    fn do_logs(&self, records: &[FastLogRecord]) {
        println!("{:?}", records)
    }
}

pub fn test_fast_log() {
    fast_log::init(
        Config::new()
            .file("/Users/zhangqiuli24/Desktop/Jd-work/topology_power/log.logg")
            //预分配 chan 的长度
            .chan_len(Some(10))
            //使用控制台
            .console()
            //自定义日志
            // .custom(CustomLog{})
            // .format()//
            //分割文件
            .file_split(
                //分割路径
                "./target/logs/",
                // B KB MB GB TB  EB
                LogSize::KB(3),
                // ALL 保留全部的日志
                // KeepTime 保留日志的时间
                // KeepNum  保留文件的数量
                // RollingType::KeepTime(std::time::Duration::from_secs(60)),
                RollingType::KeepNum(2),
                LogPacker {},
            ),
    )
    .unwrap();

    for _ in 0..200 {
        info!("Commencing yak shaving");
    }
    log::logger().flush();
    thread::sleep(std::time::Duration::from_secs(200))
}
