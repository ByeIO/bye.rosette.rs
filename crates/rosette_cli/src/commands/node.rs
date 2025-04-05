#![allow(unused)]

//! 节点命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 节点子命令定义
#[derive(Subcommand, Debug)]
pub enum NodeCommand {
    /// 输出节点信息
    Info(InfoArgs),
    /// 输出节点列表
    List(ListArgs),
}

/// 节点信息子命令参数
#[derive(Args, Debug)]
pub struct InfoArgs {
    /// 节点名称
    pub node_name: String,
    /// 等待发现的时间（秒）
    #[arg(long)]
    pub spin_time: Option<f64>,
    /// 使用仿真时间
    #[arg(long)]
    pub use_sim_time: bool,
    /// 不使用守护进程
    #[arg(long)]
    pub no_daemon: bool,
    /// 包含隐藏的 topic、service 和 action
    #[arg(long)]
    pub include_hidden: bool,
}

/// 节点列表子命令参数
#[derive(Args, Debug)]
pub struct ListArgs {
    /// 等待发现的时间（秒）
    #[arg(long)]
    pub spin_time: Option<f64>,
    /// 使用仿真时间
    #[arg(long)]
    pub use_sim_time: bool,
    /// 不使用守护进程
    #[arg(long)]
    pub no_daemon: bool,
    /// 显示所有节点，包括隐藏节点
    #[arg(long)]
    pub all: bool,
    /// 只显示发现的节点数量
    #[arg(long)]
    pub count_nodes: bool,
}

/// 节点子命令解析器
pub async fn node_cmd(cmd: NodeCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        NodeCommand::Info(args) => {
            // 处理节点信息子命令
            println!("处理节点信息子命令...");
            println!("节点名称: {}", args.node_name);
            println!("等待发现时间: {:?}", args.spin_time);
            println!("使用仿真时间: {}", args.use_sim_time);
            println!("不使用守护进程: {}", args.no_daemon);
            println!("包含隐藏内容: {}", args.include_hidden);
        }
        NodeCommand::List(args) => {
            // 处理节点列表子命令
            println!("处理节点列表子命令...");
            println!("等待发现时间: {:?}", args.spin_time);
            println!("使用仿真时间: {}", args.use_sim_time);
            println!("不使用守护进程: {}", args.no_daemon);
            println!("显示所有节点: {}", args.all);
            println!("只显示节点数量: {}", args.count_nodes);
        }
    }

    // 返回
    Ok(())
}
