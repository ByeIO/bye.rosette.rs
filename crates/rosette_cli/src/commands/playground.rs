#![allow(unused)]

//! 可视化调试命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 可视化调试子命令定义
#[derive(Subcommand, Debug)]
pub enum PlaygroundCommand {
    /// 浏览器打开webviz(类似rviz)查看消息数据
    Webviz,
    /// 浏览器打开webrqt(类似rqt)查看消息
    Webrqt,
    /// 浏览器打开仿真平台(类似gazebo)
    Mujoco,
}

/// 可视化子命令解析器
pub async fn playground_cmd(cmd: PlaygroundCommand) -> anyhow::Result<(), anyhow::Error> {

    // 返回
    anyhow::Ok(())
}
