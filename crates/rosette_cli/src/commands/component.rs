#![allow(unused)]

//! 组件命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 组件子命令定义
#[derive(Subcommand, Debug)]
pub enum ComponentCommand{
    
}

/// 组件子命令解析器
pub async fn component_cmd(cmd: ComponentCommand)->anyhow::Result<(), anyhow::Error>{
    
    // 返回
    anyhow::Ok(())
}
