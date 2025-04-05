#![allow(unused)]

//! 多播命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 多播子命令定义
#[derive(Subcommand, Debug)]
pub enum MulticastCommand{
    
}

/// 守护进程子命令解析器
pub async fn multicast_cmd(cmd: MulticastCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
