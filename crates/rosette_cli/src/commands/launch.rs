#![allow(unused)]

//! 启动脚本命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 启动脚本子命令定义
#[derive(Subcommand, Debug)]
pub enum LaunchCommand {
    /// 启动一个launch文件
    Launch {
        /// 包含launch文件的ROS包名
        package_name: String,
        /// launch文件名
        launch_file_name: Option<String>,
        /// 传递给launch文件的参数，格式为 '<name>:=<value>'
        #[clap(last = true)]
        launch_arguments: Vec<String>,
        /// 以非交互模式运行launch系统，不关联终端
        #[clap(long)]
        noninteractive: bool,
        /// 启用调试模式，提供更详细的输出
        #[clap(long)]
        debug: bool,
        /// 打印launch描述到控制台，而不实际启动它
        #[clap(long)]
        print: bool,
        /// 显示可以传递给launch文件的参数
        #[clap(long)]
        show_args: bool,
        /// 显示所有启动的子进程的输出，通过覆盖它们的输出配置
        #[clap(long)]
        show_all_subprocesses_output: bool,
        /// 在所有可执行文件之前添加前缀命令，如果命令包含空格，需要用引号包裹
        #[clap(long)]
        launch_prefix: Option<String>,
        /// 使用正则表达式模式过滤哪些可执行文件应用--launch-prefix，通过匹配可执行文件名
        #[clap(long)]
        launch_prefix_filter: Option<String>,
    },
}

/// 启动脚本子命令解析器
pub async fn launch_cmd(cmd: LaunchCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        LaunchCommand::Launch {
            package_name,
            launch_file_name,
            launch_arguments,
            noninteractive,
            debug,
            print,
            show_args,
            show_all_subprocesses_output,
            launch_prefix,
            launch_prefix_filter,
        } => {
            // 根据命令行参数执行相应的操作
            // 这里只是一个示例，实际实现需要根据具体需求进行
            println!("Package name: {}", package_name);
            if let Some(file_name) = launch_file_name {
                println!("Launch file name: {}", file_name);
            }
            if !launch_arguments.is_empty() {
                println!("Launch arguments: {:?}", launch_arguments);
            }
            if noninteractive {
                println!("Running in non-interactive mode");
            }
            if debug {
                println!("Debug mode enabled");
            }
            if print {
                println!("Printing launch description");
            }
            if show_args {
                println!("Showing available launch arguments");
            }
            if show_all_subprocesses_output {
                println!("Showing all subprocesses output");
            }
            if let Some(prefix) = launch_prefix {
                println!("Launch prefix: {}", prefix);
            }
            if let Some(filter) = launch_prefix_filter {
                println!("Launch prefix filter: {}", filter);
            }
        }
    }

    // 返回
    anyhow::Ok(())
}
