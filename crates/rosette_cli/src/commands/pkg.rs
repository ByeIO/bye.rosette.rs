#![allow(unused)]

//! 包管理命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 包管理子命令定义
#[derive(Subcommand, Debug)]
pub enum PkgCommand{
    
}

/// 守护进程子命令解析器
pub async fn pkg_cmd(cmd: PkgCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
