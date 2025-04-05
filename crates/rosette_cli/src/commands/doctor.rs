#![allow(unused)]

//! 自检自修复命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 自检自修复子命令定义
#[derive(Subcommand, Debug)]
pub enum DoctorCommand{
    
}

/// 守护进程子命令解析器
pub async fn doctor_cmd(cmd: DoctorCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
