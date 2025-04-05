#![allow(unused)]

//! 组件命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 组件命令解析器
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ComponentArgs {
    #[command(subcommand)]
    pub command: ComponentCommand,
}

/// 组件子命令定义
#[derive(Subcommand, Debug)]
pub enum ComponentCommand {
    /// 列出正在运行的容器和组件
    List {
        #[arg(long, value_name = "SPIN_TIME")]
        spin_time: Option<f64>,
        #[arg(short, long)]
        use_sim_time: bool,
        #[arg(long)]
        no_daemon: bool,
        #[arg(long)]
        containers_only: bool,
        #[arg(value_name = "CONTAINER_NODE_NAME")]
        container_node_name: Option<String>,
    },
    /// 将组件加载到容器节点中
    Load {
        #[arg(long, value_name = "SPIN_TIME")]
        spin_time: Option<f64>,
        #[arg(short, long)]
        use_sim_time: bool,
        #[arg(long)]
        no_daemon: bool,
        #[arg(short, long, value_name = "NODE_NAME")]
        node_name: Option<String>,
        #[arg(long, value_name = "NODE_NAMESPACE")]
        node_namespace: Option<String>,
        #[arg(long, value_name = "LOG_LEVEL")]
        log_level: Option<String>,
        #[arg(short, long, value_name = "REMAP_RULES", value_delimiter = ' ')]
        remap_rules: Vec<String>,
        #[arg(short, long, value_name = "PARAMETERS", value_delimiter = ' ')]
        parameters: Vec<String>,
        #[arg(short, long, value_name = "EXTRA_ARGUMENTS", value_delimiter = ' ')]
        extra_arguments: Vec<String>,
        #[arg(short, long)]
        quiet: bool,
        #[arg(value_name = "CONTAINER_NODE_NAME")]
        container_node_name: String,
        #[arg(value_name = "PACKAGE_NAME")]
        package_name: String,
        #[arg(value_name = "PLUGIN_NAME")]
        plugin_name: String,
    },
    /// 在独立的容器节点中运行组件
    Standalone {
        #[arg(long, value_name = "SPIN_TIME")]
        spin_time: Option<f64>,
        #[arg(short, long)]
        use_sim_time: bool,
        #[arg(long)]
        no_daemon: bool,
        #[arg(short, long, value_name = "NODE_NAME")]
        node_name: Option<String>,
        #[arg(long, value_name = "NODE_NAMESPACE")]
        node_namespace: Option<String>,
        #[arg(long, value_name = "LOG_LEVEL")]
        log_level: Option<String>,
        #[arg(short, long, value_name = "REMAP_RULES", value_delimiter = ' ')]
        remap_rules: Vec<String>,
        #[arg(short, long, value_name = "PARAMETERS", value_delimiter = ' ')]
        parameters: Vec<String>,
        #[arg(short, long, value_name = "EXTRA_ARGUMENTS", value_delimiter = ' ')]
        extra_arguments: Vec<String>,
        #[arg(short, long, value_name = "CONTAINER_NODE_NAME")]
        container_node_name: Option<String>,
        #[arg(value_name = "PACKAGE_NAME")]
        package_name: String,
        #[arg(value_name = "PLUGIN_NAME")]
        plugin_name: String,
    },
    /// 列出在 ament 索引中注册的组件
    Types {
        #[arg(value_name = "PACKAGE_NAME")]
        package_name: Option<String>,
    },
    /// 从容器节点中卸载组件
    Unload {
        #[arg(long, value_name = "SPIN_TIME")]
        spin_time: Option<f64>,
        #[arg(short, long)]
        use_sim_time: bool,
        #[arg(long)]
        no_daemon: bool,
        #[arg(short, long)]
        quiet: bool,
        #[arg(value_name = "CONTAINER_NODE_NAME")]
        container_node_name: String,
        #[arg(value_name = "COMPONENT_UID", value_delimiter = ' ')]
        component_uids: Vec<String>,
    },
}

/// 组件子命令解析器
pub async fn component_cmd(cmd: ComponentCommand) -> anyhow::Result<()> {
    match cmd {
        ComponentCommand::List {
            spin_time,
            use_sim_time,
            no_daemon,
            containers_only,
            container_node_name,
        } => {
            // 在这里实现 List 命令的逻辑
            println!("Executing List command...");
            if let Some(node_name) = container_node_name {
                println!("Container Node Name: {}", node_name);
            }
            if containers_only {
                println!("Only listing containers.");
            }
            // 其他参数可以根据需要处理
        }
        ComponentCommand::Load {
            spin_time,
            use_sim_time,
            no_daemon,
            node_name,
            node_namespace,
            log_level,
            remap_rules,
            parameters,
            extra_arguments,
            quiet,
            container_node_name,
            package_name,
            plugin_name,
        } => {
            // 在这里实现 Load 命令的逻辑
            println!("Executing Load command...");
            println!("Container Node Name: {}", container_node_name);
            println!("Package Name: {}", package_name);
            println!("Plugin Name: {}", plugin_name);
            // 其他参数可以根据需要处理
        }
        ComponentCommand::Standalone {
            spin_time,
            use_sim_time,
            no_daemon,
            node_name,
            node_namespace,
            log_level,
            remap_rules,
            parameters,
            extra_arguments,
            container_node_name,
            package_name,
            plugin_name,
        } => {
            // 在这里实现 Standalone 命令的逻辑
            println!("Executing Standalone command...");
            println!("Package Name: {}", package_name);
            println!("Plugin Name: {}", plugin_name);
            if let Some(container_name) = container_node_name {
                println!("Container Node Name: {}", container_name);
            }
            // 其他参数可以根据需要处理
        }
        ComponentCommand::Types { package_name } => {
            // 在这里实现 Types 命令的逻辑
            println!("Executing Types command...");
            if let Some(pkg_name) = package_name {
                println!("Package Name: {}", pkg_name);
            }
        }
        ComponentCommand::Unload {
            spin_time,
            use_sim_time,
            no_daemon,
            quiet,
            container_node_name,
            component_uids,
        } => {
            // 在这里实现 Unload 命令的逻辑
            println!("Executing Unload command...");
            println!("Container Node Name: {}", container_node_name);
            println!("Component UIDs: {:?}", component_uids);
        }
    }

    // 返回成功结果
    Ok(())
}
