#![allow(unused)]

//! 动作命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 动作子命令定义
#[derive(Subcommand, Debug)]
pub enum ActionCommand{
    
}

/// 动作子命令解析器
pub async fn action_cmd(cmd: ActionCommand)->anyhow::Result<(), anyhow::Error>{
    
    println!("动作子命令");
    // 返回
    anyhow::Ok(())
}
