// 必需的标准库引入
use std::alloc::{GlobalAlloc, Layout, System};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Sub;
use std::sync::atomic::{AtomicUsize, Ordering};
use memmap2::Mmap;

// 内存追踪分配器结构体
struct TrackingAllocator;

// 全局原子计数器，记录已分配内存
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

// 为 TrackingAllocator 实现 GlobalAlloc trait
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

// 设置为全局分配器
#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

// 打印当前内存使用量
pub fn print_memory_usage() {
    println!("Allocated memory: {} bytes", ALLOCATED.load(Ordering::SeqCst));
}

// 生成1gb 文件
// dd if=/dev/urandom of=testfile.bin bs=1M count=1024 status=progress

pub fn test_read(){
    println!("Reading from file");
    let t = chrono::Local::now();

    //标准库
   // let _ = normal("./testfile.bin");
   //  print_memory_usage();
   //  println!("耗时 {}",chrono::Local::now().sub(t).to_string());

    // Reading from file
    // Allocated memory: 1073832794 bytes
    // Allocated memory: 90970 bytes
    // 耗时 PT0.96218S

    // 使用 异步库 tokio
    // let _ = tokio_io("./testfile.bin");
    //  print_memory_usage();
    //  println!("耗时 {}",chrono::Local::now().sub(t).to_string());
    // Reading from file
    // Allocated memory: 86842 bytes
    // 耗时 PT0.000329S

    //mmap
    let _ = io_mmap("./testfile.bin");
    print_memory_usage();
    println!("耗时 {}",chrono::Local::now().sub(t).to_string());
    //Allocated memory: 90970 bytes
    // Allocated memory: 90970 bytes
    // 耗时 PT2.608394S

}


/****************************************************************/


//标准库
/// 该函数用于读取指定文件的内容，并将其写入到另一个文件中，同时打印内存使用情况。
fn normal(file_path: &str)->anyhow::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?; // 读取文件内容到缓冲区

    let mut wf = File::create("testfile222.bin").expect("failed to create the file");
    wf.write_all(&buffer)?;
    print_memory_usage();

    Ok(())
}

// 使用 异步库 tokio
async fn tokio_io(file_path: &str) -> anyhow::Result<()> {
    // 异步打开文件
    let mut file = tokio::fs::File::open(file_path).await?;

    let mut wf = tokio::fs::File::create("testfile222.bin").await?;
    let _ = tokio::io::copy(&mut file, &mut wf).await?;
    print_memory_usage();

    Ok(())
}

// 文件件映射到内存，并将其内容写入到新文件中。
fn io_mmap(file_path: &str)->anyhow::Result<()> {
    let file = File::open(file_path).expect("failed to open the file");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };

    let mut wf = File::create("testfile222.bin").expect("failed to create the file");
    let _ = wf.write_all(&mmap[..]);
    print_memory_usage();

    Ok(())
}