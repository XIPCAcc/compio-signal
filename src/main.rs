#![warn(clippy::all)]

use clap::{Parser, ValueEnum};
use nix::sys::signal::{kill, Signal as NixSignal};
use nix::unistd::Pid;
use std::error::Error;
use std::time::{Duration, Instant};
use compio::signal::unix::signal;
use libc;



/// 设置服务器端信号：由于使用了compio::signal，不需要手动设置信号处理或屏蔽
fn setup_server_signals() -> Result<(), Box<dyn Error>> {
    // 匹配C代码中的usleep(1000)
    std::thread::sleep(std::time::Duration::from_micros(1000));
    Ok(())
}

/// 设置客户端信号：由于使用了compio::signal，不需要手动设置信号处理或屏蔽
fn setup_client_signals() -> Result<(), Box<dyn Error>> {
    // 匹配C代码中的usleep(1000)
    std::thread::sleep(std::time::Duration::from_micros(1000));
    Ok(())
}

/// 基准测试结构体，用于存储性能统计数据
#[derive(Debug)]
struct Benchmarks {
    total_start: Instant,
    single_start: Instant,
    minimum: Duration,
    maximum: Duration,
    sum: Duration,
    squared_sum: f64,
    count: usize,
}

impl Benchmarks {
    /// 创建新的基准测试结构体
    fn new() -> Self {
        Self {
            total_start: Instant::now(),
            single_start: Instant::now(),
            minimum: Duration::from_secs(u64::MAX),
            maximum: Duration::from_nanos(0),
            sum: Duration::from_nanos(0),
            squared_sum: 0.0,
            count: 0,
        }
    }
    
    /// 更新基准测试数据
    fn update(&mut self, duration: Duration) {
        self.minimum = self.minimum.min(duration);
        self.maximum = self.maximum.max(duration);
        self.sum += duration;
        self.squared_sum += duration.as_nanos() as f64 * duration.as_nanos() as f64;
        self.count += 1;
    }
    
    /// 评估基准测试结果
    fn evaluate(&self, args: &Args) {
        let total_time = self.total_start.elapsed();
        let average = self.sum / (self.count as u32);
        
        let sigma = self.squared_sum / self.count as f64;
        let sigma = (sigma - (average.as_nanos() as f64).powi(2)).sqrt();
        
        let message_rate = (self.count as f64) / total_time.as_secs_f64();
        let message_rate_mb = (self.count as f64 * args.size as f64) / 1024.0 / 1024.0 / total_time.as_secs_f64();
        
        println!("\n============ RESULTS ================");
        println!("Message size:       {}", args.size);
        println!("Message count:      {}", args.count);
        println!("Total duration:     {:.3} ms", total_time.as_millis() as f64);
        println!("Average duration:   {:.3} us", average.as_micros() as f64);
        println!("Minimum duration:   {:.3} us", self.minimum.as_micros() as f64);
        println!("Maximum duration:   {:.3} us", self.maximum.as_micros() as f64);
        println!("Standard deviation: {:.3} us", sigma / 1000.0);
        println!("Message rate:       {:.0} msg/s", message_rate);
        println!("Message rate:       {:.3} MB/s", message_rate_mb);
        println!("=====================================");
    }
}

/// 发送信号到目标 PID
fn send_signal(pid: u32, signal: NixSignal) -> Result<(), Box<dyn Error>> {
    let nix_pid = Pid::from_raw(pid as i32);
    kill(nix_pid, signal)?;
    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    /// ping-pong 次数
    #[arg(long, short, default_value_t = 1_000_000)]
    count: usize,

    #[arg(long, short, default_value_t = 1)]
    size: usize,

    /// 运行模式：server / client / test
    #[arg(long, short, value_enum, default_value_t = Mode::Server)]
    mode: Mode,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
enum Mode {
    Server,
    Client,
}

async fn run_server(args: &Args) -> Result<(), Box<dyn Error>> {
    setup_server_signals()?;

    // 等待初始信号
    eprintln!("[SERVER] Waiting for initial signal from client...");
    signal(libc::SIGUSR1).await?;
    eprintln!("[SERVER] Received initial signal from client!");

    // 设置基准测试
    let mut bench = Benchmarks::new();

    for message in 0..args.count {
        bench.single_start = Instant::now();

        // 发送信号到客户端
        let _ = send_signal(0, NixSignal::SIGUSR2);
        // eprintln!("[SERVER] Sent SIGUSR2 to client!");
        // 等待客户端的响应信号
        // eprintln!("[SERVER] Waiting for signal from client...");
        signal(libc::SIGUSR1).await?;
        // eprintln!("[SERVER] Received signal from client!");
        
        // 更新基准测试数据
        let total_duration = bench.single_start.elapsed();
        bench.update(total_duration);
        
        // eprintln!("[SERVER] Progress: {} / {}", message + 1, args.count);
    }

    bench.evaluate(args);
    Ok(())
}

async fn run_client(args: &Args) -> Result<(), Box<dyn Error>> {
    setup_client_signals()?;
    
    // 发送初始信号
    eprintln!("[CLIENT] Sending initial SIGUSR1 to server...");
    let _send_result = send_signal(0, NixSignal::SIGUSR1);
    eprintln!("[CLIENT] Sent initial SIGUSR1 to server!");
    
    let mut remaining = args.count;
    
    while remaining > 0 {
        // 等待来自服务器的信号
        // eprintln!("[CLIENT] Waiting for signal from server...");
        signal(libc::SIGUSR2).await?;
        // eprintln!("[CLIENT] Received signal from server!");

        // 向进程组发送信号（使用PID 0）
        let _send_result = send_signal(0, NixSignal::SIGUSR1);
        // eprintln!("[CLIENT] Sent SIGUSR1 to server!");

        remaining -= 1;
        
        // eprintln!("[CLIENT] Progress: {} remaining", remaining);
    }
    
    Ok(())
}

/// 异步主函数
async fn main_async() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    match args.mode {
        Mode::Server => run_server(&args).await,
        Mode::Client => run_client(&args).await,
    }
}

/// 主函数
fn main() -> Result<(), Box<dyn Error>> {
    compio::runtime::Runtime::new()?.block_on(main_async())
}
