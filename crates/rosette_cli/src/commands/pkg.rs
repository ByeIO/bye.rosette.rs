#![allow(unused)]

//! 包管理命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 包管理子命令定义
#[derive(Subcommand, Debug)]
pub enum PkgCommand {
    /// 创建一个新的 ROS 2 包
    Create {
        /// 包名
        #[clap(value_name = "PACKAGE_NAME")]
        package_name: String,

        /// 包的格式，默认为 3
        #[clap(long, value_name = "FORMAT", default_value = "3")]
        package_format: String,

        /// 包的描述
        #[clap(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// 包的许可证
        #[clap(long, value_name = "LICENSE")]
        license: Option<String>,

        /// 创建包的目标目录，默认为当前目录
        #[clap(long, value_name = "DESTINATION_DIRECTORY")]
        destination_directory: Option<String>,

        /// 包的构建类型，默认为 ament_cmake
        #[clap(long, value_name = "BUILD_TYPE", default_value = "ament_cmake")]
        build_type: String,

        /// 包的依赖项
        #[clap(long, value_name = "DEPENDENCIES")]
        dependencies: Option<Vec<String>>,

        /// 包维护者的邮箱
        #[clap(long, value_name = "EMAIL")]
        maintainer_email: Option<String>,

        /// 包维护者的姓名
        #[clap(long, value_name = "NAME")]
        maintainer_name: Option<String>,

        /// 空可执行文件的名称
        #[clap(long, value_name = "NODE_NAME")]
        node_name: Option<String>,

        /// 空库的名称
        #[clap(long, value_name = "LIBRARY_NAME")]
        library_name: Option<String>,
    },

    /// 输出指定包的可执行文件列表
    Executables {
        /// 包名
        #[clap(value_name = "PACKAGE_NAME")]
        package_name: String,

        /// 是否显示可执行文件的完整路径，默认为 false
        #[clap(long)]
        full_path: bool,
    },

    /// 输出可用包的列表
    List,

    /// 输出指定包的前缀路径
    Prefix {
        /// 包名
        #[clap(value_name = "PACKAGE_NAME")]
        package_name: String,

        /// 是否显示包的 share 目录，默认为 false
        #[clap(long)]
        share: bool,
    },

    /// 输出指定包的 XML 文件内容或特定标签内容
    Xml {
        /// 包名
        #[clap(value_name = "PACKAGE_NAME")]
        package_name: String,

        /// 要输出的 XML 标签名称
        #[clap(long, value_name = "TAG")]
        tag: Option<String>,
    },
}

/// 包管理子命令解析器
pub async fn pkg_cmd(cmd: PkgCommand) -> anyhow::Result<(), anyhow::Error> {
    match cmd {
        PkgCommand::Create {
            package_name,
            package_format,
            description,
            license,
            destination_directory,
            build_type,
            dependencies,
            maintainer_email,
            maintainer_name,
            node_name,
            library_name,
        } => {
            // 创建包的逻辑
            println!("Creating package: {}", package_name);
            // 这里可以添加具体的包创建逻辑
        }
        PkgCommand::Executables {
            package_name,
            full_path,
        } => {
            // 输出可执行文件列表的逻辑
            println!("Executables for package: {}", package_name);
            // 这里可以添加具体的可执行文件列表输出逻辑
        }
        PkgCommand::List => {
            // 输出可用包列表的逻辑
            println!("Available packages:");
            // 这里可以添加具体的包列表输出逻辑
        }
        PkgCommand::Prefix {
            package_name,
            share,
        } => {
            // 输出包前缀路径的逻辑
            println!("Prefix for package: {}", package_name);
            // 这里可以添加具体的包前缀路径输出逻辑
        }
        PkgCommand::Xml {
            package_name,
            tag,
        } => {
            // 输出包 XML 文件内容或特定标签内容的逻辑
            println!("XML for package: {}", package_name);
            // 这里可以添加具体的 XML 输出逻辑
        }
    }

    // 返回
    anyhow::Ok(())
}
