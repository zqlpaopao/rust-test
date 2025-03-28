#![allow(unused)]

use flexi_logger::Duplicate;
use flexi_logger::Logger;
use flexi_logger::{colored_opt_format, Age, Cleanup, Criterion, Naming};
use flexi_logger::{FileSpec, WriteMode};

pub async fn log() {
    let _logger = Logger::try_with_str("debug, my::critical::module=trace")
        .unwrap()
        // .format_for_files(json_format)
        //https://docs.rs/flexi_logger/0.28.0/flexi_logger/struct.Logger.html#method.start_with_specfile
        // .start_with_specfile("./server/config/logspec.toml")//
        // .log_to_stderr() //日志写入标准输出。
        .log_to_file(
            FileSpec::default()
                .directory("./log_files") //文件路径
                .basename("foo") //文件名字
                .suppress_timestamp() //使用
                .suffix("bar") //文件后缀
                .discriminant("Sample4711A"),
        )
        .print_message()
        // .create_symlink("current_run")//并在 UNIX 上创建一个名为 的符号链接current_run。
        .create_symlink("link_to_log_file") //并在 UNIX 上创建一个名为 的符号链接current_run。
        //当写入文件或写入器时，您有时希望在终端上另外查看日志的某些部分；
        // 这可以通过Logger::duplicate_to_stderr或 来实现 Logger::duplicate_to_stdout，
        // 它将日志消息复制到终端。
        .duplicate_to_stderr(Duplicate::Debug)
        /*
           日志写入模式
           https://docs.rs/flexi_logger/0.28.0/flexi_logger/enum.WriteMode.html#variant.BufferAndFlush
           Direct ：不缓冲（默认）。日志行都直接写入输出，无需缓冲。这允许实时查看新的日志行，并且不需要额外的线程。
           SupportCapture ： 不缓冲和支持cargo test捕获。很像Direct，只是慢一点，并且允许 cargo test捕获日志输出并仅在失败的测试时打印它。
           BufferAndFlush ： BufferAndFlushWith与默认容量 ( DEFAULT_BUFFER_CAPACITY 8_192usize) 和默认间隔 ( )相同DEFAULT_FLUSH_INTERVAL 1s。
           BufferAndFlushWith : 元组 自定义 刷入的时间 和大小 ( usize ,Duration)
           BufferDontFlush : BufferDontFlushWith与默认容量相同( DEFAULT_BUFFER_CAPACITY)。
           BufferDontFlushWith :具有给定缓冲区容量的缓冲区，但不刷新。如果您想最大程度地减少 I/O 工作量并且不想创建额外的线程用于刷新并且不关心日志行是否出现延迟，这可能会很方便。
           Async
           AsyncWith :
           pool_capa: usize 消息缓冲区池的容量。
           message_capa: usize单个消息缓冲区的容量。
           flush_interval: Duration 刷新输出的时间间隔。随着Duration::ZERO冲洗被抑制。
        */
        .write_mode(WriteMode::BufferAndFlush)
        .format(colored_opt_format)
        // .format(json_format)
        .append() //将每次新运行的日志附加到现有文件中。//不加就会有时间戳
        .rotate(
            /*
               Criterion::Age当时钟切换到新的一天、小时、分钟或秒时，就会发生旋转
               Criterion::Size当当前日志文件超过指定限制时发生轮转
               Criterion::AgeOrSize当达到两个限制中的任何一个时，就会发生旋转
            */
            // If the program runs long enough,
            Criterion::AgeOrSize(Age::Day, 10), // - create a new file every day
            // Naming::Timestamps, // - let the rotated files have a timestamp in their name
            Naming::Numbers, // - let the rotated files have a timestamp in their name
            /*
            您Cleanup::KeepLogFiles指定应保留的日志文件的数量；如果有更多，较旧的将被删除
            您Cleanup::KeepCompressedFiles指定应保留的日志文件的数量，并且这些文件将被额外压缩
            您可以Cleanup::KeepLogAndCompressedFiles 指定应按原样保留的日志文件数量以及正在压缩的附加数量
            如果Cleanup::Never不进行清理，所有文件都会保留。
             */
            Cleanup::KeepLogFiles(7), // - keep at most 7 log files
        )
        .start()
        .unwrap();

    log::debug!("{}", format!("{}", "s"));
    log::info!("{}", format!("{}", "s"));
    log::warn!("{}", format!("{}", "s"));
    log::error!("{}", format!("{}", "s"));

    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;
    log::debug!("{}", format!("{}", "s1"));
    log::info!("{}", format!("{}", "s1"));
    log::warn!("{}", format!("{}", "1s"));
    log::error!("{}", format!("{}", "1s"));
    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;
    log::debug!("{}", format!("{}", "s1"));
    log::info!("{}", format!("{}", "s1"));
    log::warn!("{}", format!("{}", "1s"));
    log::error!("{}", format!("{}", "1s"));
}
