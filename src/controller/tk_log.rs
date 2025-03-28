#![allow(unused)]
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tklog::{debugs, errors, fatals, infos, sync::Logger, traces, warns, Format, LEVEL, LOG, MODE};
pub fn log_init() {
    let mut log = Logger::new();
    log.set_console(true) //是否在前台显示
        .set_level(LEVEL::Trace) //设置日志级别
        .set_cutmode_by_time("tkLogs.log", MODE::DAY, 10, true) //备份文件 多大备份，部分数量 是否压缩
        // .set_cutmode_by_time()//备份时间
        .set_formatter("{level}{time} {file}:{message}\n"); //日志格式
                                                            // .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName);//日志格式

    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();
    let r = vec![1, 2, 3, 4];
    traces!(
        log,
        "traces>>>>{} AAAAAAAAA {} {} {} {:?}",
        "Some string",
        1,
        2,
        3
    );
    debugs!(log, "debugs>>>>", "BBBBBBBBB", 1, 2, 3, 5);
    debugs!(log, "debugs>>>>", "BBBBBBBBB", 1, 2, 3, 5);
    infos!(log, "infos>>>>", "CCCCCCCCC", 1, 2, 3, 5);
    warns!(log, "warns>>>>", "DDDDDDDDDD", 1, 2, 3, 6);
    errors!(log, "errors>>>>", "EEEEEEEE", 1, 2, 3, 7);
    fatals!(log, "fatals>>>>", "FFFFFFFF", 1, 2, 3, 8);
    // tklog::formats!(log, LEVEL::Debug, "Debug>>>{},{}>>>{:?}", 1, 2, v);
    thread::sleep(Duration::from_secs(1))

    // .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName);//日志格式
    //[TRACE] 19:09:45 tk_log.rs 25:traces>>>>,AAAAAAAAA,1,2,3,4
    // [DEBUG] 19:09:45 tk_log.rs 26:debugs>>>>,BBBBBBBBB,1,2,3,5
    // [INFO] 19:09:45 tk_log.rs 27:infos>>>>,CCCCCCCCC,1,2,3,5
    // [WARN] 19:09:45 tk_log.rs 28:warns>>>>,DDDDDDDDDD,1,2,3,6
    // [ERROR] 19:09:45 tk_log.rs 29:errors>>>>,EEEEEEEE,1,2,3,7
    // [FATAL] 19:09:45 tk_log.rs 30:fatals>>>>,FFFFFFFF,1,2,3,8

    // .set_formatter("{level}{time} {file}:{message}\n");//日志格式
    //[TRACE] 2024-06-05 19:10:49 tk_log.rs 25:traces>>>>,AAAAAAAAA,1,2,3,4
    // [DEBUG] 2024-06-05 19:10:49 tk_log.rs 26:debugs>>>>,BBBBBBBBB,1,2,3,5
    // [INFO] 2024-06-05 19:10:49 tk_log.rs 27:infos>>>>,CCCCCCCCC,1,2,3,5
    // [WARN] 2024-06-05 19:10:49 tk_log.rs 28:warns>>>>,DDDDDDDDDD,1,2,3,6
    // [ERROR] 2024-06-05 19:10:49 tk_log.rs 29:errors>>>>,EEEEEEEE,1,2,3,7
    // [FATAL] 2024-06-05 19:10:49 tk_log.rs 30:fatals>>>>,FFFFFFFF,1,2,3,8
}
