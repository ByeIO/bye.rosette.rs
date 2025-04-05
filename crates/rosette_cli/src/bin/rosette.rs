#![allow(unused)]

//! rosette命令行

// 命令行
use clap::Parser;

// 内部库
use rosette_cli::RosetteCli;

#[tokio::main]
async fn main() {
    // 获取命令行参数
    let args = std::env::args().collect::<Vec<String>>();
    // 检查是否有命令行参数
    if args.len() < 2 {
        eprintln!("请提供命令行参数, 或-h, -v");
        return;
    }
    // 解析命令行参数
    let cmd = args[1..].join(" ");
    println!("cmd: {}", cmd);
    
    // parse函数为宏自动生成
    RosetteCli::parse();
}
