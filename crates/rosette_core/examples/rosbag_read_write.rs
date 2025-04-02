#![allow(unused)]

//! 读取/写入rosbag数据
//! 创建rosbag(ros2_humble)

// 错误处理
use anyhow::Result;

// rosbag处理
use rosbag2_rs::{ Writer, Reader };

// 临时文件
use tempfile::tempdir;

// 标准库
use std::path::Path;
use std::{cell::RefCell, rc::Rc};

fn main() -> Result<()> {
    // 指定新ROS2 bag的路径
    let bag_path = Path::new("./result/rosbag_read_write.bag");

    // 初始化Writer
    let mut writer = Writer::new(bag_path);
    writer.open()?;

    // 定义一个标准的ROS2消息类型（根据实际的ROS2消息类型进行调整）
    let topic = "example_topic";
    let msgtype = "std_msgs/msg/Int32";

    // 为此消息类型添加连接
    // 根据实际的ROS2消息类型调整qos配置文件 (https://docs.ros2.org/latest/api/rmw/structrmw__qos__profile__t.html)
    const LATCH: &str = r#"- history: 3
  depth: 0
  reliability: 1
  durability: 1
  deadline:
    sec: 2147483647
    nsec: 4294967295
  lifespan:
    sec: 2147483647
    nsec: 4294967295
  liveliness: 1
  liveliness_lease_duration:
    sec: 2147483647
    nsec: 4294967295
  avoid_ros_namespace_conventions: false
"#;

    let connection = writer.add_connection(topic, msgtype, "cdr", LATCH)?;

    // 写入一些虚拟消息
    for i in 0..50 {
        // 虚拟数据 0x2a2b = (int32)10795
        let dummy_data = [0, 1, 0, 1, 43, 42, 0, 0]; 
        writer.write(&connection, 1_000_000_000 * i, &dummy_data)?;
    }

    writer.close()?;

    println!("ROS2 bag 创建于 {:?}", bag_path);
    
    write_and_read_bag()?;
    
    Ok(())
}

fn write_and_read_bag() -> Result<()> {
    // 使用模拟数据初始化并验证属性

    let dir = tempdir().unwrap();

    let mut writer = Writer::new(dir.path());
    writer.open()?;

    // 添加一个虚拟连接
    let connection = writer.add_connection("topic1", "msgtype1", "cdr", "")?;

    // 写入虚拟消息
    for i in 0..10 {
        writer.write(&connection, i as i64, &[(i * 2 + 1) as u8])?;
    }

    // 添加另一个虚拟连接
    let connection = writer.add_connection("topic2", "msgtype2", "cdr", "")?;

    // 写入虚拟消息
    for i in 0..10 {
        writer.write(&connection, i as i64, &[(i * 2) as u8])?;
    }

    writer.close()?;

    let mut reader = Reader::new(dir.path())?;

    assert_eq!(reader.connections.len(), 2);
    assert_eq!(reader.connections[0].topic, "topic1");
    assert_eq!(reader.connections[0].msgtype, "msgtype1");
    assert_eq!(reader.connections[0].msgcount, 10);

    assert_eq!(reader.duration(), 10);

    let msg_data: Vec<(i64, i64, Vec<u8>)> = vec![];
    let msg_data = Rc::new(RefCell::new(msg_data)); // 将向量包装在 Rc 和 RefCell 中

    reader.handle_messages(
        |(id, timestamp, data)| {
            // 使用 `borrow_mut` 获取向量的可变引用
            println!("处理消息: {:?} {:?} {:?}", id, timestamp, data);

            msg_data.borrow_mut().push((id, timestamp, data));
            Ok(())
        },
        None,
        None,
    )?;

    assert_eq!(msg_data.borrow().len(), 20);

    Ok(())
}
