#![allow(unused)]

//! 安全命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 安全子命令定义
#[derive(Subcommand, Debug)]
pub enum SecurityCommand{
    
}

/// 守护进程子命令解析器
pub async fn security_cmd(cmd: SecurityCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
