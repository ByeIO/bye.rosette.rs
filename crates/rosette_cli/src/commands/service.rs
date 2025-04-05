#![allow(unused)]

//! 服务命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 服务子命令定义
#[derive(Subcommand, Debug)]
pub enum ServiceCommand{
    
}

/// 守护进程子命令解析器
pub async fn service_cmd(cmd: ServiceCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
