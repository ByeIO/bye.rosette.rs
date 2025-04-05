#![allow(unused)]

//! 运行命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 运行子命令定义
#[derive(Subcommand, Debug)]
pub enum RunCommand{
    
}

/// 守护进程子命令解析器
pub async fn run_cmd(cmd: RunCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
