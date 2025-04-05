#![allow(unused)]

//! 守护进程命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 守护进程子命令定义
#[derive(Subcommand, Debug)]
pub enum DaemonCommand{
    
}

/// 守护进程子命令解析器
pub async fn daemon_cmd(cmd: DaemonCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
