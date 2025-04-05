#![allow(unused)]

//! 生命周期命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 生命周期子命令定义
#[derive(Subcommand, Debug)]
pub enum LifecycleCommand{
    
}

/// 守护进程子命令解析器
pub async fn lifecycle_cmd(cmd: LifecycleCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
