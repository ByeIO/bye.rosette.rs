#![allow(unused)]

//! 话题命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 话题子命令定义
#[derive(Subcommand, Debug)]
pub enum TopicCommand{
    
}

/// 守护进程子命令解析器
pub async fn topic_cmd(cmd: TopicCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
