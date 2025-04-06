#![allow(unused)]

//! 命令解析器

// 命令行解析
use clap::Parser;

// 复杂命令解析
use shell_words::split;

// 错误处理
use anyhow;

// 消除重复代码
use paste::paste;

// 内部库
use super::cli::{ RosetteCli, RosetteCommand };
use crate::commands::{
    action_cmd, bag_cmd, component_cmd, daemon_cmd, doctor_cmd, interface_cmd, launch_cmd, lifecycle_cmd, multicast_cmd, node_cmd, param_cmd, pkg_cmd, playground_cmd, run_cmd, security_cmd, service_cmd, topic_cmd
};

// 按照空格分割输入
fn split_input_string(input: String) -> anyhow::Result<Vec<String>>{
    // 使用 shell-words 来分割输入字符串
    match shell_words::split(&input) {
        Ok(args) => {
            // 返回值
            anyhow::Ok(args)
        }
        Err(e) => {
            // 如果分割失败，打印错误信息
            eprintln!("Failed to parse input: {}", e);
            
            // 将 shell_words::Error 转换为 anyhow::Error 并返回
            Err(anyhow::Error::new(e))
        }
    }
}

macro_rules! generate_commands {
    ($($command:ident),+) => {
        paste! {
            $(
                RosetteCommand::[<$command>] { command } => {
                    [<$command:snake _cmd>](command).await?;
                    anyhow::Ok(())
                },
            )+
        }
    };
}

// 输出为命令
pub async fn command_parser(input: String) -> anyhow::Result<()> {
    let str_split = split_input_string(input)?;
    
    // println!("str_split: {:?}", str_split);
    
    let cli = RosetteCli::parse_from(str_split);

    // println!("cli.command: {:?}", cli.command);
    
    match cli.command {
        // 1. 动作子命令
        RosetteCommand::Action { command } => {
            // 子程序继续处理
            action_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 2. 数据包子命令
        RosetteCommand::Bag { command } => {
            // 子程序继续处理
            bag_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 3. 组件子命令
        RosetteCommand::Component { command } => {
            // 子程序继续处理
            component_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 4. 守护进程子命令
        RosetteCommand::Daemon { command } => {
            // 子程序继续处理
            daemon_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 5. 自检子命令
        RosetteCommand::Doctor { command } => {
            // 子程序继续处理
            doctor_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 6. 接口子命令
        RosetteCommand::Interface { command } => {
            // 子程序继续处理
            interface_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 7. 启动子命令
        RosetteCommand::Launch { command } => {
            // 子程序继续处理
            launch_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 8. 生命周期子命令
        RosetteCommand::Lifecycle { command } => {
            // 子程序继续处理
            lifecycle_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 9. 多播子命令
        RosetteCommand::Multicast { command } => {
            // 子程序继续处理
            multicast_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 10. 节点子命令
        RosetteCommand::Node { command } => {
            // 子程序继续处理
            node_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 11. 参数子命令
        RosetteCommand::Param { command } => {
            // 子程序继续处理
            param_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 12. 包管理子命令
        RosetteCommand::Pkg { command } => {
            // 子程序继续处理
            pkg_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 13. 运行子命令
        RosetteCommand::Run { command } => {
            // 子程序继续处理
            run_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 14. 安全子命令
        RosetteCommand::Security { command } => {
            // 子程序继续处理
            security_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 15. 服务子命令
        RosetteCommand::Service { command } => {
            // 子程序继续处理
            service_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 16. 话题子命令
        RosetteCommand::Topic { command } => {
            // 子程序继续处理
            topic_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 17. 可视化调试子命令
        RosetteCommand::Playground { command } => {
            // 子程序继续处理
            playground_cmd(command).await?;
            
            // 返回
            anyhow::Ok(())
        },
        // 其他情况
        _ => {
            println!("命令错误🚧");
            anyhow::Ok(())
        }
    }// end match

}
