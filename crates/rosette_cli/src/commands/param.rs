#![allow(unused)]

//! 参数命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 参数子命令定义
#[derive(Subcommand, Debug)]
pub enum ParamCommand{
    
}

/// 守护进程子命令解析器
pub async fn param_cmd(cmd: ParamCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
