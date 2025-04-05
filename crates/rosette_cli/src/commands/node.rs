#![allow(unused)]

//! 节点命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 节点子命令定义
#[derive(Subcommand, Debug)]
pub enum NodeCommand{
    
}

/// 守护进程子命令解析器
pub async fn node_cmd(cmd: NodeCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
