#![allow(unused)]

//! 定义clap命令入口

// clap命令行工具
use clap::{Parser, Subcommand, Args};

// 内部库
use crate::commands::{
    ActionCommand, BagCommand, ComponentCommand,
    DaemonCommand, DoctorCommand, InterfaceCommand,
    LaunchCommand, LifecycleCommand, MulticastCommand, 
    NodeCommand, ParamCommand, PkgCommand, RunCommand,
    SecurityCommand, ServiceCommand, TopicCommand, 
    PlaygroundCommand,
};
use crate::commands::{
    action_cmd, bag_cmd, component_cmd, 
    daemon_cmd, doctor_cmd, interface_cmd,
    launch_cmd, lifecycle_cmd, multicast_cmd,
    node_cmd, param_cmd, pkg_cmd, run_cmd, 
    security_cmd, service_cmd, topic_cmd,
    playground_cmd,
};

use super::parser::command_parser;

/// rosette 命令行工具集
#[derive(Parser, Debug)]
#[command(name = "rosette")]
#[command(author, version, about, long_about = None)]
// 子命令显示版本号
#[command(propagate_version = true)]  
pub struct RosetteCli {
    #[command(subcommand)]
    pub command: RosetteCommand,
}

impl RosetteCli{
    pub async fn _parse(cmd: String){
        command_parser(cmd).await;
    }
}

/// 所有可用子命令
#[derive(Subcommand, Debug)]
pub enum RosetteCommand {
    /// 1. 动作子命令
    Action{
        #[command(subcommand)]
        command: ActionCommand,
    },
    
    /// 2. 数据包子命令
    Bag{
        #[command(subcommand)]
        command: BagCommand,
    },
    
    /// 3. 组件子命令
    Component{
        #[command(subcommand)]
        command: ComponentCommand,
    },
    
    /// 4. 守护进程子命令
    Daemon{
        #[command(subcommand)]
        command: DaemonCommand,
    },
    
    /// 5. 自检自修复子命令
    Doctor{
        #[command(subcommand)]
        command: DoctorCommand,
    },
    
    /// 6. 接口子命令
    Interface{
        #[command(subcommand)]
        command: InterfaceCommand,
    },
    
    /// 7. 启动子命令
    Launch{
        #[command(subcommand)]
        command: LaunchCommand,
    },
    
    /// 8. 生命周期子命令
    Lifecycle{
        #[command(subcommand)]
        command: LifecycleCommand,
    },
    
    /// 9. 多播子命令
    Multicast{
        #[command(subcommand)]
        command: MulticastCommand,
    },
    
    /// 10. 节点子命令
    Node{
        #[command(subcommand)]
        command: NodeCommand,
    },
    
    /// 11. 参数子命令
    Param{
        #[command(subcommand)]
        command: ParamCommand,
    },
    
    /// 12. 包管理子命令
    Pkg{
        #[command(subcommand)]
        command: PkgCommand,
    },
    
    /// 13. 运行子命令
    Run{
        #[command(subcommand)]
        command: RunCommand,
    },
    
    /// 14. 安全子命令
    Security{
        #[command(subcommand)]
        command: SecurityCommand,
    },
    
    /// 15. 服务子命令
    Service{
        #[command(subcommand)]
        command: ServiceCommand,
    },
    
    /// 16. 话题子命令
    Topic{
        #[command(subcommand)]
        command: TopicCommand,
    },
    
    /// 17. 可视化调试子命令
    Playground{
        #[command(subcommand)]
        command: PlaygroundCommand,
    }
    
}
