#![allow(unused)]

//! 接口命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 接口子命令定义
#[derive(Subcommand, Debug)]
pub enum InterfaceCommand {
    /// 列出所有可用的接口类型
    List(ListArgs),
    /// 输出指定包中的接口类型
    Package(PackageArgs),
    /// 输出提供接口的包列表
    Packages(PackagesArgs),
    /// 输出接口原型
    Proto(ProtoArgs),
    /// 输出接口定义
    Show(ShowArgs),
}

/// 接口子命令解析器
pub async fn interface_cmd(cmd: InterfaceCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        InterfaceCommand::List(args) => {
            // 处理 List 命令
            handle_list_command(args).await?;
        }
        InterfaceCommand::Package(args) => {
            // 处理 Package 命令
            handle_package_command(args).await?;
        }
        InterfaceCommand::Packages(args) => {
            // 处理 Packages 命令
            handle_packages_command(args).await?;
        }
        InterfaceCommand::Proto(args) => {
            // 处理 Proto 命令
            handle_proto_command(args).await?;
        }
        InterfaceCommand::Show(args) => {
            // 处理 Show 命令
            handle_show_command(args).await?;
        }
    }

    // 返回
    Ok(())
}

// List 命令的参数
#[derive(Args, Debug)]
pub struct ListArgs {
    /// 仅打印消息类型
    #[clap(short = 'm', long = "only-msgs")]
    only_msgs: bool,
    /// 仅打印服务类型
    #[clap(short = 's', long = "only-srvs")]
    only_srvs: bool,
    /// 仅打印动作类型
    #[clap(short = 'a', long = "only-actions")]
    only_actions: bool,
}

// Package 命令的参数
#[derive(Args, Debug)]
pub struct PackageArgs {
    /// ROS 包名
    #[clap()]
    package_name: String,
    /// 仅打印消息类型
    #[clap(short = 'm', long = "only-msgs")]
    only_msgs: bool,
    /// 仅打印服务类型
    #[clap(short = 's', long = "only-srvs")]
    only_srvs: bool,
    /// 仅打印动作类型
    #[clap(short = 'a', long = "only-actions")]
    only_actions: bool,
}

// Packages 命令的参数
#[derive(Args, Debug)]
pub struct PackagesArgs {
    /// 仅列出生成消息的包
    #[clap(short = 'm', long = "only-msgs")]
    only_msgs: bool,
    /// 仅列出生成服务的包
    #[clap(short = 's', long = "only-srvs")]
    only_srvs: bool,
    /// 仅列出生成动作的包
    #[clap(short = 'a', long = "only-actions")]
    only_actions: bool,
}

// Proto 命令的参数
#[derive(Args, Debug)]
pub struct ProtoArgs {
    /// 接口类型
    #[clap()]
    type_: String,
    /// 是否不输出外层引号
    #[clap(long = "no-quotes")]
    no_quotes: bool,
}

// Show 命令的参数
#[derive(Args, Debug)]
pub struct ShowArgs {
    /// 接口类型
    #[clap()]
    type_: String,
    /// 显示所有注释，包括嵌套接口定义的注释
    #[clap(long = "all-comments")]
    all_comments: bool,
    /// 不显示注释或空白
    #[clap(long = "no-comments")]
    no_comments: bool,
}

// 处理 List 命令
async fn handle_list_command(args: ListArgs) -> anyhow::Result<()> {
    // 根据参数执行逻辑
    println!("Handling List command with args: {:?}", args);
    Ok(())
}

// 处理 Package 命令
async fn handle_package_command(args: PackageArgs) -> anyhow::Result<()> {
    // 根据参数执行逻辑
    println!("Handling Package command with args: {:?}", args);
    Ok(())
}

// 处理 Packages 命令
async fn handle_packages_command(args: PackagesArgs) -> anyhow::Result<()> {
    // 根据参数执行逻辑
    println!("Handling Packages command with args: {:?}", args);
    Ok(())
}

// 处理 Proto 命令
async fn handle_proto_command(args: ProtoArgs) -> anyhow::Result<()> {
    // 根据参数执行逻辑
    println!("Handling Proto command with args: {:?}", args);
    Ok(())
}

// 处理 Show 命令
async fn handle_show_command(args: ShowArgs) -> anyhow::Result<()> {
    // 根据参数执行逻辑
    println!("Handling Show command with args: {:?}", args);
    Ok(())
}
