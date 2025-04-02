#![allow(unused)]

//! 读取fast_livo2示例rosbag

// 错误处理
use anyhow::Result;

// rosbag读取
use rosbag2_rs::{Reader, TopicConnection};

// 标准库
use std::path::Path;
use std::env;
use std::{cell::RefCell, rc::Rc};

fn main() -> Result<()> {
    // 指定ROS2 bag文件路径（根据实际路径调整）
    let bag_relative_path = "../../assets/Retail_Street.bag";
    let current_dir = env::current_dir().expect("获取当前目录失败");
    let bag_abs_path = current_dir.join(bag_relative_path);
    println!("拼接后的绝对路径: {}", bag_abs_path.display());

    // 初始化ROS bag读取器
    let mut reader = Reader::new(bag_abs_path)?;

    // 使用Rc和RefCell实现跨闭包的计数器共享
    let message_count = Rc::new(RefCell::new(0));
    let counter = message_count.clone();

    // 获取目标连接的ID列表
    let target_connections: Vec<i32> = reader.connections
        .iter()
        .filter(|c| c.topic == "/livox/lidar")
        .map(|c| c.id)
        .collect();

    // 处理消息（带提前终止机制）
    let process_result = reader.handle_messages(
        // 消息处理闭包
        move |(conn_id, timestamp, data)| {
            // 仅处理目标连接
            if !target_connections.contains(&conn_id.try_into().unwrap()) {
                return Ok(());
            }

            let mut cnt = counter.borrow_mut();
            
            // 只处理前5条消息
            if *cnt < 5 {
                *cnt += 1;
                
                // 打印消息头信息
                println!("[第 {} 条消息]", cnt);
                println!("连接ID: {}\n时间戳: {}", conn_id, timestamp);
                
                // 打印二进制数据的前20字节（实际开发需按消息格式反序列化）
                println!("原始数据预览（前20字节）: {:X?}\n", &data[..20.min(data.len())]);

                // 达到5条后返回特殊错误终止处理
                if *cnt == 5 {
                    return Err(anyhow::anyhow!("达到5条消息后提前终止"));
                }
            }
            Ok(())
        },
        // 起始时间（None表示从最早记录开始）
        None,
        // 结束时间（None表示到最晚记录结束）
        None,
    );

    // 处理提前终止的特殊错误
    match process_result {
        Err(e) if e.to_string() == "达到5条消息后提前终止" => Ok(()),
        other => other,
    }
}
