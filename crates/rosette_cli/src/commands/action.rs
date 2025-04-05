#![allow(unused)]
#![allow(non_camel_case_types)]

//! 动作命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 动作子命令定义
#[derive(Subcommand, Debug)]
pub enum ActionCommand{
    /// 1. 打印动作的信息
    Info{
        #[command(subcommand)]
        command: ActionInfoCommand,
    },
    /// 2. 打印动作列表
    List{
        #[command(subcommand)]
        command: ActionListCommand,
    },
    /// 3. 发送动作目标
    Send_Goal{
        #[command(subcommand)]
        command: ActionSendgoalCommand,
    },
    /// 4. 打印动作类型
    Type{
        #[command(subcommand)]
        command: ActionTypeCommand,
    },
}

/// 动作子命令解析器
pub async fn action_cmd(cmd: ActionCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        ActionCommand::Info { command } => handle_action_info(command).await?,
        ActionCommand::List { command } => handle_action_list(command).await?,
        ActionCommand::Send_Goal { command } => handle_action_send_goal(command).await?,
        ActionCommand::Type { command } => handle_action_type(command).await?,
    }
    // 返回值
    anyhow::Ok(())
}

#[derive(Subcommand, Debug)]
pub enum ActionInfoCommand {
    /// 打印动作信息
    Info {
        /// 动作名称
        action_name: String,
        /// 是否显示动作类型
        #[arg(short = 't', long = "show-types")]
        show_types: bool,
        /// 是否仅显示动作客户端和服务器的数量
        #[arg(short = 'c', long = "count")]
        count: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ActionListCommand {
    /// 打印动作列表
    List {
        /// 是否显示动作类型
        #[arg(short = 't', long = "show-types")]
        show_types: bool,
        /// 是否仅显示动作数量
        #[arg(short = 'c', long = "count-actions")]
        count: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ActionSendgoalCommand {
    /// 发送动作目标
    Send_Goal {
        /// 动作名称
        action_name: String,
        /// 动作类型
        action_type: String,
        /// 目标请求值（YAML格式）
        goal: String,
        /// 是否从标准输入读取目标
        #[arg(short = 's', long = "stdin")]
        stdin: bool,
        /// 是否回显反馈消息
        #[arg(short = 'f', long = "feedback")]
        feedback: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ActionTypeCommand {
    /// 打印动作类型
    Type {
        /// 动作名称
        action_name: String,
    },
}

async fn handle_action_info(command: ActionInfoCommand) -> anyhow::Result<()> {
    match command {
        ActionInfoCommand::Info {
            action_name,
            show_types,
            count,
        } => {
            println!("动作名称: {}", action_name);
            if show_types {
                println!("显示动作类型");
                // 这里可以添加代码来获取和打印动作类型
            }
            if count {
                println!("仅显示动作客户端和服务器的数量");
                // 这里可以添加代码来获取和打印动作客户端和服务器的数量
            }
        }
    }
    anyhow::Ok(())
}

async fn handle_action_list(command: ActionListCommand) -> anyhow::Result<()> {
    match command {
        ActionListCommand::List { show_types, count } => {
            println!("动作列表:");
            // 这里可以添加代码来获取动作列表
            if show_types {
                println!("显示动作类型");
                // 这里可以添加代码来获取和打印动作类型
            }
            if count {
                println!("仅显示动作数量");
                // 这里可以添加代码来获取和打印动作数量
            }
        }
    }
    anyhow::Ok(())
}

async fn handle_action_send_goal(command: ActionSendgoalCommand) -> anyhow::Result<()> {
    match command {
        ActionSendgoalCommand::Send_Goal {
            action_name,
            action_type,
            goal,
            stdin,
            feedback,
        } => {
            println!("动作名称: {}", action_name);
            println!("动作类型: {}", action_type);
            println!("目标请求值: {}", goal);
            if stdin {
                println!("从标准输入读取目标");
                // 这里可以添加代码来从标准输入读取目标
            }
            if feedback {
                println!("回显反馈消息");
                // 这里可以添加代码来回显反馈消息
            }
            // 这里可以添加代码来发送动作目标
        }
    }
    anyhow::Ok(())
}

async fn handle_action_type(command: ActionTypeCommand) -> anyhow::Result<()> {
    match command {
        ActionTypeCommand::Type { action_name } => {
            println!("动作名称: {}", action_name);
            // 这里可以添加代码来获取和打印动作类型
        }
    }
    anyhow::Ok(())
}
