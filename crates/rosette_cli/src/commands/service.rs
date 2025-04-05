#![allow(unused)]

//! 服务命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 服务子命令定义
#[derive(Subcommand, Debug)]
pub enum ServiceCommand {
    /// 调用服务
    Call {
        /// 服务名称，例如 '/add_two_ints'
        service_name: String,
        /// 服务类型，例如 'std_srvs/srv/Empty'
        service_type: String,
        /// 服务请求的值，以 YAML 格式提供，例如 '{a: 1, b: 2}'
        #[clap(last = true)]
        values: Option<String>,
        /// 从标准输入读取值
        #[clap(long)]
        stdin: bool,
        /// 重复调用服务的频率（单位：Hz）
        #[clap(long, short = 'r')]
        rate: Option<u32>,
    },
    /// 监听服务
    Echo {
        /// 服务名称，例如 '/add_two_ints'
        service_name: String,
        /// 服务类型，例如 'example_interfaces/srv/AddTwoInts'
        #[clap(long)]
        service_type: Option<String>,
        /// 以 CSV 格式输出所有递归字段（例如用于绘图）
        #[clap(long)]
        csv: bool,
        /// 输出所有数组、字节和字符串的全部元素
        #[clap(long, short = 'f')]
        full_length: bool,
        /// 截断数组、字节和字符串的长度
        #[clap(long, short = 'l')]
        truncate_length: Option<usize>,
        /// 不打印数组字段
        #[clap(long)]
        no_arr: bool,
        /// 不打印字符串字段
        #[clap(long)]
        no_str: bool,
        /// 以块样式打印集合
        #[clap(long)]
        flow_style: bool,
    },
    /// 查找服务
    Find {
        /// 服务类型，例如 'rcl_interfaces/srv/ListParameters'
        service_type: String,
        /// 仅显示发现的服务数量
        #[clap(long, short = 'c')]
        count_services: bool,
        /// 包含隐藏服务
        #[clap(long)]
        include_hidden_services: bool,
    },
    /// 查看服务信息
    Info {
        /// 服务名称，例如 '/add_two_ints'
        service_name: String,
        /// 旋转时间（秒），用于等待发现（仅在不使用已运行的守护进程时适用）
        #[clap(long)]
        spin_time: Option<u32>,
        /// 使用 ROS 模拟时间
        #[clap(long, short = 's')]
        use_sim_time: bool,
        /// 不启动也不使用已运行的守护进程
        #[clap(long)]
        no_daemon: bool,
    },
    /// 列出服务
    List {
        /// 旋转时间（秒），用于等待发现（仅在不使用已运行的守护进程时适用）
        #[clap(long)]
        spin_time: Option<u32>,
        /// 使用 ROS 模拟时间
        #[clap(long, short = 's')]
        use_sim_time: bool,
        /// 不启动也不使用已运行的守护进程
        #[clap(long)]
        no_daemon: bool,
        /// 显示服务类型
        #[clap(long, short = 't')]
        show_types: bool,
        /// 仅显示发现的服务数量
        #[clap(long, short = 'c')]
        count_services: bool,
        /// 包含隐藏服务
        #[clap(long)]
        include_hidden_services: bool,
    },
    /// 获取服务类型
    Type {
        /// 服务名称，例如 '/talker/list_parameters'
        service_name: String,
    },
}

/// 服务子命令解析器
pub async fn service_cmd(cmd: ServiceCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        // 处理调用服务的逻辑
        ServiceCommand::Call {
            service_name,
            service_type,
            values,
            stdin,
            rate,
        } => {
            println!("Calling service: {}", service_name);
            println!("Service type: {}", service_type);
            if let Some(values) = values {
                println!("Values: {}", values);
            }
            if stdin {
                println!("Reading values from stdin");
            }
            if let Some(rate) = rate {
                println!("Rate: {} Hz", rate);
            }
        }
        // 处理监听服务的逻辑
        ServiceCommand::Echo {
            service_name,
            service_type,
            csv,
            full_length,
            truncate_length,
            no_arr,
            no_str,
            flow_style,
        } => {
            println!("Echoing service: {}", service_name);
            if let Some(service_type) = service_type {
                println!("Service type: {}", service_type);
            }
            if csv {
                println!("Output format: CSV");
            }
            if full_length {
                println!("Full length output enabled");
            }
            if let Some(truncate_length) = truncate_length {
                println!("Truncate length: {}", truncate_length);
            }
            if no_arr {
                println!("Array fields will not be printed");
            }
            if no_str {
                println!("String fields will not be printed");
            }
            if flow_style {
                println!("Flow style output enabled");
            }
        }
        // 处理查找服务的逻辑
        ServiceCommand::Find {
            service_type,
            count_services,
            include_hidden_services,
        } => {
            println!("Finding services of type: {}", service_type);
            if count_services {
                println!("Only displaying service count");
            }
            if include_hidden_services {
                println!("Including hidden services");
            }
        }
        // 处理查看服务信息的逻辑
        ServiceCommand::Info {
            service_name,
            spin_time,
            use_sim_time,
            no_daemon,
        } => {
            println!("Getting info for service: {}", service_name);
            if let Some(spin_time) = spin_time {
                println!("Spin time: {} seconds", spin_time);
            }
            if use_sim_time {
                println!("Using simulation time");
            }
            if no_daemon {
                println!("Not using daemon");
            }
        }
        // 处理列出服务的逻辑
        ServiceCommand::List {
            spin_time,
            use_sim_time,
            no_daemon,
            show_types,
            count_services,
            include_hidden_services,
        } => {
            println!("Listing services");
            if let Some(spin_time) = spin_time {
                println!("Spin time: {} seconds", spin_time);
            }
            if use_sim_time {
                println!("Using simulation time");
            }
            if no_daemon {
                println!("Not using daemon");
            }
            if show_types {
                println!("Showing service types");
            }
            if count_services {
                println!("Only displaying service count");
            }
            if include_hidden_services {
                println!("Including hidden services");
            }
        }
        // 处理获取服务类型的逻辑
        ServiceCommand::Type { service_name } => {
            println!("Getting type for service: {}", service_name);
        }
    }

    // 返回
    anyhow::Ok(())
}
