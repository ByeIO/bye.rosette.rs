#![allow(unused)]

//! 启动脚本命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 启动脚本子命令定义
#[derive(Subcommand, Debug)]
pub enum LaunchCommand{
    
}

/// 守护进程子命令解析器
pub async fn launch_cmd(cmd: LaunchCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
