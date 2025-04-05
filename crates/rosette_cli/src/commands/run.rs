#![allow(unused)]

//! 运行命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 运行子命令定义
#[derive(Subcommand, Debug)]
pub enum RunCommand {
    /// 运行一个包中的可执行文件
    Run {
        /// 包名
        #[clap(value_name = "PACKAGE_NAME")]
        package_name: String,

        /// 可执行文件名
        #[clap(value_name = "EXECUTABLE_NAME")]
        executable_name: String,

        /// 传递给可执行文件的任意参数
        #[clap(value_name = "ARGV")]
        argv: Vec<String>,

        /// 前缀命令，该命令将位于可执行文件之前
        #[clap(long, value_name = "PREFIX")]
        prefix: Option<String>,
    },
}

/// 运行子命令解析器
pub async fn run_cmd(cmd: RunCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        RunCommand::Run {
            package_name,
            executable_name,
            argv,
            prefix,
        } => {
            // 构建完整的命令
            let mut command = if let Some(prefix) = prefix {
                // 如果有前缀命令，则将其拆分为单独的参数
                let mut prefix_args = shell_words::split(&prefix)
                    .map_err(|e| anyhow::anyhow!("Failed to parse prefix command: {}", e))?;
                let mut cmd = std::process::Command::new(&prefix_args.remove(0));
                cmd.args(prefix_args);
                cmd
            } else {
                std::process::Command::new(format!(
                    "ros2 run {} {}",
                    package_name, executable_name
                ))
            };

            // 添加传递给可执行文件的参数
            command.args(argv);

            // 打印要运行的命令
            println!("Running command: {:?}", command);

            // 运行命令
            let status = command.status()?;
            if !status.success() {
                anyhow::bail!("Command failed with status: {:?}", status);
            }
        }
    }

    // 返回
    anyhow::Ok(())
}
