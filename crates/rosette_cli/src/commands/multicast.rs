#![allow(unused)]

//! 多播命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 多播子命令定义
#[derive(Subcommand, Debug)]
pub enum MulticastCommand {
    /// 接收一个 UDP 多播数据包
    Receive(ReceiveArgs),
    /// 发送一个 UDP 多播数据包
    Send(SendArgs),
}

/// 接收子命令的参数
#[derive(Args, Debug)]
pub struct ReceiveArgs {
    /// 多播组地址
    #[clap(long)]
    pub group: Option<String>,
    /// 多播端口号
    #[clap(long)]
    pub port: Option<u16>,
}

/// 发送子命令的参数
#[derive(Args, Debug)]
pub struct SendArgs {
    /// 多播组地址
    #[clap(long)]
    pub group: Option<String>,
    /// 多播端口号
    #[clap(long)]
    pub port: Option<u16>,
    /// 多播 TTL
    #[clap(long)]
    pub ttl: Option<u8>,
}

/// 多播子命令解析器
pub async fn multicast_cmd(cmd: MulticastCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        // 处理接收命令
        MulticastCommand::Receive(args) => {
            println!("接收命令参数:");
            println!("多播组地址: {:?}", args.group);
            println!("多播端口号: {:?}", args.port);
            // 在这里实现接收逻辑
        }
        // 处理发送命令
        MulticastCommand::Send(args) => {
            println!("发送命令参数:");
            println!("多播组地址: {:?}", args.group);
            println!("多播端口号: {:?}", args.port);
            println!("多播 TTL: {:?}", args.ttl);
            // 在这里实现发送逻辑
        }
    }

    // 返回
    anyhow::Ok(())
}
