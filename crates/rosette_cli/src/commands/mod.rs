#![allow(unused)]

//! 命令定义(基本上与ros2一致)

// 1. 动作相关子命令
pub mod action;
pub use action::ActionCommand;
pub use action::action_cmd;

// 2. rosbag相关子命令
pub mod bag;
pub use bag::BagCommand;
pub use bag::bag_cmd;

// 3. 组件相关子命令
pub mod component;
pub use component::ComponentCommand;
pub use component::component_cmd;

// 4. 守护进程相关子命令
pub mod daemon;
pub use daemon::DaemonCommand;
pub use daemon::daemon_cmd;

// 5. 检查 ROS 环境和潜在问题
pub mod doctor;
pub use doctor::DoctorCommand;
pub use doctor::doctor_cmd;

// 6. ROS 接口信息显示
pub mod interface;
pub use interface::InterfaceCommand;
pub use interface::interface_cmd;

// 7. 启动文件运行
pub mod launch;
pub use launch::LaunchCommand;
pub use launch::launch_cmd;

// 8. 生命周期相关子命令
pub mod lifecycle;
pub use lifecycle::LifecycleCommand;
pub use lifecycle::lifecycle_cmd;

// 9. 多播相关子命令
pub mod multicast;
pub use multicast::MulticastCommand;
pub use multicast::multicast_cmd;

// 10. 节点相关子命令
pub mod node;
pub use node::NodeCommand;
pub use node::node_cmd;

// 11. 参数相关子命令
pub mod param;
pub use param::ParamCommand;
pub use param::param_cmd;

// 12. 包相关子命令
pub mod pkg;
pub use pkg::PkgCommand;
pub use pkg::pkg_cmd;

// 13. 运行包特定的可执行文件
pub mod run;
pub use run::RunCommand;
pub use run::run_cmd;

// 14. 安全相关子命令
pub mod security;
pub use security::SecurityCommand;
pub use security::security_cmd;

// 15. 服务相关子命令
pub mod service;
pub use service::ServiceCommand;
pub use service::service_cmd;

// 16. 主题相关子命令
pub mod topic;
pub use topic::TopicCommand;
pub use topic::topic_cmd;
