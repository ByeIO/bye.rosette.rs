#![allow(unused)]

//! 测试发布std_msg消息

use ros2_client::{Context, MessageTypeName, Name, NodeName, NodeOptions};
use ros2_interfaces_humble::std_msgs;

fn main(){
    test_publisher();
}

fn test_publisher() {
    let context = Context::new().unwrap();
    let mut node = context
        .new_node(
            NodeName::new("/rustdds", "rustdds_listener").unwrap(),
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap();

    // 话题
    let topic = node
        .create_topic(
            &Name::new("/","topic").unwrap(),
            MessageTypeName::new("std_msgs", "String"),
            &ros2_client::DEFAULT_PUBLISHER_QOS,
        )
        .unwrap();

    // 发布者
    let publisher = node
        .create_publisher::<std_msgs::msg::String>(&topic, None)
        .unwrap();

    // 消息
    let message = std_msgs::msg::String {
        data: "Hello, world!".to_string(),
    };

    publisher.publish(message).unwrap();
    
    println!("发布消息成功");
}
