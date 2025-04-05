#![allow(unused)]

//! 数据包命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 数据包子命令定义
#[derive(Subcommand, Debug)]
pub enum BagCommand{
    
}

/// 动作子命令解析器
pub async fn bag_cmd(cmd: BagCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
